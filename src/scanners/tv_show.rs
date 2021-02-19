use database::genre::InsertableGenre;
use database::genre::InsertableGenreMedia;
use database::DbConnection;

use database::library;
use database::library::Library;
use database::library::MediaType;

use database::episode::Episode;
use database::episode::InsertableEpisode;
use database::media::InsertableMedia;
use database::media::Media;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;
use database::season::InsertableSeason;
use database::season::Season;
use database::tv::InsertableTVShow;

use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;

use slog::error;
use slog::info;
use slog::warn;
use slog::Logger;

use events::Message;
use events::PushEventType;
use pushevent::Event;

use super::tmdb_api;
use super::tmdb_api::TMDbSearch;
use super::APIExec;
use super::MediaScanner;
use super::ScannerDaemon;

pub struct TvShowScanner {
    conn: DbConnection,
    lib: Library,
    log: Logger,
    event_tx: super::EventTx,
}

impl TvShowScanner {
    fn match_media_to_tmdb(&self, result: tmdb_api::Media, orphan: &MediaFile) {
        let name = if let Some(x) = result.get_title() {
            x
        } else {
            warn!(
                self.log,
                "TMDB returned a none title for orphan={}", orphan.id
            );
            return;
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

        let meta_fetcher = crate::core::METADATA_FETCHER_TX.get().unwrap().get();

        if let Some(poster_path) = poster_path.as_ref() {
            let _ = meta_fetcher.send(poster_path.clone());
        }

        if let Some(backdrop_path) = backdrop_path.as_ref() {
            let _ = meta_fetcher.send(backdrop_path.clone());
        }

        let media = InsertableMedia {
            library_id: self.lib.id,
            name,
            description: result.overview.clone(),
            rating,
            year,
            added: Utc::now().to_string(),
            poster_path: result.poster_path.clone().map(|x| format!("images/{}", x)),
            backdrop_path: result
                .backdrop_path
                .clone()
                .map(|x| format!("images/{}", x)),
            media_type: Self::MEDIA_TYPE,
        };

        if self.insert(orphan, media, result).is_err() {
            warn!(
                self.log,
                "Failed to insert new media for orphan={}", orphan.id
            );
        }
    }

    fn insert(
        &self,
        orphan: &MediaFile,
        media: InsertableMedia,
        search: tmdb_api::Media,
    ) -> Result<(), ()> {
        let meta_fetcher = crate::core::METADATA_FETCHER_TX.get().unwrap().get();

        let media_id = Media::get_by_name_and_lib(&self.conn, &self.lib, media.name.as_str())
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

        let season = search.get_season(orphan.season.unwrap_or(0));

        if let Some(x) = season.poster_path.as_ref() {
            let _ = meta_fetcher.send(format!("https://images.tmdb.org/t/p/original/{}", x));
        }

        let seasonid = Season::get(&self.conn, media_id, orphan.season.unwrap()).map_or_else(
            |_| {
                let season = InsertableSeason {
                    season_number: orphan.season.unwrap_or(0),
                    added: Utc::now().to_string(),
                    poster: season
                        .poster_path
                        .clone()
                        .map(|s| format!("images/{}", s))
                        .unwrap_or_default(),
                };

                season.insert(&self.conn, media_id).unwrap()
            },
            |x| x.id,
        );

        let episode_id = Episode::get(
            &self.conn,
            media_id,
            orphan.season.unwrap_or(0),
            orphan.episode.unwrap_or(0),
        )
        .map_or_else(
            |_| {
                let search_ep = season.get_episode(orphan.episode.unwrap_or(0));

                if let Some(x) = search_ep.still_path.as_ref() {
                    let _ =
                        meta_fetcher.send(format!("https://images.tmdb.org/t/p/original/{}", x));
                }

                let episode = InsertableEpisode {
                    episode: orphan.episode.unwrap_or(0),
                    seasonid,
                    media: InsertableMedia {
                        library_id: orphan.library_id,
                        name: search_ep
                            .name
                            .unwrap_or_else(|| orphan.episode.unwrap_or(0).to_string()),
                        added: Utc::now().to_string(),
                        media_type: MediaType::Episode,
                        description: search_ep.overview,
                        backdrop_path: search_ep.still_path.map(|s| format!("images/{}", s)),
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

        Ok(())
    }

    fn push_event(&self, id: i32) {
        let event_msg = Box::new(Message {
            id,
            event_type: PushEventType::EventNewCard,
        });

        let new_event = Event::new(format!("/events/library/{}", self.lib.id), event_msg);
        let _ = self.event_tx.send(new_event);
    }
}

impl MediaScanner for TvShowScanner {
    const MEDIA_TYPE: library::MediaType = library::MediaType::Tv;

    fn new_unchecked(
        conn: DbConnection,
        lib: Library,
        log: Logger,
        event_tx: super::EventTx,
    ) -> Self {
        Self {
            conn,
            lib,
            log,
            event_tx,
        }
    }

    fn logger_ref(&self) -> &Logger {
        &self.log
    }

    fn library_ref(&self) -> &Library {
        &self.lib
    }

    fn conn_ref(&self) -> &DbConnection {
        &self.conn
    }

    fn fix_orphans(&self) {
        assert!(self.lib.media_type == Self::MEDIA_TYPE);
        info!(self.log, "Scanning orphans for lib={}", self.lib.id);

        let mut tmdb_session = TMDbSearch::new("38c372f5bc572c8aadde7a802638534e");
        let orphans = match MediaFile::get_by_lib(&self.conn, &self.lib) {
            Ok(x) => x,
            Err(e) => {
                error!(self.log, "Database fucked up somehow: {:?}", e);
                return;
            }
        };

        for orphan in orphans {
            if orphan.media_id.is_none() {
                info!(
                    self.log,
                    "Scanning orphan with raw name: {} ep={:?} season={:?}",
                    orphan.raw_name,
                    orphan.episode,
                    orphan.season
                );

                if let Some(result) = tmdb_session.search(
                    orphan.raw_name.clone(),
                    orphan.raw_year,
                    tmdb_api::MediaType::Tv,
                ) {
                    self.match_media_to_tmdb(result, &orphan);
                }
            }
        }
    }
}

impl ScannerDaemon for TvShowScanner {}
