#![allow(unstable_name_collisions)]

use crate::external::ExternalEpisode;
use crate::external::ExternalMedia;
use crate::external::ExternalSeason;
use crate::inspect::ResultExt;

use database::episode::InsertableEpisode;
use database::library::MediaType;
use database::media::InsertableMedia;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;
use database::season::InsertableSeason;
use database::Transaction;

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
    ) -> Result<(i64, i64, i64), Box<dyn std::error::Error>> {
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

        let parent_id = media
            .lazy_insert(tx)
            .await
            .inspect_err(|error| error!(?error, ?file, "Failed to lazy insert tv show"))?;

        // TODO: Decouple then re-attach genres for current tv show.
        
        let seasonid = self.match_to_season(tx, parent_id, eseason).await?;
        let episodeid = self.match_to_episode(tx, file, seasonid, eepisode).await?;

        Ok((parent_id, seasonid, episodeid))
    }

    // FIXME: In cases where we can match against a show but not find a specific season or episode,
    // we want to backfill the data as Season 0 or as an Extra.
    async fn match_to_season(
        &self,
        tx: &mut Transaction<'_>,
        parent_id: i64,
        result: ExternalSeason,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // TODO: Fetch poster.
        let season = InsertableSeason {
            season_number: result.season_number as _,
            added: Utc::now().to_string(),
            poster: None,
        };

        let season_id = season
            .insert(tx, parent_id)
            .await
            .inspect_err(|error| error!(?error, "Failed to insert season object."))?;

        Ok(season_id)
    }

    async fn match_to_episode(
        &self,
        tx: &mut Transaction<'_>,
        file: MediaFile,
        seasonid: i64,
        result: ExternalEpisode
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // NOTE: Add backdrops
        let media = InsertableMedia {
            library_id: file.library_id,
            name: result.title_or_episode(),
            added: Utc::now().to_string(),
            media_type: MediaType::Episode,
            description: result.description.clone(),
            ..Default::default()
        };

        let episode = InsertableEpisode {
            episode: result.episode_number as _,
            seasonid,
            media,
        };

        let episode_id = episode.media.insert_blind(&mut *tx).await
            .inspect_err(|error| error!(?error, ?file, "Failed to insert media for episode."))?;

        // NOTE: WE use to turn a episode into a movie here, not sure if necessary anymore.

        let episode_id = episode.insert(&mut *tx).await
            .inspect_err(|error| error!(?error, ?file, "Failed to insert episode."))?;

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(episode_id),
            ..Default::default()
        };

        updated_mediafile
            .update(&mut *tx, file.id)
            .await
            .inspect_err(|error| error!(?error, ?file, "Failed to update mediafile media id."))?;

        Ok(episode_id)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::mediafile::create_library;
    use super::TvMatcher;

    use crate::external::ExternalMedia;
    use crate::external::ExternalSeason;
    use crate::external::ExternalEpisode;

    use chrono::TimeZone;
    use database::genre::Genre;
    use database::media::Media;
    use database::mediafile::InsertableMediaFile;
    use database::mediafile::MediaFile;
    use database::rw_pool::write_tx;

    #[tokio::test(flavor = "multi_thread")]
    async fn match_show() {
        const MATCHER: TvMatcher = TvMatcher;

        let mut conn = database::get_conn_memory()
            .await
            .expect("Failed to obtain a in-memory db pool.");
        let library = create_library(&mut conn).await;

        let mut lock = conn.writer.lock_owned().await;
        let mut tx = write_tx(&mut lock).await.unwrap();

        let mut mediafile = InsertableMediaFile {
            library_id: library,
            target_file: "test.mp4".into(),
            raw_name: "test".into(),
            ..Default::default()
        };

        let mfile_id = mediafile
            .insert(&mut tx)
            .await
            .unwrap();

        let emedia = ExternalMedia {
            title: "Show 1".into(),
            ..Default::default()
        };

        let eseason = ExternalSeason {
            season_number: 1,
            ..Default::default()
        };

        let eepisode = ExternalEpisode {
            episode_number: 1,
            ..Default::default()
        };

        let mut result = (emedia, eseason, eepisode);

        let mfile = MediaFile::get_one(&mut tx, mfile_id).await.unwrap();
        let (m1, s1, e1) = MATCHER.match_to_result(&mut tx, mfile, result.clone()).await.unwrap();

        mediafile.target_file = "test2.mp4".into();
        let mfile2_id = mediafile.insert(&mut tx).await.unwrap();

        let mfile2 = MediaFile::get_one(&mut tx, mfile2_id).await.unwrap();
        let (m2, s2, e2) = MATCHER.match_to_result(&mut tx, mfile2, result.clone()).await.unwrap();

        // We attach two mediafiles to the same episode, season and show
        assert_eq!(m1, m2);
        assert_eq!(s1, s2);
        assert_eq!(e1, e2);

        mediafile.target_file = "test3.mp4".into();
        result.2.episode_number = 2;
        let mfile3_id = mediafile.insert(&mut tx).await.unwrap();

        let mfile3 = MediaFile::get_one(&mut tx, mfile3_id).await.unwrap();
        let (m3, s3, e3) = MATCHER.match_to_result(&mut tx, mfile3, result.clone()).await.unwrap();

        // we attach a third mediafile to the same show and season but different episode
        assert_eq!(m2, m3);
        assert_eq!(s2, s3);
        assert_ne!(e2, e3);

        mediafile.target_file = "test4.mp4".into();
        result.1.season_number = 2;
        let mfile4_id = mediafile.insert(&mut tx).await.unwrap();

        let mfile4 = MediaFile::get_one(&mut tx, mfile4_id).await.unwrap();
        let (m4, s4, e4) = MATCHER.match_to_result(&mut tx, mfile4, result.clone()).await.unwrap();

        // we attach a fourth mediafile to a different season and episode but same show
        assert_eq!(m4, m3);
        assert_ne!(s4, s3);
        assert_ne!(e4, e3);
    }
}
