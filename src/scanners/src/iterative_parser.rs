use crate::api::APIExec;
use crate::tmdb::TMDbSearch;
use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use dim_database::genre::*;
use dim_database::media::{InsertableMedia, Media};
use dim_database::{get_conn, library::Library, mediafile::*};
use dim_streamer::{ffprobe::FFProbeCtx, FFPROBE_BIN};
use rocket_slog::SyncLogger;
use std::path::PathBuf;
use torrent_name_parser::Metadata;
use walkdir::WalkDir;

pub fn start_iterative_parser(library_id: i32, log: SyncLogger) {
    let conn = get_conn().unwrap();
    let lib_inst = Library::get_one(&conn, library_id);

    if let Ok(lib) = lib_inst {
        iterate(conn, lib, log);
    }
}

fn iterate(conn: PgConnection, lib: Library, log: SyncLogger) {
    debug!(log, "Starting Movie scanner iterate");
    let files: Vec<PathBuf> = WalkDir::new(lib.location.as_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| {
            !f.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter(|x| {
            let ext = x.path().extension();
            match ext {
                Some(e) => ["mkv", "mp4", "avi"].contains(&e.to_str().unwrap()),
                None => false,
            }
        })
        .map(|f| f.into_path())
        .collect::<Vec<_>>();

    for file in files {
        mount_file(file, &lib, &conn, &log).unwrap();
    }
    iterate_stage2(conn, lib, &log);
}

fn mount_file(
    file: PathBuf,
    lib: &Library,
    conn: &PgConnection,
    log: &SyncLogger,
) -> Result<(), diesel::result::Error> {
    info!(
        log,
        "Scanning file: {}",
        file.file_name().unwrap().to_str().unwrap()
    );

    let ctx = FFProbeCtx::new(FFPROBE_BIN);
    let metadata = Metadata::from(file.file_name().unwrap().to_str().unwrap());
    let ffprobe_data = ctx.get_meta(&file).unwrap();

    let media_file = InsertableMediaFile {
        media_id: None,
        library_id: lib.id,
        target_file: file.into_os_string().into_string().unwrap(),

        raw_name: metadata.title().to_owned(),
        raw_year: metadata.year(),
        quality: ffprobe_data.get_quality(),
        codec: ffprobe_data.get_codec(),
        container: ffprobe_data.get_container(),
        audio: ffprobe_data.get_audio_type(),
        original_resolution: ffprobe_data.get_res(),
        duration: ffprobe_data.get_duration(),
    };

    if let Err(err) = media_file.insert(&conn) {
        error!(log, "Failed to insert media_file {} {:?}", err, media_file);
    }

    Ok(())
}

fn iterate_stage2(conn: PgConnection, lib: Library, log: &SyncLogger) {
    let mut tmdb_session = TMDbSearch::new("38c372f5bc572c8aadde7a802638534e");
    let orphans = MediaFile::get_by_lib(&conn, &lib).unwrap();
    for orphan in &orphans {
        if let Some(result) = tmdb_session.search(orphan.raw_name.clone(), orphan.raw_year) {
            match_media_to_tmdb(&conn, result, &lib, &orphan, &log);
        }
    }
}

fn match_media_to_tmdb(
    conn: &PgConnection,
    result: crate::tmdb::MovieResult,
    lib: &Library,
    orphan: &MediaFile,
    log: &SyncLogger,
) {
    let year: Option<i32> = match result.release_date {
        Some(x) => Some(
            NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d")
                .unwrap()
                .year() as i32,
        ),
        None => None,
    };

    let media_id: i32;
    if let Ok(media) = Media::get_by_name_and_lib(&conn, &lib, &result.title) {
        media_id = media.id;
    } else {
        let media = InsertableMedia {
            library_id: lib.id,
            name: result.title,
            description: result.overview,
            rating: match result.vote_average {
                Some(d) => Some(d as i32),
                None => None,
            },
            year,
            added: Utc::now().to_string(),
            poster_path: match result.poster_path {
                Some(path) => Some(format!(
                    "https://image.tmdb.org/t/p/w600_and_h900_bestv2{}",
                    path
                )),
                None => None,
            },
            backdrop_path: match result.backdrop_path {
                Some(path) => Some(format!("https://image.tmdb.org/t/p/original/{}", path)),
                None => None,
            },
            media_type: String::from("movie"),
        };

        media_id = match media.insert(&conn) {
            Ok(id) => id,
            Err(err) => {
                error!(log, "Error inserting media: {}", err);
                return;
            }
        };

        if let Some(y) = result.genres {
            for x in y {
                let genre = InsertableGenre {
                    name: x.name.clone(),
                };

                let genre_id = genre.insert(&conn).unwrap();

                let pair = InsertableGenreMedia { genre_id, media_id };

                pair.insert(&conn);
            }
        }
    }

    let updated_mediafile = UpdateMediaFile {
        media_id: Some(media_id),
        target_file: None,
        raw_name: None,
        raw_year: None,
        quality: None,
        codec: None,
        container: None,
        audio: None,
        original_resolution: None,
        duration: None,
    };

    updated_mediafile.update(&conn, orphan.id).unwrap();
}
