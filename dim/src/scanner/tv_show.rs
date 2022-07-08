#![allow(unstable_name_collisions)]

use crate::external::ExternalMedia;
use crate::external::ExternalSeason;
use crate::external::ExternalEpisode;
use crate::inspect::ResultExt;

use database::Transaction;
use database::media::InsertableMedia;
use database::library::MediaType;
use database::mediafile::MediaFile;
use database::episode::InsertableEpisode;

use chrono::prelude::Utc;
use chrono::Datelike;

use tracing::error;

pub struct TvMatcher;

impl TvMatcher {
    async fn match_to_result<'life0>(
        &self,
        tx: &mut Transaction<'life0>,
        file: MediaFile,
        result: (ExternalMedia, ExternalSeason, ExternalEpisode),
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: insert poster and backdrops.
        let (emedia, eseason, eepisode) = result;

        let media = InsertableMedia {
            media_type: MediaType::Tv,
            library_id: file.library_id,
            name: emedia.title,
            description: emedia.description,
            rating: emedia.rating,
            year: emedia.release_date.map(|x| x.year() as _),
            added: Utc::now().to_string(),
            poster: None,
            backdrop: None,
        };

        let parent_id = media.lazy_insert(tx, file.media_id).await
            .inspect_err(|error| error!(?error, ?file, "Failed to lazy insert tv show"))?;

        Ok(())
    }
}
