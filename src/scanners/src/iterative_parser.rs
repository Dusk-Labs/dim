use dim_database::{library::Library, mediafile::*, get_conn};
use dim_database::media::{InsertableMedia, Media};
use walkdir::WalkDir;
use diesel::pg::PgConnection;
use torrent_name_parser::Metadata;
use std::path::PathBuf;
use crate::api::APIExec;
use crate::tmdb::TMDbSearch;
use chrono::prelude::Utc;
use chrono::NaiveDate;
use chrono::Datelike;


pub fn start_iterative_parser(library_id: i32) {
    let conn = get_conn().unwrap();

    let lib_inst = Library::get_one(&conn, library_id);
    match lib_inst {
        Ok(lib) => iterate(conn, lib),
        Err(_) => return,
    }
}

fn iterate(conn: PgConnection, lib: Library) {
    println!("Stage 1");
    let files: Vec<PathBuf> = WalkDir::new(lib.location.as_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| !f.file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false))
        .filter(|x| {
            let ext = x.path().extension();
            match ext {
                Some(e) => ["mkv", "mp4", "avi"].contains(&e.to_str().unwrap()),
                None => return false,
            }
        })
        .map(|f| f.into_path())
        .collect::<Vec<_>>();

    for file in files {
        println!("[SCANNER] Scanning file: {}", file.file_name().unwrap().to_str().unwrap());
        let metadata = Metadata::from(file.file_name().unwrap().to_str().unwrap());

        let media_file = InsertableMediaFile {
            media_id: None,
            library_id: lib.id,
            target_file: file.into_os_string().into_string().unwrap(),

            raw_name: metadata.title().to_owned(),
            raw_year: metadata.year(),
            quality: Some(String::from("1080p")),
            codec: Some(String::from("h264")),
            container: Some(String::from("matroska")),
            audio: Some(String::from("DTS-ES")),
            original_resolution: Some(String::from("1920x1080")),
            duration: Some(2836),
        };

        if let Err(err) = media_file.insert(&conn) {
            println!("[ITERATIVE_PARSER] Failed to insert media_file {} {:?}", err, media_file);
        }
    }
    iterate_stage2(conn, lib);
}

fn iterate_stage2(conn: PgConnection, lib: Library) {
    let mut tmdb_session = TMDbSearch::new("38c372f5bc572c8aadde7a802638534e");
    let orphans = MediaFile::get_by_lib(&conn, &lib).unwrap();
    for orphan in &orphans {
        if let Some(result) = tmdb_session.search(orphan.raw_name.clone(), orphan.raw_year) {
            let year: Option<i32> = match result.release_date {
                Some(x) => Some(NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d").unwrap().year() as i32),
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
                    year: year,
                    added: Utc::now().to_string(),
                    poster_path: match result.poster_path {
                        Some(path) => Some(format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", path)),
                        None => None,
                    },
                    backdrop_path: match result.backdrop_path {
                        Some(path) => Some(format!("https://image.tmdb.org/t/p/original/{}", path)),
                        None => None,
                    },
                    media_type: String::from("movie"),
                };

                media_id = match media.new(&conn) {
                    Ok(id) => id,
                    Err(err) => {
                        println!("[ITERATE_STAGE2] Error inserting media {}", err);
                        continue
                    },
                };
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
    }
}
