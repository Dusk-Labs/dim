use crate::tmdb_api;
use crate::tmdb_api::TMDbSearch;
use crate::APIExec;
use crate::EventTx;
use chrono::{prelude::Utc, Datelike, NaiveDate};
use database::{
    episode::{Episode, InsertableEpisode},
    genre::*,
    get_conn,
    library::{Library, MediaType},
    media::{InsertableMedia, Media},
    mediafile::*,
    movie::InsertableMovie,
    season::{InsertableSeason, Season},
    tv::InsertableTVShow,
};
use diesel::pg::PgConnection;
use events::*;
use pushevent::Event;
use rayon::prelude::*;
use slog::{debug, error, info, Logger};
use streamer::{ffprobe::FFProbeCtx, FFPROBE_BIN};
use torrent_name_parser::Metadata;
use walkdir::WalkDir;

pub struct IterativeScanner {
    conn: PgConnection,
    lib: Library,
    log: Logger,
    event_tx: EventTx,
}

impl IterativeScanner {
    pub fn new(library_id: i32, log: Logger, event_tx: EventTx) -> Result<Self, ()> {
        let conn = get_conn().expect("Failed to get a valid connection to db");

        if let Ok(lib) = Library::get_one(&conn, library_id) {
            return Ok(Self {
                conn,
                lib,
                log,
                event_tx,
            });
        }

        Err(())
    }

    pub fn start(&self, custom_path: Option<&str>) {
        debug!(self.log, "Starting Movie scanner iterate");
        let path = match custom_path {
            Some(x) => x,
            None => self.lib.location.as_str(),
        };

        let files: Vec<String> = WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|f| {
                !f.path()
                    .iter()
                    .any(|s| s.to_str().unwrap().starts_with('.'))
            })
            .filter(|x| {
                let ext = x.path().extension();
                match ext {
                    Some(e) => {
                        ["mkv", "mp4", "avi"].contains(&e.to_string_lossy().into_owned().as_str())
                    }
                    None => false,
                }
            })
            .filter_map(|f| {
                f.into_path().to_str().map_or_else(
                    || {
                        println!("Failed to unwrap full path");
                        None
                    },
                    |x| Some(x.to_owned()),
                )
            })
            .collect::<Vec<_>>();

        let logger = self.log.clone();
        let lib_id = self.lib.id;
        files.par_iter().for_each(|x| {
            let _ = mount_file(logger.clone(), x.clone(), lib_id).map_err(|e| {
                slog::error!(logger, "Failed mounting file into the database: {:?}", e)
            });
        });

        self.fix_orphans();
    }

    pub fn mount_file<T: std::string::ToString>(
        &self,
        file: T,
    ) -> Result<(), &dyn std::error::Error> {
        mount_file(self.log.clone(), file.to_string(), self.lib.id)
    }

    pub fn fix_orphans(&self) {
        let mut tmdb_session = TMDbSearch::new("38c372f5bc572c8aadde7a802638534e");
        let orphans = match MediaFile::get_by_lib(&self.conn, &self.lib) {
            Ok(x) => x,
            Err(e) => {
                slog::error!(self.log, "Database fucked up somehow: {:?}", e);
                return;
            }
        };

        info!(
            self.log,
            "Starting orphan scanner for library: {}", self.lib.id
        );

        for orphan in orphans {
            if orphan.media_id.is_none() {
                let mediatype = match self.lib.media_type {
                    MediaType::Tv => tmdb_api::MediaType::Tv,
                    _ => tmdb_api::MediaType::Movie,
                };

                info!(
                    self.log,
                    "Scanning orphan with raw name: {}", orphan.raw_name
                );
                if let Some(result) =
                    tmdb_session.search(orphan.raw_name.clone(), orphan.raw_year, mediatype)
                {
                    self.match_media_to_tmdb(result, &orphan, mediatype);
                }
            }
        }
    }

    fn match_media_to_tmdb(
        &self,
        result: tmdb_api::Media,
        orphan: &MediaFile,
        mediatype: tmdb_api::MediaType,
    ) {
        let name = match result.get_title() {
            Some(x) => x,
            None => {
                println!("TMDBApi returned a None title");
                return;
            }
        };

        let year: Option<i32> = result
            .get_release_date()
            .map(|x| NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d"))
            .map(Result::ok)
            .unwrap_or(None)
            .map(|s| s.year() as i32);

        let rating = result.vote_average.map(|x| x as i32);

        let poster_path = result
            .poster_path
            .clone()
            .map(|s| format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", s));

        let backdrop_path = result
            .backdrop_path
            .clone()
            .map(|s| format!("https://image.tmdb.org/t/p/original/{}", s));

        let media = InsertableMedia {
            library_id: self.lib.id,
            name,
            description: result.overview.clone(),
            rating,
            year,
            added: Utc::now().to_string(),
            poster_path,
            backdrop_path,
            media_type: self.lib.media_type.clone(),
        };

        if let crate::tmdb_api::MediaType::Tv = mediatype {
            self.insert_tv(orphan, media, result);
            return;
        }

        self.insert_movie(orphan, media, result);
    }

    fn insert_tv(&self, orphan: &MediaFile, media: InsertableMedia, search: tmdb_api::Media) {
        let media_id =
            Media::get_by_name_and_lib(&self.conn, &self.lib, media.name.clone().as_str())
                .map_or_else(
                    |_| {
                        media
                            .into_static::<InsertableTVShow>(&self.conn)
                            .and_then(|x| {
                                self.push_event(x);
                                Ok(x)
                            })
                            .unwrap()
                    },
                    |x| x.id,
                );

        if let Some(genres) = search.genres.clone() {
            for genre in genres {
                let genre = InsertableGenre {
                    name: genre.name.clone(),
                };

                let _ = genre
                    .insert(&self.conn)
                    .map(|z| InsertableGenreMedia::insert_pair(z, media_id, &self.conn));
            }
        };

        let season = search.get_season(orphan.season.unwrap());
        let seasonid = Season::get(&self.conn, media_id, orphan.season.unwrap()).map_or_else(
            |_| {
                let season = InsertableSeason {
                    season_number: orphan.season.unwrap(),
                    added: Utc::now().to_string(),
                    poster: String::from(""),
                };

                season.insert(&self.conn, media_id).unwrap()
            },
            |x| x.id,
        );

        let episode_id = Episode::get(
            &self.conn,
            media_id,
            orphan.season.unwrap(),
            orphan.episode.unwrap(),
        )
        .map_or_else(
            |_| {
                let search_ep = season.get_episode(orphan.episode.unwrap());
                let episode = InsertableEpisode {
                    episode: orphan.episode.unwrap(),
                    seasonid,
                    media: InsertableMedia {
                        library_id: orphan.library_id,
                        name: format!("{}", orphan.episode.unwrap()),
                        added: Utc::now().to_string(),
                        media_type: MediaType::Episode,
                        description: search_ep.overview,
                        backdrop_path: search_ep.still_path,
                        ..Default::default()
                    },
                };

                episode.insert(&self.conn, media_id).unwrap()
            },
            |x| x.id,
        );

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(episode_id),
            ..Default::default()
        };

        updated_mediafile.update(&self.conn, orphan.id).unwrap();
    }

    fn insert_movie(
        &self,
        orphan: &MediaFile,
        media: InsertableMedia,
        search: crate::tmdb_api::Media,
    ) {
        let media_id =
            Media::get_by_name_and_lib(&self.conn, &self.lib, media.name.clone().as_str())
                .map_or_else(
                    |_| {
                        media
                            .into_streamable::<InsertableMovie>(&self.conn, None)
                            .unwrap()
                    },
                    |x| x.id,
                );

        // TODO: use .map instead of if let
        if let Some(genres) = search.genres {
            for genre in genres {
                let genre = InsertableGenre {
                    name: genre.name.clone(),
                };

                let _ = genre
                    .insert(&self.conn)
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
        let event_msg = Box::new(Message {
            id: media_id,
            event_type: PushEventType::EventNewCard,
        });

        let new_event = Event::new(format!("/events/library/{}", self.lib.id), event_msg);
        let _ = self.event_tx.send(new_event);
    }
}

fn mount_file(
    log: Logger,
    file: String,
    lib_id: i32,
) -> Result<(), &'static dyn std::error::Error> {
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
        error!(log, "Failed to insert media_file {} {:?}", err, media_file);
    }

    Ok(())
}
