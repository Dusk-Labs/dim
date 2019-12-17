use crate::core::DbConnection;
use crate::errors;
use chrono::prelude::*;
use chrono::{NaiveDateTime, Utc};
use database::mediafile::MediaFile;
use rocket::http::ContentType;
use rocket::response::{NamedFile, Response};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use streamer::transcode::Session;

lazy_static::lazy_static! {
    static ref STREAMS: Arc<Mutex<HashMap<i32, Session>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[get("/stream/<id>/manifest.mpd")]
pub fn return_manifest(conn: DbConnection, id: i32) -> Result<Response<'static>, errors::DimError> {
    let media = MediaFile::get_one(conn.as_ref(), id)?;
    let info = streamer::ffprobe::FFProbeCtx::new("/usr/bin/ffprobe")
        .get_meta(&std::path::PathBuf::from(media.target_file))
        .unwrap();

    let mut ms = info.get_ms().unwrap().to_string();
    ms.truncate(4);

    let duration = chrono::DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(info.get_duration().unwrap() as i64, 0),
        Utc,
    );

    println!("{:#?}", info);

    let duration_string = format!(
        "PT{}H{}M{}.{}S",
        duration.hour(),
        duration.minute(),
        duration.second(),
        ms
    );

    let formatted = format!(
        include_str!("../streaming/static/manifest.mpd"),
        duration_string,
        duration_string,
        info.get_bitrate().as_str().parse::<u64>().unwrap()
    );

    Response::build()
        .header(ContentType::new("application", "dash+xml"))
        .sized_body(Cursor::new(formatted))
        .ok()
}

#[get("/stream/<id>/chunks/<path>/<chunk..>")]
pub fn return_static(
    conn: DbConnection,
    id: i32,
    path: String,
    chunk: PathBuf,
) -> Option<NamedFile> {
    let extension = match chunk.extension() {
        Some(x) => x.to_string_lossy().into_owned(),
        None => return None,
    };

    // Chunks will always be m4s or mp4
    if !["m4s", "mp4"].contains(&extension.as_str()) {
        println!("{:?}", chunk.file_stem());
        return None;
    }

    // Parse the chunk filename into a u64, we unwrap_or because sometimes it can be a init chunk,
    // if its a init chunk we assume a chunk index of 0 because we are fetching the first few
    // chunks.
    let chunk_num = match chunk.file_stem() {
        Some(x) => x.to_string_lossy().into_owned().parse::<u64>().unwrap_or(0),
        None => return None,
    };

    let media = MediaFile::get_one(conn.as_ref(), id).unwrap();
    let mut lock = STREAMS.lock().unwrap();

    let full_path = Path::new("./transcoding").join(id.to_string());

    if let Some(_) = lock.get(&id) {
        for _ in 0..30 {
            if let Ok(x) = NamedFile::open(full_path.join(path.clone()).join(chunk.clone())) {
                return Some(x);
            }
            // TODO: Replace this with a dameon that monitors a file with a timeout then returns Option<T>
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    if let Some(x) = lock.remove(&id) {
        x.join();
    }

    let session = Session::new(
        media.target_file,
        None,
        chunk_num,
        full_path.clone().into_os_string().into_string().unwrap(),
    )
    .unwrap();

    lock.insert(id, session);

    for _ in 0..40 {
        if let Ok(x) = NamedFile::open(full_path.join(path.clone()).join(chunk.clone())) {
            return Some(x);
        }
        // TODO: Replace this with a dameon that monitors a file with a timeout then returns Option<T>
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    None
}
