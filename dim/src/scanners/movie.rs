use database::asset::InsertableAsset;
use database::genre::InsertableGenre;
use database::genre::InsertableGenreMedia;
use database::movie::InsertableMovie;
use database::DbConnection;

use database::library::MediaType;
use database::media::InsertableMedia;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;

use chrono::prelude::Utc;
use chrono::Datelike;
use chrono::NaiveDate;

use events::Message;
use events::PushEventType;

use tracing::error;
use tracing::instrument;
use tracing::warn;

use super::format_path;
use crate::core::EventTx;
use crate::fetcher::insert_into_queue;

pub struct MovieMatcher<'a> {
    pub conn: &'a DbConnection,
    pub event_tx: &'a EventTx,
}

impl<'a> MovieMatcher<'a> {
    #[instrument(skip(self, result), fields(result.id = %result.id, result.name = %result.title))]
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

        if let Err(e) = tx.commit().await {
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
            .as_ref()
            .cloned()
            .map(|x| NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d"))
            .map(Result::ok)
            .unwrap_or(None)
            .map(|s| s.year() as i64);

        let poster_path = result.poster_path.clone();

        let backdrop_path = result.backdrop_path.clone();

        if let Some(poster_path) = poster_path.as_ref() {
            let _ = insert_into_queue(poster_path.clone(), 3);
        }

        if let Some(backdrop_path) = backdrop_path.as_ref() {
            let _ = insert_into_queue(backdrop_path.clone(), 3);
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
            library_id: orphan.library_id,
            name,
            description: result.overview.clone(),
            rating: result.rating.map(|x| x as i64),
            year,
            added: Utc::now().to_string(),

            poster,
            backdrop,
            media_type: MediaType::Movie,
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
        // the reason we ignore the result here is that in some cases this can fail. Specifically when there are multiple mediafiles for a movie.
        let _ = InsertableMovie::insert(&mut *tx, media_id).await;

        for name in result.genres {
            let genre = InsertableGenre { name };

            if let Ok(x) = genre.insert(&mut *tx).await {
                let _ = InsertableGenreMedia::insert_pair(x, media_id, &mut *tx).await;
            }
        }

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(media_id),
            ..Default::default()
        };

        updated_mediafile.update(&mut *tx, orphan.id).await?;

        Ok(media_id)
    }

    async fn push_event(&self, id: i64, lib_id: i64, mediafile: i64) {
        // TODO: verify if this scanner suffers from the same duplicate top-level media insertion
        // bug.
        let event = Message {
            id,
            event_type: PushEventType::EventNewCard { lib_id },
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());

        // Notify that a mediafile was matched.
        let event = Message {
            id,
            event_type: PushEventType::MediafileMatched { mediafile, library_id: lib_id },
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());
    }
}
