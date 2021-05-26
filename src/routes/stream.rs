use auth::Wrapper as Auth;
use errors::StreamingErrors;

use crate::core::DbConnection;
use crate::core::StateManager;
use crate::errors;
use crate::stream_tracking::ContentType;
use crate::stream_tracking::StreamTracking;
use crate::stream_tracking::VirtualManifest;
use crate::streaming::ffprobe::FFProbeCtx;
use crate::streaming::get_avc1_tag;
use crate::streaming::level_to_tag;
use crate::streaming::Avc1Level;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;

use database::mediafile::MediaFile;

use rocket::http;
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::response::Response;
use rocket::State;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use rocket_contrib::uuid::Uuid;

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

#[get("/<id>/manifest?<gid>")]
pub async fn return_virtual_manifest(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    auth: Auth,
    conn: State<'_, DbConnection>,
    id: i32,
    gid: Option<Uuid>,
) -> Result<JsonValue, errors::StreamingErrors> {
    if let Some(gid) = gid {
        return Ok(json!({
            "tracks": stream_tracking.get_for_gid(&gid).await,
            "gid": gid.to_hyphenated().to_string(),
        }));
    }

    let gid = uuid::Uuid::new_v4();

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

    let duration = chrono::DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(info.get_duration().unwrap() as i64, 0),
        Utc,
    );

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

    let video = state
        .create(
            StreamType::Video {
                map: video_stream.index as usize,
                profile,
            },
            media.target_file.clone().into(),
        )
        .await?;

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

    stream_tracking
        .insert(
            &gid,
            VirtualManifest {
                id: video.clone(),
                is_direct: true,
                mime: "video/mp4".into(),
                duration: info.get_duration(),
                content_type: ContentType::Video,
                chunk_path: format!("{}/data/$Number$.m4s", video.clone()),
                init_seg: Some(format!("{}/data/init.mp4", video.clone())),
                codecs: video_avc.to_string(),
                bandwidth: info.get_bitrate().parse::<u64>().unwrap(),
                args: {
                    let mut x = HashMap::new();
                    x.insert(
                        "height".to_string(),
                        video_stream.height.clone().unwrap().to_string(),
                    );
                    x
                },
            },
        )
        .await;

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

        stream_tracking
            .insert(
                &gid,
                VirtualManifest {
                    id: audio.clone(),
                    is_direct: false,
                    mime: "audio/mp4".into(),
                    duration: info.get_duration(),
                    codecs: "mp4a.40.2".into(),
                    bandwidth: 120_000,
                    content_type: ContentType::Audio,
                    chunk_path: format!("{}/data/$Number$.m4s", audio.clone()),
                    init_seg: Some(format!("{}/data/init.mp4", audio.clone())),
                    args: HashMap::new(),
                },
            )
            .await;
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

        stream_tracking
            .insert(
                &gid,
                VirtualManifest {
                    id: subtitle.clone(),
                    is_direct: false,
                    content_type: ContentType::Subtitle,
                    mime: "text/vtt".into(),
                    codecs: "vtt".into(), //ignored
                    bandwidth: 0,         // ignored
                    duration: None,
                    chunk_path: format!("{}/data/stream.vtt", subtitle.clone()),
                    init_seg: None,
                    args: {
                        let mut x = HashMap::new();
                        if let Some(y) = stream
                            .tags
                            .as_ref()
                            .and_then(|x| x.title.clone().or(x.language.clone()))
                        {
                            x.insert("title".to_string(), y);
                        }
                        x
                    },
                },
            )
            .await;
    }

    Ok(json!({
        "tracks": stream_tracking.get_for_gid(&gid).await,
        "gid": gid.to_hyphenated().to_string(),
    }))
}

#[get("/<gid>/manifest.mpd?<start_num>&<should_kill>&<includes>")]
pub async fn return_manifest(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    auth: Auth,
    conn: State<'_, DbConnection>,
    gid: Uuid,
    start_num: Option<u64>,
    should_kill: Option<bool>,
    includes: Option<String>,
) -> Result<Response<'static>, errors::StreamingErrors> {
    if should_kill.unwrap_or(true) {
        let ids = stream_tracking
            .get_for_gid(&gid)
            .await
            .into_iter()
            .filter(|x| !matches!(x.content_type, ContentType::Video | ContentType::Audio))
            .map(|x| x.id)
            .collect::<Vec<_>>();
        stream_tracking.kill(&state, &gid, ids, true).await;
    }

    let manifest = if let Some(includes) = includes {
        let includes = includes
            .split(",")
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        stream_tracking
            .compile_only(&gid, start_num.unwrap_or(0), includes)
            .await
            .unwrap()
    } else {
        stream_tracking
            .compile(&gid, start_num.unwrap_or(0))
            .await
            .unwrap()
    };

    Response::build()
        .header(http::ContentType::new("application", "dash+xml"))
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
    gid: Uuid,
    chunk_num: u32,
) -> Result<JsonValue, errors::StreamingErrors> {
    let ids = stream_tracking.get_for_gid(&gid).await;

    let mut should_client_hard_seek = false;

    for manifest in ids {
        should_client_hard_seek |= state.should_hard_seek(manifest.id, chunk_num).await?;
    }

    Ok(json!({
        "should_client_seek": should_client_hard_seek,
    }))
}

#[get("/<gid>/state/get_stderr")]
pub async fn session_get_stderr(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    gid: Uuid,
) -> Result<JsonValue, errors::StreamingErrors> {
    Ok(json!({
    "errors": stream::iter(stream_tracking
        .get_for_gid(&gid)
        .await)
        .filter_map(|x| async { state.get_stderr(x.id).await.ok() })
        .collect::<Vec<_>>().await,
    }))
}

#[get("/<gid>/state/kill")]
pub async fn kill_session(
    state: State<'_, StateManager>,
    stream_tracking: State<'_, StreamTracking>,
    gid: Uuid,
) -> Result<Status, errors::StreamingErrors> {
    for manifest in stream_tracking.get_for_gid(&gid).await {
        let _ = state.die(manifest.id).await;
    }

    Ok(Status::NoContent)
}
