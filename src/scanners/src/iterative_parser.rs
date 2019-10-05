use crate::api::APIExec;
use crate::tmdb::TMDbSearch;
use crate::EventTx;
use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use dim_database::genre::*;
use dim_database::media::{InsertableMedia, Media};
use dim_database::{get_conn, library::Library, mediafile::*};
use dim_events::event::*;
use dim_streamer::{ffprobe::FFProbeCtx, FFPROBE_BIN};
use slog::Logger;
use std::path::PathBuf;
use torrent_name_parser::Metadata;
use walkdir::WalkDir;

pub struct IterativeScanner {
    conn: PgConnection,
    lib: Library,
    log: Logger,
    event_tx: EventTx,
}

impl<'a> IterativeScanner {
    pub fn new(library_id: i32, log: Logger, event_tx: EventTx) -> Result<Self, ()> {
        let conn = get_conn().expect("Failed to get a valid connection to db");

        if let Ok(lib) = Library::get_one(&conn, library_id) {
            return Ok(Self { conn, lib, log, event_tx});
        }

        Err(())
    }

    pub fn start(&self, custom_path: Option<&'a str>) {
        debug!(self.log, "Starting Movie scanner iterate");
        let path = match custom_path {
            Some(x) => x,
            None => self.lib.location.as_str(),
        };

        let files: Vec<PathBuf> = WalkDir::new(path)
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
            self.mount_file(file).unwrap();
        }

        self.fix_orphans();
    }

    pub fn mount_file(&self, file: PathBuf) -> Result<(), diesel::result::Error> {
        let path = file.clone().into_os_string().into_string().unwrap();

        if MediaFile::exists_by_file(&self.conn, &path) {
            return Ok(());
        }

        info!(self.log, "Scanning file: {}", &path);

        let ctx = FFProbeCtx::new(FFPROBE_BIN);
        let metadata = Metadata::from(file.file_name().unwrap().to_str().unwrap());
        let ffprobe_data = ctx.get_meta(&file).unwrap();

        let media_file = InsertableMediaFile {
            media_id: None,
            library_id: self.lib.id,
            target_file: path,

            raw_name: metadata.title().to_owned(),
            raw_year: metadata.year(),
            quality: ffprobe_data.get_quality(),
            codec: ffprobe_data.get_codec(),
            container: ffprobe_data.get_container(),
            audio: ffprobe_data.get_audio_type(),
            original_resolution: ffprobe_data.get_res(),
            duration: ffprobe_data.get_duration(),
            corrupt: ffprobe_data.is_corrupt(),

            season: metadata.season(),
            episode: metadata.episode(),
        };

        if let Err(err) = media_file.insert(&self.conn) {
            error!(
                self.log,
                "Failed to insert media_file {} {:?}", err, media_file
            );
        }

        Ok(())
    }

    pub fn fix_orphans(&self) {
        let mut tmdb_session = TMDbSearch::new("38c372f5bc572c8aadde7a802638534e");
        let orphans = MediaFile::get_by_lib(&self.conn, &self.lib).unwrap();

        info!(
            self.log,
            "Starting orphan scanner for library: {}", self.lib.id
        );

        for orphan in &orphans {
            if orphan.media_id.is_none() {
                let q_type = match self.lib.media_type.as_str() {
                    "tv" => true,
                    _ => false,
                };

                info!(self.log, "Scanning {} orphan", orphan.raw_name.clone());
                if let Some(result) = tmdb_session.search(orphan.raw_name.clone(), orphan.raw_year, q_type) {
                    self.match_media_to_tmdb(result, &orphan);
                }
            }
        }
    }

    fn match_media_to_tmdb(&self, result: crate::tmdb::QueryResult, orphan: &MediaFile) {
        let year: Option<i32> = match result.get_release_date() {
            Some(x) => Some(
                NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d")
                    .unwrap()
                    .year() as i32,
            ),
            None => None,
        };

        let media_id: i32;
        if let Ok(media) = Media::get_by_name_and_lib(&self.conn, &self.lib, &result.get_title().unwrap()) {
            media_id = media.id;
        } else {
            info!(self.log, "Inserting movie: {}", result.get_title().unwrap());
            let media = InsertableMedia {
                library_id: self.lib.id,
                name: result.get_title().unwrap(),
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
                media_type: self.lib.media_type.clone(),
            };

            media_id = match media.insert(&self.conn) {
                Ok(id) => id,
                Err(err) => {
                    error!(self.log, "Error inserting media: {}", err);
                    return;
                }
            };

            if let Some(y) = result.genres {
                for x in y {
                    let genre = InsertableGenre {
                        name: x.name.clone(),
                    };

                    let genre_id = genre.insert(&self.conn).unwrap();

                    let pair = InsertableGenreMedia { genre_id, media_id };

                    pair.insert(&self.conn);
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
            corrupt: None,
            episode: None,
            season: None,
        };

        updated_mediafile.update(&self.conn, orphan.id).unwrap();

        let event_message = Message {
            id: media_id,
            event_type: PushEventType::EventNewCard,
        };

        let new_event = Event::new(&format!("/events/library/{}", self.lib.id), event_message);

        let _ = self.event_tx.send(new_event);
    }
}
