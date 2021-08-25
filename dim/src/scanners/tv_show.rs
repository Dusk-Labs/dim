use database::asset::InsertableAsset;
use database::genre::InsertableGenre;
use database::genre::InsertableGenreMedia;
use database::DbConnection;

use database::episode::InsertableEpisode;
use database::library::MediaType;
use database::media::InsertableMedia;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;
use database::movie::InsertableMovie;
use database::season::InsertableSeason;
use database::tv::TVShow;

use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;

use slog::debug;
use slog::error;
use slog::warn;
use slog::Logger;

use events::Message;
use events::PushEventType;

use crate::core::EventTx;
use crate::fetcher::PosterType;

pub struct TvShowMatcher<'a> {
    pub conn: &'a DbConnection,
    pub log: &'a Logger,
    pub event_tx: &'a EventTx,
}

impl<'a> TvShowMatcher<'a> {
    pub async fn match_to_result(&self, result: super::ApiMedia, orphan: &'a MediaFile) {
        let name = result.title.clone();

        let year: Option<i64> = result
            .release_date
            .clone()
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

        let poster = match poster_path {
            Some(path) => InsertableAsset {
                remote_url: Some(path),
                local_path: result
                    .poster_file
                    .clone()
                    .map(|x| format!("images/{}", x.trim_start_matches("/")))
                    .unwrap_or_default(),
                file_ext: "jpg".into(),
            }
            .insert(self.conn)
            .await
            .ok()
            .map(|x| x.id),
            None => None,
        };

        let backdrop = match backdrop_path {
            Some(path) => InsertableAsset {
                remote_url: Some(path),
                local_path: result
                    .backdrop_file
                    .clone()
                    .map(|x| format!("images/{}", x.trim_start_matches("/")))
                    .unwrap_or_default(),
                file_ext: "jpg".into(),
            }
            .insert(self.conn)
            .await
            .ok()
            .map(|x| x.id),
            None => None,
        };

        let media = InsertableMedia {
            name,
            year,
            library_id: orphan.library_id,
            description: result.overview.clone(),
            rating: result.rating.map(|x| x as i64),
            added: Utc::now().to_string(),
            poster,
            backdrop,
            media_type: MediaType::Tv,
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
        let meta_fetcher = crate::core::METADATA_FETCHER_TX.get().unwrap().get();

        let media_id = media.insert(&self.conn).await?;
        let _ = TVShow::insert(&self.conn, media_id).await;

        self.push_event(media_id).await;

        for name in result.genres {
            let genre = InsertableGenre { name };

            if let Ok(x) = genre.insert(&self.conn).await {
                let _ = InsertableGenreMedia::insert_pair(x, media_id, &self.conn).await;
            }
        }

        let season = {
            let orphan_season = orphan.season.unwrap_or(0) as u64;

            result
                .seasons
                .iter()
                .find(|s| s.season_number == orphan_season)
        };

        let poster_file = season.and_then(|x| x.poster_path.clone());

        if let Some(x) = poster_file.as_ref() {
            let _ = meta_fetcher.send(PosterType::Season(x.clone()));
        }

        let season_poster = match poster_file {
            Some(path) => InsertableAsset {
                remote_url: Some(path),
                local_path: season
                    .and_then(|x| x.poster_file.clone())
                    .map(|x| format!("images/{}", x.trim_start_matches("/")))
                    .unwrap_or_default(),
                file_ext: "jpg".into(),
            }
            .insert(self.conn)
            .await
            .ok()
            .map(|x| x.id),
            None => None,
        };

        let insertable_season = InsertableSeason {
            season_number: orphan.season.unwrap_or(0),
            added: Utc::now().to_string(),
            poster: season_poster,
        };

        let seasonid = insertable_season.insert(&self.conn, media_id).await?;

        let search_ep = {
            let orphan_episode = orphan.episode.unwrap_or(0) as u64;
            season.and_then(|x| {
                x.episodes
                    .iter()
                    .find(|&s| s.episode == Some(orphan_episode))
            })
        };

        let still = search_ep.as_ref().and_then(|x| x.still.clone());

        if let Some(x) = still.as_ref() {
            let _ = meta_fetcher.send(PosterType::Episode(x.clone()));
        }

        let backdrop = match still {
            Some(path) => InsertableAsset {
                remote_url: Some(path),
                local_path: search_ep
                    .and_then(|x| x.still_file.clone())
                    .clone()
                    .map(|x| format!("images/{}", x.trim_start_matches("/")))
                    .unwrap_or_default(),
                file_ext: "jpg".into(),
            }
            .insert(self.conn)
            .await
            .ok()
            .map(|x| x.id),
            None => None,
        };

        debug!(
            self.log,
            "Inserting new episode";
            "seasonid" => seasonid,
            "episode" => orphan.episode.unwrap_or(0),
            "target_file" => &orphan.target_file,
        );

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
                backdrop,
                ..Default::default()
            },
        };

        // manually insert the underlying `media` into the table and convert it into a streamable movie/ep
        let raw_ep_id = episode.media.insert_blind(&self.conn).await?;
        if let Err(e) = InsertableMovie::insert(&self.conn, raw_ep_id).await {
            error!(
                self.log,
                "Failed to turn episode into a streamable movie";
                "error" => format!("{:?}", e),
                "episode_id" => raw_ep_id,
                "file" => &orphan.target_file,
            );
        }

        let episode_id = episode.insert(&self.conn).await?;

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(episode_id),
            ..Default::default()
        };

        updated_mediafile.update(&self.conn, orphan.id).await?;

        Ok(())
    }

    async fn push_event(&self, id: i64) {
        use std::lazy::SyncLazy;
        use std::sync::Mutex;

        static DUPLICATE_LOG: SyncLazy<Mutex<Vec<i64>>> = SyncLazy::new(Default::default);

        {
            let mut lock = DUPLICATE_LOG.lock().unwrap();
            if lock.contains(&id) {
                return;
            }
            lock.push(id);
        }

        let event = Message {
            id,
            event_type: PushEventType::EventNewCard,
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());
    }
}
