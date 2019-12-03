use crate::core::DbConnection;
use crate::errors;
use auth::Wrapper as Auth;
use database::schema::mediafile::dsl::*;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket_contrib::{json, json::JsonValue};
use std::path::{Path, PathBuf};
use streamer::{ffmpeg::FFmpeg, FFMPEG_BIN};

#[get("/stream/start/<_id>?<seek>")]
pub fn start_stream(
    conn: DbConnection,
    _id: i32,
    seek: Option<u64>,
    _user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let mediafile_id = mediafile
        .filter(media_id.eq(Some(_id)))
        .select(id)
        .first::<i32>(conn.as_ref())?;

    let mut stream = FFmpeg::new(FFMPEG_BIN, mediafile_id)?;
    let uuid = stream.stream(seek)?;
    return Ok(json!({ "uuid": uuid }));
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
