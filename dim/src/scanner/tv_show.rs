#![allow(unstable_name_collisions)]

use super::movie::asset_from_url;
use super::MediaMatcher;
use super::Metadata;
use super::WorkUnit;
use crate::external::ExternalEpisode;
use crate::external::ExternalMedia;

use crate::external::ExternalQueryIntoShow;
use crate::external::ExternalQueryShow;
use crate::external::ExternalSeason;
use crate::inspect::ResultExt;

use async_trait::async_trait;

use dim_database::episode::Episode;
use dim_database::episode::InsertableEpisode;
use dim_database::genre::Genre;
use dim_database::genre::InsertableGenre;
use dim_database::genre::InsertableGenreMedia;
use dim_database::library::MediaType;
use dim_database::media::InsertableMedia;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use dim_database::mediafile::UpdateMediaFile;
use dim_database::movie::Movie;
use dim_database::season::InsertableSeason;
use dim_database::season::Season;
use dim_database::tv::TVShow;
use dim_database::Transaction;

use chrono::prelude::Utc;
use chrono::Datelike;

use serde::Serialize;
use std::sync::Arc;
use tracing::error;
use tracing::info;
use tracing::instrument;

use displaydoc::Display;
use thiserror::Error;

#[derive(Clone, Debug, Display, Error, Serialize)]
pub enum Error {
    /// Failed to insert poster into database: {0:?}
    PosterInsert(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to insert backdrop into database: {0:?}
    BackdropInsert(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to decouple genres from media: {0:?}
    GenreDecouple(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to create or get genre: {0:?}
    GetOrInsertGenre(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to attach genre to media object: {0:?}
    CoupleGenre(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to update mediafile to point to new parent: {0:?}
    UpdateMediafile(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to get children count for movie: {0:?}
    ChildrenCount(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to cleanup child-less parent: {0:?}
    ChildCleanup(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to insert or get tv object: {0:?}
    GetOrInsertMedia(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to insert or get season: {0:?}
    GetOrInsertSeason(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to insert media object for episode: {0:?}
    GetOrInsertMediaEpisode(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to insert episode object: {0:?}
    GetOrInsertEpisode(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to get season id for episode: {0:?}
    GetSeasonId(#[serde(skip)] dim_database::DatabaseError),
    /// Failed to get tvshowid for season: {0:?}
    GetTvId(#[serde(skip)] dim_database::DatabaseError),
    /// Season not found
    SeasonNotFound,
    /// Episode not found
    EpisodeNotFound,
}

#[derive(Clone, Copy)]
pub struct TvMatcher;

impl TvMatcher {
    async fn match_to_result<'life0>(
        &self,
        tx: &mut Transaction<'life0>,
        file: MediaFile,
        result: (ExternalMedia, ExternalSeason, ExternalEpisode),
    ) -> Result<(i64, i64, i64), Error> {
        // TODO: insert poster and backdrops.
        let (emedia, eseason, eepisode) = result;

        let posters = emedia
            .posters
            .iter()
            .filter_map(|x| asset_from_url(x))
            .collect::<Vec<_>>();

        let mut poster_ids = vec![];

        for poster in posters {
            let asset = poster
                .insert(&mut *tx)
                .await
                .inspect_err(|error| error!(?error, "Failed to insert asset into db."))
                .map_err(Error::PosterInsert)?;

            poster_ids.push(asset);
        }

        let backdrops = emedia
            .backdrops
            .iter()
            .filter_map(|x| asset_from_url(x))
            .collect::<Vec<_>>();

        let mut backdrop_ids = vec![];

        for backdrop in backdrops {
            let asset = backdrop
                .insert(&mut *tx)
                .await
                .inspect_err(|error| error!(?error, "Failed to insert asset into db."))
                .map_err(Error::BackdropInsert)?;

            backdrop_ids.push(asset);
        }

        let media = InsertableMedia {
            media_type: MediaType::Tv,
            library_id: file.library_id,
            name: emedia.title,
            description: emedia.description,
            rating: emedia.rating,
            year: emedia.release_date.map(|x| x.year() as _),
            added: Utc::now().to_string(),
            poster: poster_ids.first().map(|x| x.id),
            backdrop: backdrop_ids.first().map(|x| x.id),
        };

        let parent_id = media
            .lazy_insert(tx)
            .await
            .inspect_err(|error| error!(?error, ?file, "Failed to lazy insert tv show"))
            .map_err(Error::GetOrInsertMedia)?;

        // NOTE: We want to decouple this media from all genres and essentially rebuild the list.
        // Its a lot simpler than doing a diff-update but it might be more expensive.
        Genre::decouple_all(tx, parent_id)
            .await
            .inspect_err(|error| error!(?error, "Failed to decouple genres from media."))
            .map_err(Error::GenreDecouple)?;

        for name in emedia.genres {
            let genre = InsertableGenre { name }
                .insert(tx)
                .await
                .inspect_err(|error| error!(?error, "Failed to create or get genre."))
                .map_err(Error::GetOrInsertGenre)?;

            // TODO: Recouple genres always otherwise rematching would get buggy genre lists
            InsertableGenreMedia::insert_pair(genre, parent_id, tx)
                .await
                .inspect_err(
                    |error| error!(?error, %parent_id, "Failed to attach genre to media object."),
                )
                .map_err(Error::CoupleGenre)?;
        }

        let seasonid = self.match_to_season(tx, parent_id, eseason).await?;
        let episodeid = self
            .match_to_episode(tx, file.clone(), seasonid, eepisode)
            .await?;

        // If the mediafile used to belong to a different episode/season/show we want to
        // recursively search if we need to delete the parents. If the parents have 0 children, we
        // want to erase their existance.
        match file.media_id {
            Some(x) if x != episodeid => {
                let season_id = Episode::get_seasonid(tx, x)
                    .await
                    .inspect_err(
                        |error| error!(?error, id = %x, "Failed to get seasonid for episode"),
                    )
                    .map_err(Error::GetSeasonId)?;

                let tvshow_id = Season::get_tvshowid(tx, season_id).await.inspect_err(
                    |error| error!(?error, id = %x, "Failed to get tvshowid for season/episode."),
                ).map_err(Error::GetTvId)?;

                let count = Movie::count_children(tx, x).await.inspect_err(
                    |error| error!(?error, id = %x, "Failed to obtain children count for episode."),
                ).map_err(Error::ChildrenCount)?;

                if count == 0 {
                    Media::delete(tx, x)
                        .await
                        .inspect_err(
                            |error| error!(?error, id = %x, "Failed to delete child-less episode"),
                        )
                        .map_err(Error::ChildCleanup)?;
                }

                let count = Season::count_children(tx, season_id)
                    .await
                    .inspect_err(
                        |error| error!(?error, id = %x, "Failed to get children count for season"),
                    )
                    .map_err(Error::ChildrenCount)?;

                if count == 0 {
                    Season::delete_by_id(tx, season_id)
                        .await
                        .inspect_err(
                            |error| error!(?error, id = %x, "Failed to delete child-less season"),
                        )
                        .map_err(Error::ChildCleanup)?;
                }

                let count = TVShow::count_children(tx, tvshow_id).await.inspect_err(
                    |error| error!(?error, id = %x, "Failed to get children count for tv show."),
                ).map_err(Error::ChildrenCount)?;

                if count == 0 {
                    Media::delete(tx, tvshow_id)
                        .await
                        .inspect_err(
                            |error| error!(?error, id = %x, "Failed to delete child-less tv show"),
                        )
                        .map_err(Error::ChildCleanup)?;
                }
            }
            _ => {}
        }

        Ok((parent_id, seasonid, episodeid))
    }

    // FIXME: In cases where we can match against a show but not find a specific season or episode,
    // we want to backfill the data as Season 0 or as an Extra.
    async fn match_to_season(
        &self,
        tx: &mut Transaction<'_>,
        parent_id: i64,
        result: ExternalSeason,
    ) -> Result<i64, Error> {
        let posters = result
            .posters
            .iter()
            .filter_map(|x| asset_from_url(x))
            .collect::<Vec<_>>();

        let mut poster_ids = vec![];

        for poster in posters {
            let asset = poster
                .insert(&mut *tx)
                .await
                .inspect_err(|error| error!(?error, "Failed to insert asset into db."))
                .map_err(Error::PosterInsert)?;

            poster_ids.push(asset);
        }

        let season = InsertableSeason {
            season_number: result.season_number as _,
            added: Utc::now().to_string(),
            poster: poster_ids.first().map(|x| x.id),
        };

        let season_id = season
            .insert(tx, parent_id)
            .await
            .inspect_err(|error| error!(?error, "Failed to insert season object."))
            .map_err(Error::GetOrInsertSeason)?;

        Ok(season_id)
    }

    async fn match_to_episode(
        &self,
        tx: &mut Transaction<'_>,
        file: MediaFile,
        seasonid: i64,
        result: ExternalEpisode,
    ) -> Result<i64, Error> {
        let stills = result
            .stills
            .iter()
            .filter_map(|x| asset_from_url(x))
            .collect::<Vec<_>>();

        let mut still_ids = vec![];

        for still in stills {
            let asset = still
                .insert(&mut *tx)
                .await
                .inspect_err(|error| error!(?error, "Failed to insert asset into db."))
                .map_err(Error::PosterInsert)?;

            still_ids.push(asset);
        }

        let media = InsertableMedia {
            library_id: file.library_id,
            name: result.title_or_episode(),
            added: Utc::now().to_string(),
            media_type: MediaType::Episode,
            description: result.description.clone(),
            backdrop: still_ids.first().map(|x| x.id),
            ..Default::default()
        };

        let episode = InsertableEpisode {
            episode: result.episode_number as _,
            seasonid,
            media,
        };

        episode
            .media
            .insert_blind(&mut *tx)
            .await
            .inspect_err(|error| error!(?error, ?file, "Failed to insert media for episode."))
            .map_err(Error::GetOrInsertMediaEpisode)?;

        // NOTE: WE use to turn a episode into a movie here, not sure if necessary anymore.

        let episode_id = episode
            .insert(&mut *tx)
            .await
            .inspect_err(|error| error!(?error, ?file, "Failed to insert episode."))
            .map_err(Error::GetOrInsertEpisode)?;

        let updated_mediafile = UpdateMediaFile {
            media_id: Some(episode_id),
            ..Default::default()
        };

        updated_mediafile
            .update(&mut *tx, file.id)
            .await
            .inspect_err(|error| error!(?error, ?file, "Failed to update mediafile media id."))
            .map_err(Error::UpdateMediafile)?;

        Ok(episode_id)
    }

    #[instrument(skip(provider, metadata))]
    async fn lookup_metadata(
        provider: Arc<dyn ExternalQueryShow>,
        file: MediaFile,
        metadata: Vec<Metadata>,
    ) -> Option<(MediaFile, (ExternalMedia, ExternalSeason, ExternalEpisode))> {
        for meta in metadata {
            match provider
                .search(meta.name.as_ref(), meta.year.map(|x| x as _))
                .await
            {
                Ok(provided) => {
                    let first = if let Some(x) = provided.first() {
                        x.clone()
                    } else {
                        continue;
                    };

                    let Ok(seasons) = provider.seasons_for_id(&first.external_id).await else {
                        info!(?meta, "Failed to find season match with the current metadata set.");
                        continue;
                    };

                    // FIXME: If a file doesnt have season metadata, we want to default to
                    // marking this file as an extra and put it in season 0
                    let Some(season) = seasons
                        .into_iter()
                        .find(|x| x.season_number as i64 == meta.season.unwrap_or(0)) else {
                        info!(?meta, "Provider didnt return our desired season with current metadata.");
                        continue;
                    };

                    let Ok(episodes) = provider
                        .episodes_for_season(&first.external_id, meta.season.unwrap_or(0) as _)
                        .await else {
                        // FIXME: We might want to propagate this error.
                        info!(?meta, "Failed to fetch episodes with current metadata set.");
                        continue;
                    };

                    let Some(episode) = episodes
                        .into_iter()
                        .find(|x| x.episode_number as i64 == meta.episode.unwrap_or(0)) else {
                        info!(
                            ?meta,
                            "Provider didnt return our desired episode with current metadata."
                        );
                        continue;
                    };

                    return Some((file, (first, season, episode)));
                }
                Err(e) => error!(?meta, error = ?e, "Failed to find a movie match."),
            }
        }

        None
    }
}

#[async_trait]
impl MediaMatcher for TvMatcher {
    async fn batch_match(
        &self,
        tx: &mut Transaction<'_>,
        provider: Arc<dyn ExternalQueryIntoShow>,
        work: Vec<WorkUnit>,
    ) -> Result<(), super::Error> {
        let provider_show: Arc<dyn ExternalQueryShow> = provider
            .into_query_show()
            .expect("Scanner needs a show provider");

        let metadata_futs = work
            .into_iter()
            .map(|WorkUnit(file, metadata)| {
                let provider_show = Arc::clone(&provider_show);
                tokio::spawn(Self::lookup_metadata(provider_show, file, metadata))
            })
            .collect::<Vec<_>>();

        let metadata = futures::future::join_all(metadata_futs).await;

        for meta in metadata.into_iter() {
            if let Ok(Some((file, provided))) = meta {
                self.match_to_result(tx, file, provided)
                    .await
                    .inspect_err(|error| error!(?error, "failed to match to result"))?;
            }
        }

        Ok(())
    }

    async fn match_to_id(
        &self,
        tx: &mut Transaction<'_>,
        provider: Arc<dyn ExternalQueryIntoShow>,
        work: WorkUnit,
        external_id: &str,
    ) -> Result<(), super::Error> {
        let provider: Arc<dyn ExternalQueryShow> = provider
            .into_query_show()
            .expect("Scanner needs a show provider");

        let WorkUnit(file, metadata) = work;

        let provided = match provider.search_by_id(external_id).await {
            Ok(provided) => provided,
            Err(e) => {
                error!(%external_id, error = ?e, "Failed to find a movie match.");
                return Err(super::Error::InvalidExternalId);
            }
        };

        let mut season_result = None;
        let mut episode_result = None;

        for meta in metadata {
            let Ok(seasons) = provider.seasons_for_id(external_id).await else {
                info!(?meta, "Failed to find season match with the current metadata set.");
                continue;
            };

            // FIXME: If a file doesnt have season metadata, we want to default to
            // marking this file as an extra and put it in season 0
            let Some(season) = seasons
                .into_iter()
                .find(|x| x.season_number as i64 == meta.season.unwrap_or(0)) else {
                    info!(?meta, "Provider didnt return our desired season with current metadata.");
                    continue;
                };

            let Ok(episodes) = provider
                .episodes_for_season(external_id, meta.season.unwrap_or(0) as _)
                .await else {
                    // FIXME: We might want to propagate this error.
                    info!(?meta, "Failed to fetch episodes with current metadata set.");
                    continue;
                };

            let Some(episode) = episodes
                .into_iter()
                .find(|x| x.episode_number as i64 == meta.episode.unwrap_or(0)) else {
                    info!(
                        ?meta,
                        "Provider didnt return our desired episode with current metadata."
                    );
                    continue
                };

            season_result = Some(season);
            episode_result = Some(episode);
        }

        let Some(season_result) = season_result else { return Err(Error::SeasonNotFound.into()); };
        let Some(episode_result) = episode_result else { return Err(Error::EpisodeNotFound.into()); };

        self.match_to_result(tx, file, (provided, season_result, episode_result))
            .await
            .inspect_err(|error| error!(?error, "failed to match to result"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::mediafile::create_library;
    use super::TvMatcher;

    use crate::external::ExternalEpisode;
    use crate::external::ExternalMedia;
    use crate::external::ExternalSeason;

    use dim_database::episode::Episode;
    use dim_database::media::Media;
    use dim_database::mediafile::InsertableMediaFile;
    use dim_database::mediafile::MediaFile;
    use dim_database::rw_pool::write_tx;
    use dim_database::season::Season;
    use dim_database::tv::TVShow;

    #[tokio::test(flavor = "multi_thread")]
    async fn match_show() {
        const MATCHER: TvMatcher = TvMatcher;

        let mut conn = dim_database::get_conn_memory()
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

        let mfile_id = mediafile.insert(&mut tx).await.unwrap();

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
        let (m1, s1, e1) = MATCHER
            .match_to_result(&mut tx, mfile, result.clone())
            .await
            .unwrap();

        mediafile.target_file = "test2.mp4".into();
        let mfile2_id = mediafile.insert(&mut tx).await.unwrap();

        let mfile2 = MediaFile::get_one(&mut tx, mfile2_id).await.unwrap();
        let (m2, s2, e2) = MATCHER
            .match_to_result(&mut tx, mfile2, result.clone())
            .await
            .unwrap();

        // We attach two mediafiles to the same episode, season and show
        assert_eq!(m1, m2);
        assert_eq!(s1, s2);
        assert_eq!(e1, e2);

        mediafile.target_file = "test3.mp4".into();
        result.2.episode_number = 2;
        let mfile3_id = mediafile.insert(&mut tx).await.unwrap();

        let mfile3 = MediaFile::get_one(&mut tx, mfile3_id).await.unwrap();
        let (m3, s3, e3) = MATCHER
            .match_to_result(&mut tx, mfile3, result.clone())
            .await
            .unwrap();

        // we attach a third mediafile to the same show and season but different episode
        assert_eq!(m2, m3);
        assert_eq!(s2, s3);
        assert_ne!(e2, e3);

        mediafile.target_file = "test4.mp4".into();
        result.1.season_number = 2;
        let mfile4_id = mediafile.insert(&mut tx).await.unwrap();

        let mfile4 = MediaFile::get_one(&mut tx, mfile4_id).await.unwrap();
        let (m4, s4, e4) = MATCHER
            .match_to_result(&mut tx, mfile4, result.clone())
            .await
            .unwrap();

        // we attach a fourth mediafile to a different season and episode but same show
        assert_eq!(m4, m3);
        assert_ne!(s4, s3);
        assert_ne!(e4, e3);

        // we should have two seasons matched
        let seasons = Season::get_all(&mut tx, m4).await.unwrap();
        assert_eq!(seasons.len(), 2);

        // First season should have two episodes.
        let episodes = Episode::get_all_of_season(&mut tx, s2).await.unwrap();
        assert_eq!(episodes.len(), 2);

        // Last season should have one episode
        let episodes = Episode::get_all_of_season(&mut tx, s4).await.unwrap();
        assert_eq!(episodes.len(), 1);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn rematch_episode() {
        crate::setup_test_logging();
        const MATCHER: TvMatcher = TvMatcher;

        let mut conn = dim_database::get_conn_memory()
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

        let mfile_id = mediafile.insert(&mut tx).await.unwrap();

        mediafile.target_file = "test1.mp4".into();
        let mfile2_id = mediafile.insert(&mut tx).await.unwrap();

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
        let mfile2 = MediaFile::get_one(&mut tx, mfile2_id).await.unwrap();

        let (t1, s1, _e1) = MATCHER
            .match_to_result(&mut tx, mfile.clone(), result.clone())
            .await
            .unwrap();

        MATCHER
            .match_to_result(&mut tx, mfile2.clone(), result.clone())
            .await
            .unwrap();

        let mfile = MediaFile::get_one(&mut tx, mfile_id).await.unwrap();
        let mfile2 = MediaFile::get_one(&mut tx, mfile2_id).await.unwrap();

        // Rematch mfile 1 to a different show
        result.1.season_number = 1;
        result.0.title = "Show 2".into();

        let _seasons = Season::get_all(&mut tx, t1).await.unwrap();

        let (t2, s2, _e2) = MATCHER
            .match_to_result(&mut tx, mfile, result.clone())
            .await
            .unwrap();

        // Season 1 of t1 should have only one episode at this point.
        let episodes = Episode::get_all_of_season(&mut tx, s1).await.unwrap();
        assert_eq!(episodes.len(), 1);

        result.2.episode_number = 2;
        MATCHER
            .match_to_result(&mut tx, mfile2, result.clone())
            .await
            .unwrap();

        // Season 1 of t1 should not exist now because no episodes are linked to it anymore
        let res = Season::get_by_id(&mut tx, s1).await;
        assert!(res.is_err());

        // Tv show should not exist anymore because no seasons are linked to it.
        let res = Media::get(&mut tx, t1).await;
        assert!(res.is_err());

        // New show should exist and have one season with two episodes.
        let count = TVShow::count_children(&mut tx, t2).await.unwrap();
        assert_eq!(count, 1);

        let episodes = Episode::get_all_of_season(&mut tx, s2).await.unwrap();
        assert_eq!(episodes.len(), 2);
    }
}
