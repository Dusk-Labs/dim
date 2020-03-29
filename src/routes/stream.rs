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
use slog::{info, Logger};
use std::{
    collections::HashMap,
    io::Cursor,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
use uuid::Uuid;

lazy_static::lazy_static! {
    /// Hashmp holding all the streams keyed by the media id, then subsequentely the quality
    /// TODO: Start using UUIDs
    ///                                   id            map     profile
    static ref STREAMS: Arc<RwLock<HashMap<(i32, String), HashMap<(String, String), Session>>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub(crate) fn cleanup_daemon(logger: Logger) {
    info!(logger, "Summoning the stream cleanup daemon");
    loop {
        let to_remove = {
            let mut to_remove = Vec::new();
            let lock = STREAMS.read().unwrap();
            for ((id, unique_id), streams) in lock.iter() {
                for ((map, profile), sess) in streams.iter() {
                    if sess.is_timeout() {
                        to_remove.push((
                            id.clone(),
                            unique_id.clone(),
                            map.clone(),
                            profile.clone(),
                        ));
                    }
                }
            }
            to_remove
        };

        {
            for (id, unique_id, map, profile) in to_remove {
                let mut lock = STREAMS.write().unwrap();
                if let Some(streams) = lock.get_mut(&(id, unique_id.clone())) {
                    if let Some(sess) = streams.remove(&(map.clone(), profile.clone())) {
                        info!(
                            logger,
                            "Deleting stale stream key: (id: {} unique_id: {} map: {} profile: {}) @ {}",
                            id,
                            unique_id,
                            map,
                            profile,
                            sess.current_chunk()
                        );
                        sess.join();
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn report_access(id: i32, unique_id: String, map: String, profile: String, chunk_num: u64) {
    let mut lock = STREAMS.write().unwrap();
    if let Some(streams) = lock.get_mut(&(id, unique_id)) {
        if let Some(stream) = streams.get_mut(&(map, profile)) {
            stream.reset_timeout(chunk_num);
        }
    }
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

    let stream_id = Uuid::new_v4().to_hyphenated().to_string();

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
        info.get_bitrate().as_str().parse::<u64>().unwrap_or(0),
        stream_id,
        stream_id,
        stream_id,
        stream_id
    );

    Response::build()
        .header(ContentType::new("application", "dash+xml"))
        .sized_body(Cursor::new(formatted))
        .ok()
}

// id: media id
// unique_id: random value to determine between user streams
// map: video/audio
// profile: bitrate/remux
// chunk: *.m4s/mp4
#[get("/stream/<id>/chunks/<unique_id>/<map>/<profile>/<chunk..>")]
pub fn return_static(
    conn: DbConnection,
    id: i32,
    unique_id: String,
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
    let full_path = Path::new("./transcoding").join(id.to_string());

    report_access(
        id,
        unique_id.clone(),
        map.clone(),
        profile.clone(),
        chunk_num,
    );

    {
        let lock = STREAMS.read().unwrap();

        // TODO: Mathematically determine if we are gonna get a chunk within the next 1.5k,
        // otherwise start a new stream.
        // If we are currently transcoding spin till a chunk is ready
        if let Some(session) = lock.get(&(id, unique_id.clone())) {
            for _ in 0..200 {
                if let Ok(x) = NamedFile::open(
                    full_path
                        .join(map.clone())
                        .join(profile.clone())
                        .join(chunk.clone()),
                ) {
                    // If we have a ongoing stream we check if the chunk is complete
                    // If we dont have a ongoing stream we assume the chunk is complete
                    if let Some(stream) = session.get(&(map.clone(), profile.clone())) {
                        if stream.is_chunk_done(chunk_num) {
                            return Ok(Some(x));
                        }
                    } else {
                        return Ok(Some(x));
                    }
                    // TODO: Replace this with a dameon that monitors a file with a timeout then returns Option<T>
                    thread::sleep(Duration::from_millis(10));
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
    }

    // kill any stream that matches the map and profile for our media id.
    {
        let mut lock = STREAMS.write().unwrap();
        if let Some(x) = lock.get_mut(&(id, unique_id.clone())) {
            if let Some(y) = x.remove(&(map.clone(), profile.clone())) {
                y.join();
            }
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

    {
        let mut lock = STREAMS.write().unwrap();
        if let Some(v) = lock.get_mut(&(id, unique_id.clone())) {
            v.insert((map.clone(), profile.clone()), session);
        } else {
            lock.insert((id, unique_id.clone()), {
                let mut m = HashMap::new();
                m.insert((map.clone(), profile.clone()), session);
                m
            });
        };
    }

    let lock = STREAMS.read().unwrap();
    if let Some(session) = lock.get(&(id, unique_id.clone())) {
        if let Some(stream) = session.get(&(map.clone(), profile.clone())) {
            for _ in 0..80 {
                if stream.is_chunk_done(chunk_num) {
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
