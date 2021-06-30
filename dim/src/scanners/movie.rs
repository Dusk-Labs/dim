use database::genre::InsertableGenre;
use database::genre::InsertableGenreMedia;
use database::movie::InsertableMovie;
use database::DbConnection;

use database::library;
use database::library::Library;
use database::library::MediaType;

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

use crate::core::{fetcher::PosterType, EventTx};

use super::tmdb;
use super::tmdb::Tmdb;

pub struct MovieMatcher<'a> {
    pub conn: &'a DbConnection,
    pub log: &'a Logger,
    pub event_tx: &'a EventTx,
}

impl<'a> MovieMatcher<'a> {
    pub async fn match_to_result(&self, result: super::ApiMedia, orphan: &'a MediaFile) {
        let name = result.title.clone();

        let year: Option<i64> = result
            .release_date
            .as_ref()
            .clone()
            .map(|st| st.clone())
            .map(|x| NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d"))
            .map(Result::ok)
            .unwrap_or(None)
            .map(|s| s.year() as i64);

        let poster_path = result.poster_path.clone();

        let backdrop_path = result.backdrop_path.clone();

        let meta_fetcher = crate::core::METADATA_FETCHER_TX.get().unwrap().get();

        if let Some(poster_path) = poster_path.as_ref() {
            let _ = meta_fetcher.send(PosterType::Banner(poster_path.clone()));
        }

        if let Some(backdrop_path) = backdrop_path.as_ref() {
            let _ = meta_fetcher.send(PosterType::Banner(backdrop_path.clone()));
        }

        let media = InsertableMedia {
            library_id: orphan.library_id,
            name,
            description: result.overview.clone(),
            rating: result.rating.map(|x| x as i64),
            year,
            added: Utc::now().to_string(),

            poster_path: result.poster_file.clone().map(|x| format!("images/{}", x)),
            backdrop_path: result
                .backdrop_file
                .clone()
                .map(|x| format!("images/{}", x)),

            media_type: MediaType::Movie,
        };

        if let Err(e) = self.insert(orphan, media, result).await {
            warn!(
                self.log,
                "Failed to insert new media";
                "id" => orphan.id,
                "reason" => e.to_string(),
            );
        }
    }

    async fn insert(
        &self,
        orphan: &MediaFile,
        media: InsertableMedia,
        result: super::ApiMedia,
    ) -> Result<(), super::base::ScannerError> {
        let media_id = media.insert(&self.conn).await?;
        // the reason we ignore the result here is that in some cases this can fail. Specifically when there are multiple mediafiles for a movie.
        let _ = InsertableMovie::insert(&self.conn, media_id).await;

        for name in result.genres {
            let genre = InsertableGenre { name };

            if let Ok(x) = genre.insert(&self.conn).await {
                InsertableGenreMedia::insert_pair(x, media_id, &self.conn).await?;
            }
        }

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(media_id),
            ..Default::default()
        };

        updated_mediafile.update(&self.conn, orphan.id).await?;

        self.push_event(media_id).await;

        Ok(())
    }

    async fn push_event(&self, id: i64) {
        let event = Message {
            id,
            event_type: PushEventType::EventNewCard,
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());
    }
}
