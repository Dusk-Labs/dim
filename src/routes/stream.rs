use crate::core::DbConnection;
use diesel::prelude::*;
use dim_streamer::{ffmpeg::FFmpeg, FFMPEG_BIN};
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket_contrib::json::JsonValue;
use std::path::{Path, PathBuf};

#[get("/stream/start/<_id>")]
pub fn start_stream(conn: DbConnection, _id: i32) -> Result<JsonValue, Status> {
    use dim_database::schema::mediafile::dsl::*;
    let media_inst = mediafile
        .filter(media_id.eq(Some(_id)))
        .select(id)
        .first::<i32>(&*conn);

    if let Ok(m_id) = media_inst {
        let mut stream = match FFmpeg::new(FFMPEG_BIN, m_id) {
            Ok(x) => x,
            Err(_) => return Err(Status::NotFound),
        };

        let uuid = match stream.stream() {
            Ok(uuid) => uuid,
            Err(_) => return Err(Status::NotFound),
        };

        return Ok(json!({ "uuid": uuid }));
    }
    Err(Status::NotFound)
}

#[get("/stream/static/<uuid>/<path..>")]
pub fn return_static(uuid: String, path: PathBuf) -> Option<NamedFile> {
    let full_path = Path::new("/home/hinach4n/media/media1/transcoding").join(uuid);
    NamedFile::open(full_path.join(path)).ok()
}
