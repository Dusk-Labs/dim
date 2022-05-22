use crate::external::ExternalMedia;
use crate::external::ExternalQuery;
use crate::inspect::ResultExt;

use super::MediaMatcher;
use super::WorkUnit;

use async_trait::async_trait;
use chrono::prelude::Utc;
use chrono::Datelike;

use database::library::MediaType;
use database::media::InsertableMedia;
use database::media::Media;
use database::mediafile::UpdateMediaFile;
use database::mediafile::MediaFile;
use database::Transaction;

use std::sync::Arc;
use tracing::error;

pub struct MovieMatcher;

impl MovieMatcher {
    /// Method will match a mediafile to a new media. Caller must ensure that the mediafile
    /// supplied is not coupled to a media object. If it is coupled we will assume that we can
    /// replace the metadata supplied to it.
    async fn match_to_result(
        &self,
        tx: &mut Transaction<'_>,
        file: MediaFile,
        provided: ExternalMedia,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Push posters and backdrops to download queue.

        let media = InsertableMedia {
            media_type: MediaType::Movie,
            library_id: file.library_id,
            name: provided.title,
            description: provided.description,
            rating: provided.rating,
            year: provided.release_date.map(|x| x.year() as _),
            added: Utc::now().to_string(),
            poster: None,
            backdrop: None,
        };

        // NOTE: If the mediafile is coupled to a media we assume that we want to reuse the media
        // object, but replace its metadata in-place. This is useful when rematching a media.
        let media_id = if file.media_id.is_some() {
            media.insert(tx, file.media_id).await.inspect_err(|error| {
                error!(
                    ?error,
                    ?file.media_id,
                    "Failed to assign mediafile to media."
                )
            })?
        } else {
            // Maybe a media object that can be linked against this file already exists and we want
            // to bind to it?
            match Media::get_id_by_name(tx, &media.name)
                .await
                .inspect_err(|error| error!(?error, %media.name, "Failed to get a media by name"))?
            {
                Some(id) => id,
                None => media
                    .insert(tx, None)
                    .await
                    .inspect_err(|error| error!(?error, "Failed to insert media object."))?,
            }
        };

        // NOTE: Previous scanner had a `InsertableMovie::insert`. Honestly no clue if we need
        // that, or if that is a remnant from the openflix days.

        // Update mediafile to point to a new parent media_id. We also want to set raw_name and
        // raw_year to what its parent has so that when we refresh metadata, files that were
        // matched manually (due to bogus filenames) dont get unmatched, or matched wrongly.
        UpdateMediaFile {
            media_id: Some(media_id),
            raw_name: Some(media.name),
            raw_year: media.year,
            ..Default::default()
        }
        .update(tx, file.id)
        .await
        .inspect_err(|error| {
            error!(?error, "Failed to update mediafile to point to new parent.")
        })?;

        Ok(())
    }
}

#[async_trait]
impl MediaMatcher for MovieMatcher {
    async fn batch_match(self: Arc<Self>, provider: Arc<dyn ExternalQuery>, work: Vec<WorkUnit>) {
        let metadata_futs = work
            .into_iter()
            .map(|WorkUnit(file, metadata)| async {
                for meta in metadata {
                    match provider.search(meta.name.as_ref(), meta.year.map(|x| x as _)).await {
                        Ok(provided) => return Some((file, provided)),
                        Err(e) => error!(?meta, "Failed to find a movie match."),
                    }
                }

                None
            })
            .collect::<Vec<_>>();

        let metadata = futures::future::join_all(metadata_futs).await;
    }
}
