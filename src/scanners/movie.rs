use database::genre::InsertableGenre;
use database::genre::InsertableGenreMedia;
use database::movie::InsertableMovie;
use database::DbConnection;

use database::library;
use database::library::Library;

use database::media::InsertableMedia;
use database::media::Media;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;

use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;

use slog::error;
use slog::info;
use slog::warn;
use slog::Logger;

use events::Message;
use events::PushEventType;

use crate::core::EventTx;

use super::tmdb;
use super::tmdb::Tmdb;
use super::MediaScanner;
use super::MetadataAgent;
use super::ScannerDaemon;

pub struct MovieScanner {
    conn: DbConnection,
    lib: Library,
    log: Logger,
    event_tx: EventTx,
}

impl MovieScanner {
    fn match_media_to_tmdb(&self, result: super::ApiMedia, orphan: &MediaFile) {
        let name = result.title.clone();

        let year: Option<i32> = result
            .release_date
            .as_ref()
            .clone()
            .map(|st| st.clone())
            .map(|x| NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d"))
            .map(Result::ok)
            .unwrap_or(None)
            .map(|s| s.year() as i32);

        let poster_path = result.poster_path.clone();

        let backdrop_path = result.backdrop_path.clone();

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
            rating: result.rating,
            year,
            added: Utc::now().to_string(),

            poster_path: result.poster_file.clone().map(|x| format!("images/{}", x)),
            backdrop_path: result
                .backdrop_file
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
        result: super::ApiMedia,
    ) -> Result<(), ()> {
        let media_id = Media::get_by_name_and_lib(&self.conn, &self.lib, media.name.as_str())
            .map_or_else(
                |_| {
                    // this should never panic unless some serious fuckup happened to the db
                    media
                        .into_streamable::<InsertableMovie>(&self.conn, None)
                        .unwrap()
                },
                |x| x.id,
            );

        for name in result.genres {
            let genre = InsertableGenre { name };

            let _ = genre
                .insert(&self.conn)
                .map(|z| InsertableGenreMedia::insert_pair(z, media_id, &self.conn));
        }

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(media_id),
            ..Default::default()
        };

        updated_mediafile
            .update(&self.conn, orphan.id)
            .map_err(|_| ())?;

        self.push_event(media_id);

        Ok(())
    }

    fn push_event(&self, id: i32) {
        let event = Message {
            id,
            event_type: PushEventType::EventNewCard,
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());
    }
}

impl MediaScanner for MovieScanner {
    const MEDIA_TYPE: library::MediaType = library::MediaType::Movie;

    fn new_unchecked(
        conn: DbConnection,
        lib: Library,
        log: Logger,
        event_tx: EventTx,
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

        let mut tmdb_session = Tmdb::new(
            "38c372f5bc572c8aadde7a802638534e".to_string(),
            tmdb::MediaType::Movie,
        );
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
                    "Scanning orphan with raw name: {}", orphan.raw_name
                );

                let result = match tmdb_session.search(orphan.raw_name.clone(), orphan.raw_year) {
                    Ok(v) => v,
                    Err(e) => {
                        error!(self.log, "fix-orphans: {:?}", e);
                        continue;
                    }
                };

                self.match_media_to_tmdb(result, &orphan);
            }
        }
    }
}

impl ScannerDaemon for MovieScanner {}
