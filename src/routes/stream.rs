use auth::Wrapper as Auth;
use errors::StreamingErrors;

use crate::core::DbConnection;
use crate::core::StateManager;
use crate::errors;
use crate::stream_tracking::StreamTracking;
use crate::streaming::ffprobe::FFProbeCtx;
use crate::streaming::get_avc1_tag;
use crate::streaming::level_to_tag;
use crate::streaming::Avc1Level;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;

use database::mediafile::MediaFile;

use rocket::http::ContentType;
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::response::Response;
use rocket::State;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use slog::info;
use slog::Logger;

use nightfall::error::NightfallError;
use nightfall::profile::StreamType;
use nightfall::profile::*;

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use futures::stream;
use futures::StreamExt;
use std::future::Future;
use tokio::task::spawn_blocking;

#[get("/<id>/manifest.mpd?<start_num>&<gid>")]
pub async fn return_manifest(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    auth: Auth,
    conn: State<'_, DbConnection>,
    id: i32,
    start_num: Option<u32>,
    gid: Option<u128>,
) -> Result<Response<'static>, errors::StreamingErrors> {
    let start_num = start_num.unwrap_or(0);
    let media = MediaFile::get_one(&conn, id)
        .await
        .map_err(|e| errors::StreamingErrors::NoMediaFileFound(e.to_string()))?;

    let user_id = auth.0.claims.id;

    let target_file = media.target_file.clone();
    let info = spawn_blocking(move || {
        FFProbeCtx::new(crate::streaming::FFPROBE_BIN.as_ref())
            .get_meta(&std::path::PathBuf::from(target_file))
    })
    .await
    .unwrap()
    .map_err(|_| errors::StreamingErrors::FFProbeCtxFailed)?;

    let mut ms = info
        .get_ms()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?
        .to_string();

    ms.truncate(4);

    let gid = gid.unwrap_or(uuid::Uuid::new_v4().as_u128());
    stream_tracking.kill_all(&state, gid).await;

    let duration = chrono::DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(info.get_duration().unwrap() as i64, 0),
        Utc,
    );

    let duration_string = format!(
        "PT{}H{}M{}.{}S",
        duration.hour(),
        duration.minute(),
        duration.second(),
        ms
    );

    let mut tracks = Vec::new();

    let video_stream = info
        .find_by_codec("video")
        .first()
        .cloned()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?;

    let profile = if video_stream.codec_name == "hevc".to_string() {
        VideoProfile::Native
    } else {
        VideoProfile::Direct
    };

    // temporarily fix High@L4.1 streams from not playing.
    let profile = if video_stream.level == Some(41) {
        VideoProfile::Native
    } else {
        profile
    };

    let video = state
        .create(
            StreamType::Video {
                map: video_stream.index as usize,
                profile,
            },
            media.target_file.clone().into(),
        )
        .await?;

    stream_tracking.insert(gid, video.clone()).await;

    // FIXME: Stop hardcoding a fps of 24
    let video_avc = video_stream
        .level
        .and_then(|x| level_to_tag(x))
        .unwrap_or(get_avc1_tag(
            video_stream.width.clone().unwrap_or(1920) as u64,
            video_stream.height.clone().unwrap_or(1080) as u64,
            info.get_bitrate().parse().unwrap(),
            24,
        ));

    tracks.push(format!(
        include_str!("../static/video_segment.mpd"),
        id = video.clone(),
        height = video_stream.height.clone().unwrap_or(1080),
        bandwidth = info.get_bitrate().parse::<i32>().unwrap(),
        init = format!("{}/data/init.mp4?start_num={}", video.clone(), start_num),
        chunk_path = format!("{}/data/$Number$.m4s", video.clone()),
        start_num = start_num,
        avc = video_avc.to_string(),
    ));

    let audio_streams = info.find_by_codec("audio");

    for stream in audio_streams {
        let audio = state
            .create(
                StreamType::Audio {
                    map: stream.index as usize,
                    profile: AudioProfile::Low,
                },
                media.target_file.clone().into(),
            )
            .await?;

        tracks.push(format!(
            include_str!("../static/audio_segment.mpd"),
            id = audio.clone(),
            init = format!("{}/data/init.mp4?start_num={}", audio.clone(), start_num),
            chunk_path = format!("{}/data/$Number$.m4s", audio.clone()),
            start_num = start_num,
        ));

        stream_tracking.insert(gid, audio.clone()).await;
    }

    let subtitles = info.find_by_codec("subtitle");

    for stream in subtitles {
        let subtitle = state
            .create(
                StreamType::Subtitle {
                    map: stream.index as usize,
                    profile: SubtitleProfile::Webvtt,
                },
                media.target_file.clone().into(),
            )
            .await?;

        tracks.push(format!(
            include_str!("../static/subtitle_segment.mpd"),
            id = subtitle.clone(),
            title = stream
                .tags
                .clone()
                .and_then(|x| x.title)
                .unwrap_or(format!("Subtitle {}", stream.index)),
            path = format!("{}/data/stream.vtt", subtitle.clone())
        ));

        stream_tracking.insert(gid, subtitle.clone()).await;
    }

    let manifest = format!(
        include_str!("../static/manifest.mpd"),
        duration = duration_string,
        base_url = "/api/v1/stream/",
        segments = tracks.join("\n"),
    );

    Response::build()
        .header(ContentType::new("application", "dash+xml"))
        .streamed_body(Cursor::new(manifest))
        .ok()
}

/// Repeatedly invoke a nightfall routine until a timeout occurs waiting for a chunk to be "ready".
///
/// `tick_dur` will the the duration amount that gets passed into `std::thread::sleep` and it will
/// block for AT MOST `tick_limit` ticks. When a the tick limit has been hit `None` is returned
/// otherwise `Some(Result<T, NightfallError>)` is returned.
///
async fn timeout_segment<F, T>(
    f: impl Fn() -> F,
    tick_dur: Duration,
    tick_limit: usize,
) -> Result<T, NightfallError>
where
    F: Future<Output = Result<T, NightfallError>>,
{
    let mut ticks = 0usize;

    loop {
        if ticks >= tick_limit {
            return Err(NightfallError::ChunkNotDone.into());
        }

        let result = f().await;

        if let Err(NightfallError::ChunkNotDone) = result {
            ticks += 1;
            tokio::time::sleep(tick_dur).await;
        } else {
            break result;
        }
    }
}

#[get("/<id>/data/init.mp4?<start_num>", rank = 1)]
pub async fn get_init(
    state: State<'_, StateManager>,
    id: String,
    start_num: Option<u32>,
) -> Result<Option<NamedFile>, errors::StreamingErrors> {
    let path: String = timeout_segment(
        || state.chunk_init_request(id.clone(), start_num.unwrap_or(0)),
        Duration::from_millis(100),
        100,
    )
    .await?;

    Ok(NamedFile::open(path).await.ok())
}

#[get("/<id>/data/<chunk..>", rank = 3)]
pub async fn get_chunk(
    state: State<'_, StateManager>,
    id: String,
    chunk: PathBuf,
) -> Result<Option<NamedFile>, errors::StreamingErrors> {
    let extension = chunk
        .extension()
        .ok_or(errors::StreamingErrors::InvalidRequest)?
        .to_string_lossy()
        .into_owned();

    // Chunks will always be m4s or mp4
    if extension.as_str() != "m4s" {
        return Err(errors::StreamingErrors::InvalidRequest);
    }

    // Parse the chunk filename into a u64, we unwrap_or because sometimes it can be a init chunk,
    // if its a init chunk we assume a chunk index of 0 because we are fetching the first few
    // chunks.
    let chunk_num = chunk
        .file_stem()
        .ok_or(errors::StreamingErrors::InvalidRequest)?
        .to_string_lossy()
        .into_owned()
        .parse::<u32>()
        .unwrap_or(0);

    let path: String = timeout_segment(
        || state.chunk_request(id.clone(), chunk_num),
        Duration::from_millis(100),
        100,
    )
    .await?;

    Ok(NamedFile::open(path).await.ok())
}

#[get("/<id>/data/stream.vtt", rank = 2)]
pub async fn get_subtitle(
    state: State<'_, StateManager>,
    id: String,
) -> Result<Option<NamedFile>, errors::StreamingErrors> {
    let path: String = timeout_segment(
        || state.get_sub(id.clone(), "stream.vtt".into()),
        Duration::from_millis(100),
        200,
    )
    .await?;

    Ok(NamedFile::open(path).await.ok())
}

#[get("/<gid>/state/should_hard_seek/<chunk_num>")]
pub async fn should_client_hard_seek(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    gid: u128,
    chunk_num: u32,
) -> Result<JsonValue, errors::StreamingErrors> {
    let ids = stream_tracking
        .get_for_gid(gid)
        .await
        .ok_or(errors::StreamingErrors::InvalidRequest)?;

    let mut should_client_hard_seek = false;

    for id in ids {
        should_client_hard_seek |= state.should_hard_seek(id, chunk_num).await?;
    }

    Ok(json!({
        "should_client_seek": should_client_hard_seek,
    }))
}

#[get("/<gid>/state/get_stderr")]
pub async fn session_get_stderr(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    gid: u128,
) -> Result<JsonValue, errors::StreamingErrors> {
    Ok(json!({
    "errors": stream::iter(stream_tracking
        .get_for_gid(gid)
        .await
        .ok_or(errors::StreamingErrors::InvalidRequest)?)
        .filter_map(|x| async { state.get_stderr(x).await.ok() })
        .collect::<Vec<_>>().await,
    }))
}

#[get("/<gid>/state/kill")]
pub async fn kill_session(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    gid: u128,
) -> Result<Status, errors::StreamingErrors> {
    for id in stream_tracking
        .get_for_gid(gid)
        .await
        .ok_or(errors::StreamingErrors::InvalidRequest)?
    {
        let _ = state.die(id).await;
    }

    Ok(Status::NoContent)
}
