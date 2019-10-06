use crate::api::APIExec;
use crate::tmdb::TMDbSearch;
use crate::EventTx;
use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use dim_database::genre::*;
use dim_database::media::{InsertableMedia, Media};
use dim_database::movie::{InsertableMovie};
use dim_database::{get_conn, library::Library, mediafile::*};
use dim_events::event::*;
use dim_streamer::{ffprobe::FFProbeCtx, FFPROBE_BIN};
use slog::Logger;
use std::path::PathBuf;
use torrent_name_parser::Metadata;
use walkdir::WalkDir;
use rayon::prelude::*;

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

        let files: Vec<String> = WalkDir::new(path)
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
            .map(|f| f.into_path().to_str().unwrap().to_owned())
            .collect::<Vec<_>>();

        let logger = self.log.clone();
        let lib_id = self.lib.id;
        files.par_iter()
            .for_each(|x| mount_file(logger.clone(), x.clone(), lib_id).unwrap());

        self.fix_orphans();
    }

    pub fn mount_file(&self, file: PathBuf) -> Result<(), diesel::result::Error> {
        mount_file(self.log.clone(), file.to_str().unwrap().to_owned(), self.lib.id)
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
                    self.match_media_to_tmdb(result, &orphan, q_type);
                }
            }
        }
    }

    fn match_media_to_tmdb(
        &self,
        result: crate::tmdb::QueryResult,
        orphan: &MediaFile,
        tv: bool,
    ) {
        let name = result.get_title().unwrap();

        let year: Option<i32> = result
            .get_release_date()
            .map(|x| NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d"))
            .map(Result::ok)
            .unwrap_or(None)
            .map(|s| s.year() as i32);

        let rating = result.vote_average
            .map(|x| x as i32);

        let poster_path = result.poster_path
            .map(|s| format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", s));

        let backdrop_path = result.backdrop_path
            .map(|s| format!("https://image.tmdb.org/t/p/original/{}", s));

        let media = InsertableMedia {
            library_id: self.lib.id,
            name,
            description: result.overview,
            rating,
            year,
            added: Utc::now().to_string(),
            poster_path,
            backdrop_path,
            media_type: self.lib.media_type.clone(),
        };

        if tv {
            self.insert_tv(orphan, media);
            return;
        }

        self.insert_movie(orphan, media, result.genres);
    }
 
    fn insert_tv(&self, _: &MediaFile, _: InsertableMedia) {
    }

    fn insert_movie(&self, orphan: &MediaFile, media: InsertableMedia, genres: Option<Vec<crate::tmdb::Genre>>) {
        let media_id = Media::get_by_name_and_lib(&self.conn, &self.lib, media.name.clone().as_str())
            .map_or_else(
                |_| media.into_streamable::<InsertableMovie>(&self.conn).unwrap(),
                |x| x.id);

        if let Some(genres) = genres {
            for genre in genres {
                let genre = InsertableGenre {
                    name: genre.name.clone()
                };

                let _ = genre.insert(&self.conn)
                    .map(|z| InsertableGenreMedia::insert_pair(z, media_id, &self.conn));
            }
        };

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(media_id),
            ..Default::default()
        };
        
        updated_mediafile.update(&self.conn, orphan.id).unwrap();
        self.push_event(media_id);
    }

    fn push_event(&self, media_id: i32) {
        let event_msg = Message {
            id: media_id,
            event_type: PushEventType::EventNewCard,
        };

        let new_event = Event::new(&format!("/events/library/{}", self.lib.id), event_msg);
        let _ = self.event_tx.send(new_event);
    }
}

pub fn mount_file(log: Logger, file: String, lib_id: i32) -> Result<(), diesel::result::Error> {
    let file = std::path::PathBuf::from(file);
    let conn = get_conn().unwrap();
    let path = file.clone().into_os_string().into_string().unwrap();

    if MediaFile::exists_by_file(&conn, &path) {
        return Ok(());
    }

    info!(log, "Scanning file: {}", &path);

    let ctx = FFProbeCtx::new(FFPROBE_BIN);
    let metadata = Metadata::from(file.file_name().unwrap().to_str().unwrap()).unwrap();
    let ffprobe_data = ctx.get_meta(&file).unwrap();

    let media_file = InsertableMediaFile {
        media_id: None,
        library_id: lib_id,
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

    if let Err(err) = media_file.insert(&conn) {
        error!(
            log,
            "Failed to insert media_file {} {:?}", err, media_file
            );
    }

    Ok(())
}
