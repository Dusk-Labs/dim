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

use tracing::warn;
use tracing::error;

use crate::core::EventTx;
use crate::fetcher::insert_into_queue;

pub struct MovieMatcher<'a> {
    pub conn: &'a DbConnection,
    pub event_tx: &'a EventTx,
}

impl<'a> MovieMatcher<'a> {
    pub async fn match_to_result(&self, result: super::ApiMedia, orphan: &'a MediaFile) {
        let name = result.title.clone();

        let mut tx = match self.conn.write().begin().await {
            Ok(x) => x,
            Err(e) => {
                error!(reason = ?e, "Failed to create transaction.");
                return;
            }
        };

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
                    local_path: result
                        .poster_file
                        .clone()
                        .map(|x| format!("images/{}", x.trim_start_matches("/")))
                        .unwrap_or_default(),
                    file_ext: "jpg".into(),
                    ..Default::default()
                }
                .insert(&mut tx)
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
                    local_path: result
                        .backdrop_file
                        .clone()
                        .map(|x| format!("images/{}", x.trim_start_matches("/")))
                        .unwrap_or_default(),
                    file_ext: "jpg".into(),
                    ..Default::default()
                }
                .insert(&mut tx)
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

        if let Err(e) = tx.commit().await {
            error!(reason = ?e, "Failed to commit transaction.");
            return;
        }

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

        if let Err(e) = self.insert(orphan, media, result).await {
            warn!(
                reason = ?e,
                orphan_id = orphan.id,
                "Failed to insert new media",
            );
        }
    }

    async fn insert(
        &self,
        orphan: &MediaFile,
        media: InsertableMedia,
        result: super::ApiMedia,
    ) -> Result<(), super::base::ScannerError> {
        let mut tx = self
            .conn
            .write()
            .begin()
            .await
            .map_err(|e| super::base::ScannerError::DatabaseError(format!("{:?}", e)))?;
        let media_id = media.insert(&mut tx).await?;
        // the reason we ignore the result here is that in some cases this can fail. Specifically when there are multiple mediafiles for a movie.
        let _ = InsertableMovie::insert(&mut tx, media_id).await;

        for name in result.genres {
            let genre = InsertableGenre { name };

            if let Ok(x) = genre.insert(&mut tx).await {
                let _ = InsertableGenreMedia::insert_pair(x, media_id, &mut tx).await;
            }
        }

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(media_id),
            ..Default::default()
        };

        updated_mediafile.update(&mut tx, orphan.id).await?;
        tx.commit()
            .await
            .map_err(|e| super::base::ScannerError::DatabaseError(format!("{:?}", e)))?;

        self.push_event(media_id, media.library_id).await;

        Ok(())
    }

    async fn push_event(&self, id: i64, lib_id: i64) {
        // TODO: verify if this scanner suffers from the same duplicate top-level media insertion
        // bug.
        let event = Message {
            id,
            event_type: PushEventType::EventNewCard { lib_id },
        };

        let _ = self.event_tx.send(serde_json::to_string(&event).unwrap());
    }
}
