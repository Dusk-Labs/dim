use auth::Wrapper as Auth;
use errors::StreamingErrors;

use crate::core::DbConnection;
use crate::errors;
use crate::stream_tracking::StreamTracking;
use crate::streaming::ffprobe::FFProbeCtx;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;

use database::mediafile::MediaFile;

use rocket::http::ContentType;
use rocket::http::Header;
use rocket::http::Status;
use rocket::request::State;
use rocket::response::NamedFile;
use rocket::response::Response;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use slog::info;
use slog::Logger;

use nightfall::profile::StreamType;
use nightfall::StateManager;
use nightfall::{error::NightfallError, profile::Profile};

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

#[get("/<id>/manifest.mpd?<start_num>&<gid>")]
pub fn return_manifest(
    state: State<StateManager>,
    stream_tracking: State<StreamTracking>,
    auth: Auth,
    conn: DbConnection,
    id: i32,
    start_num: Option<u32>,
    gid: Option<u128>,
) -> Result<Response<'static>, errors::StreamingErrors> {
    let start_num = start_num.unwrap_or(0);
    let media = MediaFile::get_one(conn.as_ref(), id)
        .map_err(|e| errors::StreamingErrors::NoMediaFileFound(e.to_string()))?;

    let user_id = auth.0.claims.id;

    let info = FFProbeCtx::new(crate::streaming::FFPROBE_BIN.as_ref())
        .get_meta(&std::path::PathBuf::from(media.target_file.clone()))
        .map_err(|_| errors::StreamingErrors::FFProbeCtxFailed)?;

    let mut ms = info
        .get_ms()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?
        .to_string();

    ms.truncate(4);

    let gid = gid.unwrap_or(uuid::Uuid::new_v4().as_u128());
    stream_tracking.kill_all(&state, gid);

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
        Profile::Native
    } else {
        Profile::Direct
    };

    let video = state.create(
        media.target_file.clone().into(),
        profile,
        StreamType::Video(video_stream.index as usize),
    );

    stream_tracking.insert(gid, video.clone());

    tracks.push(format!(
        include_str!("../static/video_segment.mpd"),
        id = video.clone(),
        bandwidth = info.get_bitrate(),
        init = format!("{}/data/init.mp4?start_num={}", video.clone(), start_num),
        chunk_path = format!("{}/data/$Number$.m4s", video.clone()),
        start_num = start_num,
        avc = "avc1.64001f",
    ));

    let audio_streams = info.find_by_codec("audio");

    for stream in audio_streams {
        let audio = state.create(
            media.target_file.clone().into(),
            Profile::Audio,
            StreamType::Audio(stream.index as usize),
        );

        tracks.push(format!(
            include_str!("../static/audio_segment.mpd"),
            id = audio.clone(),
            init = format!("{}/data/init.mp4?start_num={}", audio.clone(), start_num),
            chunk_path = format!("{}/data/$Number$.m4s", audio.clone()),
            start_num = start_num,
        ));

        stream_tracking.insert(gid, audio.clone());
    }

    let manifest = format!(
        include_str!("../static/manifest.mpd"),
        duration = duration_string,
        base_url = "/api/v1/stream/",
        segments = tracks.join("\n"),
    );

    Response::build()
        .header(ContentType::new("application", "dash+xml"))
        .sized_body(Cursor::new(manifest))
        .ok()
}

/// Repeatedly invoke a nightfall routine until a timeout occurs waiting for a chunk to be "ready".
///
/// `tick_dur` will the the duration amount that gets passed into `std::thread::sleep` and it will
/// block for AT MOST `tick_limit` ticks. When a the tick limit has been hit `None` is returned
/// otherwise `Some(Result<T, NightfallError>)` is returned.
///
fn timeout_segment<F, T>(f: F, tick_dur: Duration, tick_limit: usize) -> Result<T, NightfallError>
where
    F: Fn() -> Result<T, NightfallError>,
{
    let mut ticks = 0usize;

    loop {
        if ticks >= tick_limit {
            return Err(NightfallError::ChunkNotDone.into());
        }

        let result = f();

        if let Err(NightfallError::ChunkNotDone) = result {
            ticks += 1;
            std::thread::sleep(tick_dur);
        } else {
            break result;
        }
    }
}

#[get("/<id>/data/init.mp4?<start_num>", rank = 1)]
pub fn get_init(
    state: State<StateManager>,
    id: String,
    start_num: Option<u64>,
) -> Result<Option<NamedFile>, errors::StreamingErrors> {
    let path: String = timeout_segment(
        || state.init_or_create(id.clone(), start_num.unwrap_or(0)),
        Duration::from_millis(100),
        20,
    )?;

    Ok(NamedFile::open(path).ok())
}

#[get("/<id>/data/<chunk..>", rank = 2)]
pub fn get_chunk(
    state: State<StateManager>,
    conn: DbConnection,
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
        return Ok(None);
    }

    // Parse the chunk filename into a u64, we unwrap_or because sometimes it can be a init chunk,
    // if its a init chunk we assume a chunk index of 0 because we are fetching the first few
    // chunks.
    let chunk_num = chunk
        .file_stem()
        .ok_or(errors::StreamingErrors::InvalidRequest)?
        .to_string_lossy()
        .into_owned()
        .parse::<u64>()
        .unwrap_or(0);

    state
        .exists(id.clone())
        .map_err(|_| errors::StreamingErrors::SessionDoesntExist)?;

    let path: String = timeout_segment(
        || state.get_segment(id.clone(), chunk_num),
        Duration::from_millis(100),
        20,
    )?;

    Ok(NamedFile::open(path).ok())
}

#[get("/<gid>/state/should_hard_seek/<chunk_num>")]
pub fn should_client_hard_seek(
    state: State<StateManager>,
    stream_tracking: State<StreamTracking>,
    gid: u128,
    chunk_num: u64,
) -> Result<JsonValue, errors::StreamingErrors> {
    let ids = stream_tracking
        .get_for_gid(gid)
        .ok_or(errors::StreamingErrors::InvalidRequest)?;

    let mut should_client_hard_seek = false;
    for id in ids {
        should_client_hard_seek |= state.should_client_hard_seek(id, chunk_num)?;
    }

    Ok(json!({
        "should_client_seek": should_client_hard_seek,
    }))
}

#[get("/<gid>/state/get_stderr")]
pub fn session_get_stderr(
    state: State<StateManager>,
    stream_tracking: State<StreamTracking>,
    gid: u128,
) -> Result<JsonValue, errors::StreamingErrors> {
    Ok(json!({
    "errors": stream_tracking
        .get_for_gid(gid)
        .ok_or(errors::StreamingErrors::InvalidRequest)?
        .into_iter()
        .filter_map(|x| state.get_stderr(x).ok())
        .collect::<Vec<_>>(),
    }))
}

#[get("/<gid>/state/kill")]
pub fn kill_session(
    state: State<StateManager>,
    stream_tracking: State<StreamTracking>,
    gid: u128,
) -> Result<Status, errors::StreamingErrors> {
    for id in stream_tracking
        .get_for_gid(gid)
        .ok_or(errors::StreamingErrors::InvalidRequest)?
    {
        let _ = state.kill(id);
    }

    Ok(Status::NoContent)
}
