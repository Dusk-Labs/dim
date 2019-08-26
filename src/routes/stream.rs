use crate::core::DbConnection;
use diesel::prelude::*;
use dim_streamer::{FFMPEG_BIN, ffmpeg::FFmpeg};
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

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

        Ok(json!({"uuid": uuid}))
    } else {
        Err(Status::NotFound)
    }
}

#[get("/stream/static/<uuid>/<path..>")]
pub fn return_static(uuid: String, path: PathBuf) -> Option<NamedFile> {
    let full_path = Path::new("/home/hinach4n/media/media1/transcoding").join(uuid);
    /*
    if path.to_str().unwrap() == "index.m3u8" {
        std::fs::copy(full_path.join(&path), full_path.join("new_index.mpd")).unwrap();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(full_path.join("new_index.mpd"))
            .unwrap();

        writeln!(file, "#EXT-X-ENDLIST");
        return NamedFile::open(full_path.join("new_index.mpd")).ok()
    }*/
    NamedFile::open(full_path.join(path)).ok()
}
