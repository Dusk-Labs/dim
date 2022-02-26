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

use events::Message;
use events::PushEventType;

use tracing::debug;
use tracing::debug_span;
use tracing::error;
use tracing::instrument;
use tracing::warn;
use tracing::Instrument;

use super::format_path;
use crate::core::EventTx;
use crate::fetcher::insert_into_queue;

pub struct TvShowMatcher<'a> {
    pub conn: &'a DbConnection,
    pub event_tx: &'a EventTx,
}

impl<'a> TvShowMatcher<'a> {
    #[instrument(skip(self, result, orphan), fields(result.id = %result.id, result.name = %result.title, orphan.id = %orphan.id))]
    pub async fn match_to_result(&self, result: super::ApiMedia, orphan: &'a MediaFile) {
        let library_id = orphan.library_id;
        let mut lock = self.conn.writer().lock_owned().await;
        let mut tx = match database::write_tx(&mut lock).await {
            Ok(x) => x,
            Err(e) => {
                error!(reason = ?e, "Failed to create transaction.");
                return;
            }
        };

        let media_id = match self.inner_match(result, orphan, &mut tx, None).await {
            Ok(x) => x,
            Err(e) => {
                error!(reason = ?e, "Failed to match media");
                return;
            }
        };

        if let Err(e) = tx.commit().instrument(debug_span!("TxCommit")).await {
            error!(reason = ?e, "Failed to commit transaction.");
            return;
        }

        self.push_event(media_id, library_id, orphan.id).await;
    }

    pub async fn inner_match(
        &self,
        result: super::ApiMedia,
        orphan: &'a MediaFile,
        tx: &mut database::Transaction<'_>,
        reuse_media_id: Option<i64>,
    ) -> Result<i64, super::base::ScannerError> {
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

        if let Some(poster_path) = poster_path.as_ref() {
            let _ = insert_into_queue(poster_path.clone(), 3).await;
        }

        if let Some(backdrop_path) = backdrop_path.as_ref() {
            let _ = insert_into_queue(backdrop_path.clone(), 3).await;
        }

        let poster = match poster_path {
            Some(path) => {
                let asset = InsertableAsset {
                    remote_url: Some(path),
                    local_path: format_path(result.poster_file.clone()),
                    file_ext: "jpg".into(),
                }
                .insert(&mut *tx)
                .await;

                match asset {
                    Ok(x) => Some(x.id),
                    Err(e) => {
                        warn!(
                            reason = ?e,
                            orphan_id = orphan.id,
                            "Failed to insert poster into db",
                        );

                        None
                    }
                }
            }
            None => None,
        };

        let backdrop = match backdrop_path {
            Some(path) => {
                let asset = InsertableAsset {
                    remote_url: Some(path),
                    local_path: format_path(result.backdrop_file.clone()),
                    file_ext: "jpg".into(),
                }
                .insert(&mut *tx)
                .await;

                match asset {
                    Ok(x) => Some(x.id),
                    Err(e) => {
                        warn!(
                            reason = ?e,
                            orphan_id = orphan.id,
                            "Failed to insert backdrop into db",
                        );
                        None
                    }
                }
            }
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

        let media_id = self
            .inner_insert(orphan, media, result, &mut *tx, reuse_media_id)
            .await
            .map_err(|e| {
                error!(reason = ?e, orphan_id = orphan.id, "Failed to insert new media.");
                e
            })?;

        Ok(media_id)
    }

    #[instrument(skip(self, result, orphan, tx, reuse_media_id), level = "debug")]
    async fn inner_insert(
        &self,
        orphan: &MediaFile,
        media: InsertableMedia,
        result: super::ApiMedia,
        tx: &mut database::Transaction<'_>,
        reuse_media_id: Option<i64>,
    ) -> Result<i64, super::base::ScannerError> {
        let media_id = if let Some(id) = reuse_media_id {
            media.insert_with_id(&mut *tx, id).await?
        } else {
            media.insert(&mut *tx).await?
        };

        let _ = TVShow::insert(&mut *tx, media_id).await;

        for name in result.genres {
            let genre = InsertableGenre { name };

            if let Ok(x) = genre.insert(&mut *tx).await {
                let _ = InsertableGenreMedia::insert_pair(x, media_id, &mut *tx).await;
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
            let _ = insert_into_queue(x.clone(), 2).await;
        }

        let season_poster = match poster_file {
            Some(path) => {
                let asset = InsertableAsset {
                    remote_url: Some(path),
                    local_path: format_path(season.and_then(|x| x.poster_file.clone())),
                    file_ext: "jpg".into(),
                }
                .insert(&mut *tx)
                .await;

                match asset {
                    Ok(x) => Some(x.id),
                    Err(e) => {
                        warn!(
                            reason = ?e,
                            orphan_id = orphan.id,
                            "Failed to insert season poster into db"
                        );

                        None
                    }
                }
            }
            None => None,
        };

        let insertable_season = InsertableSeason {
            season_number: orphan.season.unwrap_or(0),
            added: Utc::now().to_string(),
            poster: season_poster,
        };

        let seasonid = match insertable_season.insert(&mut *tx, media_id).await {
            Ok(x) => x,
            Err(e) => {
                warn!(
                    "Failed to insert season into the database. {}",
                    reason = e.to_string()
                );
                return Err(e.into());
            }
        };

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
            let _ = insert_into_queue(x.clone(), 1).await;
        }

        let backdrop = match still {
            Some(path) => {
                let asset = InsertableAsset {
                    remote_url: Some(path),
                    local_path: format_path(search_ep.and_then(|x| x.still_file.clone()).clone()),
                    file_ext: "jpg".into(),
                }
                .insert(&mut *tx)
                .await;

                match asset {
                    Ok(x) => Some(x.id),
                    Err(e) => {
                        warn!(
                            reason = ?e,
                            orphan_id = orphan.id,
                            "Failed to insert still into db",
                        );

                        None
                    }
                }
            }
            None => None,
        };

        debug!(
            seasonid = seasonid,
            episode = orphan.episode.unwrap_or(0),
            target_file = ?&orphan.target_file,
            "Inserting new episode",
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
        let raw_ep_id = episode.media.insert_blind(&mut *tx).await?;
        if let Err(e) = InsertableMovie::insert(&mut *tx, raw_ep_id).await {
            error!(
                error = ?e,
                episode_id = raw_ep_id,
                file = ?&orphan.target_file,
                "Failed to turn episode into a streamable movie",
            );
        }

        let episode_id = episode.insert(&mut *tx).await?;

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(episode_id),
            ..Default::default()
        };

        updated_mediafile
            .update(&mut *tx, orphan.id)
            .instrument(debug_span!("UpdateMediafile"))
            .await?;

        Ok(media_id)
    }

    async fn push_event(&self, id: i64, lib_id: i64, mediafile: i64) {
        use once_cell::sync::Lazy;
        use std::sync::Mutex;

        static DUPLICATE_LOG: Lazy<Mutex<Vec<(i64, i64)>>> = Lazy::new(Default::default);

        // Notify that a mediafile was matched.
        let event = Message {
            id,
            event_type: PushEventType::MediafileMatched { mediafile, library_id: lib_id },
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());

        {
            let mut lock = DUPLICATE_LOG.lock().unwrap();
            if lock.contains(&(lib_id, id)) {
                return;
            }
            lock.push((lib_id, id));
        }

        let event = Message {
            id,
            event_type: PushEventType::EventNewCard { lib_id },
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());
    }
}
