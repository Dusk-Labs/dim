use crate::core::DbConnection;
use crate::errors;
use crate::streaming::ffprobe::FFProbeCtx;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;

use database::mediafile::MediaFile;

use rocket::http::ContentType;
use rocket::http::Header;
use rocket::request::State;
use rocket::response::NamedFile;
use rocket::response::Response;
use rocket_contrib::json::JsonValue;

use slog::info;
use slog::Logger;

use nightfall::profile::Profile;
use nightfall::profile::StreamType;
use nightfall::StateManager;

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

#[get("/<id>/manifest.mpd?<start_num>")]
pub fn return_manifest(
    state: State<StateManager>,
    conn: DbConnection,
    id: i32,
    start_num: Option<u32>,
) -> Result<Response<'static>, errors::DimError> {
    let start_num = start_num.unwrap_or(0);
    let media = MediaFile::get_one(conn.as_ref(), id)?;
    let info = FFProbeCtx::new(crate::streaming::FFPROBE_BIN.as_ref())
        .get_meta(&std::path::PathBuf::from(media.target_file.clone()))?;

    let mut ms = info.get_ms().unwrap().to_string();
    ms.truncate(4);

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

    let video = state.create(
        media.target_file.clone().into(),
        Profile::Direct,
        StreamType::Video,
    );
    let audio = state.create(media.target_file.into(), Profile::Audio, StreamType::Audio);

    let video_part = format!(
        include_str!("../static/video_segment.mpd"),
        bandwidth = info.get_bitrate(),
        init = format!("data/{}/init.mp4?start_num={}", video.clone(), start_num),
        chunk_path = format!("data/{}/$Number$.m4s", video.clone()),
        start_num = start_num,
    );

    let audio_part = format!(
        include_str!("../static/audio_segment.mpd"),
        init = format!("data/{}/init.mp4?start_num={}", audio.clone(), start_num),
        chunk_path = format!("data/{}/$Number$.m4s", audio.clone()),
        start_num = start_num,
    );

    let manifest = format!(
        include_str!("../static/manifest.mpd"),
        duration = duration_string,
        segments = format!("{}\n{}", video_part, audio_part),
    );

    Response::build()
        .header(ContentType::new("application", "dash+xml"))
        .header(Header::new("X-STREAM-ID", video))
        .sized_body(Cursor::new(manifest))
        .ok()
}

#[get("/<id>/data/init.mp4?<start_num>", rank = 1)]
pub fn get_init(
    state: State<StateManager>,
    id: String,
    start_num: Option<u64>,
) -> Result<Option<NamedFile>, errors::StreamingErrors> {
    let path = state.init_or_create(id, start_num.unwrap_or(0))?;

    Ok(NamedFile::open(path).ok())
}

#[get("/<id>/data/<chunk..>", rank = 2)]
pub fn get_chunk(
    state: State<StateManager>,
    conn: DbConnection,
    id: String,
    chunk: PathBuf,
) -> Result<Option<NamedFile>, errors::DimError> {
    let extension = chunk.extension()?.to_string_lossy().into_owned();

    // Chunks will always be m4s or mp4
    if extension.as_str() != "m4s" {
        return Ok(None);
    }

    // Parse the chunk filename into a u64, we unwrap_or because sometimes it can be a init chunk,
    // if its a init chunk we assume a chunk index of 0 because we are fetching the first few
    // chunks.
    let chunk_num = chunk
        .file_stem()?
        .to_string_lossy()
        .into_owned()
        .parse::<u64>()
        .unwrap_or(0);

    state
        .exists(id.clone())
        .map_err(|_| errors::StreamingErrors::OtherNightfall)?;

    let path = state
        .get_segment(id.clone(), chunk_num)
        .map_err(|_| errors::StreamingErrors::OtherNightfall)?;

    for _ in 0..5 {
        if let Ok(_) = NamedFile::open(path.clone()) {
            return Ok(NamedFile::open(path).ok());
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(NamedFile::open(path).ok())
}

#[get("/<id>/state/should_hard_seek/<chunk_num>")]
pub fn should_client_hard_seek(
    state: State<StateManager>,
    id: String,
    chunk_num: u64,
) -> Result<JsonValue, errors::DimError> {
    Ok(json!({
        "should_client_seek": state
            .should_client_hard_seek(id, chunk_num)
            .map_err(|_| errors::StreamingErrors::OtherNightfall)?,
    }))
}
