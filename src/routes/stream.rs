use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;
use auth::Wrapper as Auth;
use database::schema::mediafile::dsl::*;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::{json, json::JsonValue};
use rocket_slog::SyncLogger;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use streamer::{ffmpeg::FFmpeg, FFMPEG_BIN};

#[get("/stream/start/<_id>?<seek>&<vcodec>&<acodec>&<_brate>&<_res>")]
pub fn start_stream(
    conn: DbConnection,
    _id: i32,
    _user: Auth,
    seek: Option<u64>,
    vcodec: Option<String>,
    acodec: Option<String>,
    _brate: Option<u64>,
    _res: Option<u64>,
    log: SyncLogger,
    event_tx: State<Arc<Mutex<EventTx>>>,
) -> Result<JsonValue, errors::DimError> {
    let mediafile_id = mediafile
        .filter(media_id.eq(Some(_id)))
        .select(id)
        .first::<i32>(conn.as_ref())?;

    let seek = seek.unwrap_or(0);
    let _vcodec = vcodec.unwrap_or_else(|| "copy".to_string());
    let _acodec = acodec.unwrap_or_else(|| "aac".to_string());

    let event_tx = event_tx.lock().unwrap().clone();

    let mut stream = FFmpeg::new(FFMPEG_BIN, mediafile_id, event_tx, log.get().clone())?;
    let uuid = stream.stream(seek)?;
    Ok(json!({ "uuid": uuid }))
}

#[delete("/stream/<uuid>")]
pub fn stop_stream(uuid: String, _user: Auth) -> Result<Status, errors::DimError> {
    FFmpeg::stop(uuid)?;
    Ok(Status::Ok)
}

#[get("/stream/static/<uuid>/<path..>")]
pub fn return_static(uuid: String, path: PathBuf, _user: Auth) -> Option<NamedFile> {
    let full_path = Path::new("./transcoding").join(uuid);
    NamedFile::open(full_path.join(path)).ok()
}
