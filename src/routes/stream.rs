use crate::{
    core::DbConnection,
    errors,
    streaming::{
        ffprobe::FFProbeCtx,
        transcode::{Profile, Session},
    },
};
use chrono::{prelude::*, NaiveDateTime, Utc};
use database::mediafile::MediaFile;
use rocket::{
    http::ContentType,
    response::{NamedFile, Response},
};
use std::{
    collections::HashMap,
    io::Cursor,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

lazy_static::lazy_static! {
    static ref STREAMS: Arc<Mutex<HashMap<i32, HashMap<String, Session>>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[get("/stream/<id>/manifest.mpd")]
pub fn return_manifest(conn: DbConnection, id: i32) -> Result<Response<'static>, errors::DimError> {
    let media = MediaFile::get_one(conn.as_ref(), id)?;
    let info = FFProbeCtx::new("/usr/bin/ffprobe")
        .get_meta(&std::path::PathBuf::from(media.target_file))?;

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

    let formatted = format!(
        include_str!("../static/manifest.mpd"),
        duration_string,
        duration_string,
        info.get_bitrate().as_str().parse::<u64>().unwrap_or(0)
    );

    Response::build()
        .header(ContentType::new("application", "dash+xml"))
        .sized_body(Cursor::new(formatted))
        .ok()
}

#[get("/stream/<id>/chunks/<map>/<profile>/<chunk..>")]
pub fn return_static(
    conn: DbConnection,
    id: i32,
    map: String,
    profile: String,
    chunk: PathBuf,
) -> Result<Option<NamedFile>, errors::DimError> {
    let asserted_profile = if map == "audio" {
        Profile::Audio
    } else {
        Profile::from_string(profile.clone())?
    };

    let extension = chunk.extension()?.to_string_lossy().into_owned();

    // Chunks will always be m4s or mp4
    if !["m4s", "mp4"].contains(&extension.as_str()) {
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

    let media = MediaFile::get_one(conn.as_ref(), id)?;
    let mut lock = STREAMS.lock().unwrap();

    let full_path = Path::new("./transcoding").join(id.to_string());

    // If we are currently transcoding spin till a chunk is ready
    if let Some(session) = lock.get(&id) {
        for _ in 0..20 {
            if let Ok(x) = NamedFile::open(
                full_path
                    .join(map.clone())
                    .join(profile.clone())
                    .join(chunk.clone()),
            ) {
                // If we have a ongoing stream we check if the chunk is complete
                // If we dont have a ongoing stream we assume the chunk is complete
                if let Some(stream) = session.get(&map) {
                    if stream.is_chunk_done(chunk_num) {
                        return Ok(Some(x));
                    }
                } else {
                    return Ok(Some(x));
                }
                // TODO: Replace this with a dameon that monitors a file with a timeout then returns Option<T>
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    // If we are not transcoding try to return a chunk
    if let Ok(x) = NamedFile::open(
        full_path
            .join(map.clone())
            .join(profile.clone())
            .join(chunk.clone()),
    ) {
        return Ok(Some(x));
    }

    if let Some(mut x) = lock.remove(&id) {
        if let Some(y) = x.remove(&map) {
            y.join();
        }
    }

    let session = if map == "video" {
        Session::new_video(
            media.target_file,
            asserted_profile,
            chunk_num,
            full_path.clone().into_os_string().into_string().unwrap(),
        )?
    } else if map == "audio" {
        Session::new_audio(
            media.target_file,
            chunk_num,
            full_path.clone().into_os_string().into_string().unwrap(),
        )?
    } else {
        return Err(errors::DimError::StreamingError(
            errors::StreamingErrors::InvalidProfile,
        ));
    };

    if let Some(v) = lock.get_mut(&id) {
        v.insert(map.clone(), session);
    } else {
        lock.insert(id, {
            let mut m = HashMap::new();
            m.insert(map.clone(), session);
            m
        });
    };

    if let Some(session) = lock.get(&id) {
        if let Some(stream) = session.get(&map) {
            println!("STREAM: {}", stream.process_id);
            for _ in 0..80 {
                if stream.is_chunk_done(chunk_num) {
                    println!("Chunk is done");
                    if let Ok(x) = NamedFile::open(
                        full_path
                            .join(map.clone())
                            .join(profile.clone())
                            .join(chunk.clone()),
                    ) {
                        return Ok(Some(x));
                    }
                }
                // TODO: Replace this with a dameon that monitors a file with a timeout then returns Option<T>
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
    Ok(None)
}
