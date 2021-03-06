use crate::core::DbConnection;
use crate::errors;
use crate::streaming::ffprobe::FFProbeCtx;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;

use database::mediafile::MediaFile;

use rocket::http::ContentType;
use rocket::request::State;
use rocket::response::NamedFile;
use rocket::response::Response;

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

    let formatted = format!(
        include_str!("../static/manifest.mpd"),
        duration_string,
        duration_string,
        info.get_bitrate(),
        video,
        video,
        start_num.unwrap_or(0),
        audio,
        audio,
        start_num.unwrap_or(0)
    );

    Response::build()
        .header(ContentType::new("application", "dash+xml"))
        .sized_body(Cursor::new(formatted))
        .ok()
}

#[get("/stream/<id>/init.mp4", rank = 1)]
pub fn get_init(
    state: State<StateManager>,
    id: String,
) -> Result<Option<NamedFile>, errors::StreamingErrors> {
    let path = state.init_or_create(id)?;

    Ok(NamedFile::open(path).ok())
}

#[get("/stream/<id>/<chunk..>", rank = 2)]
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

    if let Err(_) = state.exists(id.clone()) {
        state
            .init_or_create(id.clone())
            .map_err(|_| errors::StreamingErrors::OtherNightfall)?;
    }

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
