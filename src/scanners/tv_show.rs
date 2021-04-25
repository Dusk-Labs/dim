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
use database::movie::InsertableMovie;
use database::season::InsertableSeason;
use database::season::Season;
use database::tv::InsertableTVShow;

use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;

use slog::debug;
use slog::error;
use slog::info;
use slog::warn;
use slog::Logger;

use events::Message;
use events::PushEventType;

use tmdb::Tmdb;

use crate::core::EventTx;

use super::tmdb;

pub struct TvShowMatcher<'a> {
    pub conn: &'a DbConnection,
    pub log: &'a Logger,
    pub event_tx: &'a EventTx,
}

impl<'a> TvShowMatcher<'a> {
    pub async fn match_to_result(&self, result: super::ApiMedia, orphan: &'a MediaFile) {
        let name = result.title.clone();

        let year: Option<i32> = result
            .release_date
            .clone()
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
            name,
            year,
            library_id: orphan.library_id,
            description: result.overview.clone(),
            rating: result.rating,
            added: Utc::now().to_string(),
            poster_path: result.poster_file.clone().map(|x| format!("images/{}", x)),
            backdrop_path: result
                .backdrop_file
                .clone()
                .map(|x| format!("images/{}", x)),
            media_type: MediaType::Tv,
        };

        if let Err(e) = self.insert(orphan, media, result).await {
            warn!(
                self.log,
                "Failed to insert new media";
                "id" => orphan.id,
                "reason" => format!("{:?}", e),
            );
        }
    }

    async fn insert(
        &self,
        orphan: &MediaFile,
        media: InsertableMedia,
        result: super::ApiMedia,
    ) -> Result<(), super::base::ScannerError> {
        let meta_fetcher = crate::core::METADATA_FETCHER_TX.get().unwrap().get();

        let media_id = media.insert(&self.conn).await?;
        let _ = media
            .into_static::<InsertableTVShow>(&self.conn, media_id)
            .await;

        for name in result.genres {
            let genre = InsertableGenre { name };

            if let Ok(x) = genre.insert(&self.conn).await {
                InsertableGenreMedia::insert_pair(x, media_id, &self.conn).await;
            }
        }

        let season = {
            let orphan_season = orphan.season.unwrap_or(0) as u64;

            result
                .seasons
                .iter()
                .find(|s| s.season_number == orphan_season)
        };

        if let Some(x) = season.and_then(|x| x.poster_path.as_ref()) {
            let _ = meta_fetcher.send(x.clone());
        }

        let insertable_season = InsertableSeason {
            season_number: orphan.season.unwrap_or(0),
            added: Utc::now().to_string(),
            poster: season
                .and_then(|x| x.poster_file.clone())
                .map(|s| format!("images/{}", s))
                .unwrap_or_default(),
        };

        let seasonid = insertable_season.insert(&self.conn, media_id).await?;

        let search_ep = {
            let orphan_episode = orphan.episode.unwrap_or(0) as u64;
            season.and_then(|x| {
                x.episodes
                    .iter()
                    .cloned()
                    .find(|s| s.episode == Some(orphan_episode))
            })
        };

        if let Some(x) = search_ep.as_ref().and_then(|x| x.still.clone()) {
            let _ = meta_fetcher.send(x);
        }

        let episode = InsertableEpisode {
            episode: orphan.episode.unwrap_or(0),
            seasonid,
            media: InsertableMedia {
                library_id: orphan.library_id,
                name: search_ep
                    .as_ref()
                    .and_then(|x| x.name.clone())
                    .unwrap_or_else(|| orphan.episode.unwrap_or(0).to_string()),
                added: Utc::now().to_string(),
                media_type: MediaType::Episode,
                description: search_ep
                    .as_ref()
                    .map(|x| x.overview.clone())
                    .unwrap_or_default(),
                backdrop_path: search_ep
                    .and_then(|x| x.still_file)
                    .map(|s| format!("images/{}", s)),
                ..Default::default()
            },
        };

        // manually insert the underlying `media` into the table and convert it into a streamable movie/ep
        let raw_ep_id = episode.media.insert(&self.conn).await?;
        let _ = episode
            .media
            .into_streamable::<InsertableMovie>(&self.conn, raw_ep_id, Some(()))
            .await;

        let episode_id = dbg!(episode.insert(&self.conn, media_id, raw_ep_id).await)?;

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(episode_id),
            ..Default::default()
        };

        updated_mediafile.update(&self.conn, orphan.id).await?;

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
