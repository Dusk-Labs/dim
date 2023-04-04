#![allow(unstable_name_collisions)]

use crate::external::ExternalMedia;
use crate::external::ExternalQueryIntoShow;
use crate::inspect::ResultExt;
use crate::scanner::format_path;

use super::MediaMatcher;
use super::WorkUnit;

use async_trait::async_trait;
use chrono::prelude::Utc;
use chrono::Datelike;

use dim_database::asset::InsertableAsset;
use dim_database::genre::Genre;
use dim_database::genre::InsertableGenre;
use dim_database::genre::InsertableGenreMedia;
use dim_database::library::MediaType;
use dim_database::media::InsertableMedia;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use dim_database::mediafile::UpdateMediaFile;
use dim_database::movie::Movie;
use dim_database::Transaction;

use serde::Serialize;
use std::sync::Arc;
use tracing::error;
use tracing::warn;

use url::Url;

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
    /// Failed to insert or get media object: {0:?}
    GetOrInsertMedia(#[serde(skip)] dim_database::DatabaseError),
}

pub fn asset_from_url(url: &str) -> Option<InsertableAsset> {
    let url = Url::parse(url).ok()?;
    let filename = uuid::Uuid::new_v4().as_hyphenated().to_string();
    let local_path = format_path(Some(format!("{filename}.jpg")));

    Some(InsertableAsset {
        remote_url: Some(url.into()),
        local_path,
        file_ext: "jpg".into(),
    })
}

#[derive(Clone, Copy)]
pub struct MovieMatcher;

impl MovieMatcher {
    /// Method will match a mediafile to a new media. Caller must ensure that the mediafile supplied is not coupled to a media object. If it is coupled we will assume that we can
    /// replace the metadata supplied to it.
    #[tracing::instrument(skip(self, tx))]
    async fn match_to_result<'life0>(
        &self,
        tx: &mut Transaction<'life0>,
        file: MediaFile,
        provided: ExternalMedia,
    ) -> Result<i64, Error> {
        // TODO: Push posters and backdrops to download queue. Push CDC events.
        let posters = provided
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

        let backdrops = provided
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
            media_type: MediaType::Movie,
            library_id: file.library_id,
            name: provided.title,
            description: provided.description,
            rating: provided.rating,
            year: provided.release_date.map(|x| x.year() as _),
            added: Utc::now().to_string(),
            poster: poster_ids.first().map(|x| x.id),
            backdrop: backdrop_ids.first().map(|x| x.id),
        };

        let media_id = media
            .lazy_insert(tx)
            .await
            .map_err(Error::GetOrInsertMedia)?;

        // Link all backdrops and posters to our media.
        for poster in poster_ids {
            let _ = poster
                .into_media_poster(tx, media_id)
                .await
                .inspect_err(|error| warn!(?error, "Failed to link poster to media."));
        }

        for backdrop in backdrop_ids {
            let _ = backdrop
                .into_media_backdrop(tx, media_id)
                .await
                .inspect_err(|error| warn!(?error, "Failed to link backdrop."));

            // TODO: Queuing
        }

        // NOTE: We want to decouple this media from all genres and essentially rebuild the list.
        // Its a lot simpler than doing a diff-update but it might be more expensive.
        Genre::decouple_all(tx, media_id)
            .await
            .inspect_err(|error| error!(?error, "Failed to decouple genres from media."))
            .map_err(Error::GenreDecouple)?;

        for name in provided.genres {
            let genre = InsertableGenre { name }
                .insert(tx)
                .await
                .inspect_err(|error| error!(?error, "Failed to create or get genre."))
                .map_err(Error::GetOrInsertGenre)?;

            // TODO: Recouple genres always otherwise rematching would get buggy genre lists
            InsertableGenreMedia::insert_pair(genre, media_id, tx)
                .await
                .inspect_err(
                    |error| error!(?error, %media_id, "Failed to attach genre to media object."),
                )
                .map_err(Error::CoupleGenre)?;
        }

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
        .inspect_err(|error| error!(?error, "Failed to update mediafile to point to new parent."))
        .map_err(Error::UpdateMediafile)?;

        // Sometimes we rematch against a media object that already exists but we are the last
        // child for the parent. When this happens we want to cleanup.
        match file.media_id {
            Some(old_id) => {
                let count = Movie::count_children(tx, old_id)
                    .await
                    .inspect_err(|error| error!(?error, %old_id, "Failed to grab children count."))
                    .map_err(Error::ChildrenCount)?;

                if count == 0 {
                    Media::delete(tx, old_id)
                        .await
                        .inspect_err(
                            |error| error!(?error, %old_id, "Failed to cleanup child-less parent."),
                        )
                        .map_err(Error::ChildCleanup)?;
                }
            }
            _ => {}
        }

        Ok(media_id)
    }
}

#[async_trait]
impl MediaMatcher for MovieMatcher {
    async fn batch_match(
        &self,
        tx: &mut Transaction<'_>,
        provider: Arc<dyn ExternalQueryIntoShow>,
        work: Vec<WorkUnit>,
    ) -> Result<(), super::Error> {
        let metadata_futs = work
            .into_iter()
            .map(|WorkUnit(file, metadata)| async {
                for meta in metadata {
                    match provider
                        .search(meta.name.as_ref(), meta.year.map(|x| x as _))
                        .await
                    {
                        Ok(provided) => return Some((file, provided)),
                        Err(e) => error!(?meta, error = ?e, "Failed to find a movie match."),
                    }
                }

                None
            })
            .collect::<Vec<_>>();

        let metadata = futures::future::join_all(metadata_futs).await;

        // FIXME: Propagate errors.
        for meta in metadata.into_iter() {
            if let Some((file, provided)) = meta {
                if let Some(provided) = provided.first() {
                    self.match_to_result(tx, file, provided.clone())
                        .await
                        .inspect_err(|error| error!(?error, "failed to match to result"))?;
                }
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
        let WorkUnit(file, _) = work;

        let provided = match provider.search_by_id(external_id).await {
            Ok(provided) => provided,
            Err(e) => {
                error!(%external_id, error = ?e, "Failed to find a movie match.");
                return Err(super::Error::InvalidExternalId);
            }
        };

        self.match_to_result(tx, file, provided)
            .await
            .inspect_err(|error| error!(?error, "failed to match file to external id."))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::mediafile::create_library;

    use super::MovieMatcher;
    use crate::external::ExternalMedia;

    use chrono::TimeZone;
    use dim_database::genre::Genre;
    use dim_database::media::Media;
    use dim_database::mediafile::InsertableMediaFile;
    use dim_database::mediafile::MediaFile;
    use dim_database::movie::Movie;
    use dim_database::rw_pool::write_tx;

    #[tokio::test(flavor = "multi_thread")]
    async fn match_to_movie() {
        let mut conn = dim_database::get_conn_memory()
            .await
            .expect("Failed to obtain a in-memory db pool.");
        let library = create_library(&mut conn).await;

        let mut lock = conn.writer.lock_owned().await;
        let mut tx = write_tx(&mut lock).await.unwrap();

        let mediafile = InsertableMediaFile {
            library_id: library,
            target_file: "test.mp4".into(),
            raw_name: "test".into(),
            ..Default::default()
        }
        .insert(&mut tx)
        .await
        .unwrap();

        let mfile = MediaFile::get_one(&mut tx, mediafile).await.unwrap();

        // no media should be linked to the mfile at this point
        assert_eq!(mfile.media_id, None);

        let dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Test Title".into(),
            description: Some("test description".into()),
            release_date: Some(chrono::Utc.ymd(1983, 1, 10).and_hms(0, 0, 0)),
            posters: vec![],
            backdrops: vec![],
            genres: vec!["Comedy".into()],
            rating: Some(0.0),
            duration: None,
        };

        const MATCHER: MovieMatcher = MovieMatcher;

        let media_id = MATCHER
            .match_to_result(&mut tx, mfile, dummy_external)
            .await
            .unwrap();

        let mfile = MediaFile::get_one(&mut tx, mediafile).await.unwrap();

        // mediafile should now be linked to a media object
        assert_eq!(mfile.media_id, Some(media_id));

        let media_obj = Media::get(&mut tx, media_id).await.unwrap();
        assert_eq!(media_obj.name, "Test Title".to_string());

        // Genre should be Comedy at this point.
        let genres = Genre::get_by_media(&mut tx, media_id).await.unwrap();
        assert_eq!(genres[0].name, "Comedy".to_string());

        let dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Other title".into(),
            description: None,
            genres: vec!["Anime".into()],
            ..Default::default()
        };

        let updated_media = MATCHER
            .match_to_result(&mut tx, mfile, dummy_external.clone())
            .await
            .unwrap();

        // in-place replacement doesnt exist, so we should get a new media id here.
        assert_ne!(media_id, updated_media);

        // the old object should be automatically erased.
        assert!(Media::get(&mut tx, media_id).await.is_err());

        let media_obj = Media::get(&mut tx, updated_media).await.unwrap();
        assert_eq!(media_obj.name, "Other title".to_string());
        assert_eq!(media_obj.description, None);

        // insert a new mediafile and link it to the same media object.
        let mfile2_id = InsertableMediaFile {
            library_id: library,
            target_file: "test2.mp4".into(),
            raw_name: "test".into(),
            ..Default::default()
        }
        .insert(&mut tx)
        .await
        .unwrap();

        let mfile2 = MediaFile::get_one(&mut tx, mfile2_id).await.unwrap();

        let mfile2_mediaid = MATCHER
            .match_to_result(&mut tx, mfile2, dummy_external)
            .await
            .unwrap();

        // the new mediafile should point to the same parent as the previous mediafile.
        assert_eq!(mfile2_mediaid, updated_media);

        let children_cnt = Movie::count_children(&mut tx, mfile2_mediaid)
            .await
            .unwrap();

        assert_eq!(children_cnt, 2);

        // now that the parent has multiple children, rematching should trigger the creation of a
        // new media and trigger a decoupling.
        let mfile = MediaFile::get_one(&mut tx, mediafile).await.unwrap();
        let dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Other other title".into(),
            description: None,
            genres: vec!["Anime".into()],
            ..Default::default()
        };

        let updated_media = MATCHER
            .match_to_result(&mut tx, mfile, dummy_external)
            .await
            .unwrap();

        assert_ne!(updated_media, mfile2_mediaid);

        let media_obj = Media::get(&mut tx, updated_media).await.unwrap();
        assert_eq!(media_obj.name, "Other other title".to_string());
        assert_eq!(media_obj.description, None);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn rematch_new_genres() {
        let mut conn = dim_database::get_conn_memory()
            .await
            .expect("Failed to obtain a in-memory db pool.");
        let library = create_library(&mut conn).await;

        let mut lock = conn.writer.lock_owned().await;
        let mut tx = write_tx(&mut lock).await.unwrap();

        const MATCHER: MovieMatcher = MovieMatcher;

        let mfile_id = InsertableMediaFile {
            library_id: library,
            target_file: "test.mp4".into(),
            raw_name: "test".into(),
            ..Default::default()
        }
        .insert(&mut tx)
        .await
        .unwrap();

        let mfile = MediaFile::get_one(&mut tx, mfile_id).await.unwrap();

        let dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Title".into(),
            description: None,
            genres: vec!["Anime".into()],
            ..Default::default()
        };

        let media = MATCHER
            .match_to_result(&mut tx, mfile.clone(), dummy_external)
            .await
            .unwrap();

        let genres = Genre::get_by_media(&mut tx, media).await.unwrap();

        // Only linked to genre Anime
        assert_eq!(genres.len(), 1);
        assert_eq!(genres[0].name, "Anime".to_string());

        let dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Title".into(),
            description: None,
            genres: vec!["Comedy".into(), "Adventure".into()],
            ..Default::default()
        };

        MATCHER
            .match_to_result(&mut tx, mfile, dummy_external)
            .await
            .unwrap();

        let genres = Genre::get_by_media(&mut tx, media).await.unwrap();

        // Now linked to two genres
        assert_eq!(genres.len(), 2);
        assert_eq!(genres[0].name, "Comedy".to_string());
        assert_eq!(genres[1].name, "Adventure".to_string());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn mass_rematch() {
        let mut conn = dim_database::get_conn_memory()
            .await
            .expect("Failed to obtain a in-memory db pool.");
        let library = create_library(&mut conn).await;

        let mut lock = conn.writer.lock_owned().await;
        let mut tx = write_tx(&mut lock).await.unwrap();

        const MATCHER: MovieMatcher = MovieMatcher;

        let mfile_id = InsertableMediaFile {
            library_id: library,
            target_file: "test.mp4".into(),
            raw_name: "test".into(),
            ..Default::default()
        }
        .insert(&mut tx)
        .await
        .unwrap();

        let mfile = MediaFile::get_one(&mut tx, mfile_id).await.unwrap();

        let mfile2_id = InsertableMediaFile {
            library_id: library,
            target_file: "test1.mp4".into(),
            raw_name: "test".into(),
            ..Default::default()
        }
        .insert(&mut tx)
        .await
        .unwrap();

        let mfile2 = MediaFile::get_one(&mut tx, mfile2_id).await.unwrap();

        // link two files to the same media.
        let dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Test Title".into(),
            description: Some("test description".into()),
            release_date: Some(chrono::Utc.ymd(1983, 1, 10).and_hms(0, 0, 0)),
            posters: vec![],
            backdrops: vec![],
            genres: vec!["Comedy".into()],
            rating: Some(0.0),
            duration: None,
        };

        let media_id = MATCHER
            .match_to_result(&mut tx, mfile.clone(), dummy_external.clone())
            .await
            .unwrap();

        let media_id2 = MATCHER
            .match_to_result(&mut tx, mfile2.clone(), dummy_external.clone())
            .await
            .unwrap();

        assert_eq!(media_id, media_id2);
        assert!(Movie::count_children(&mut tx, media_id).await.unwrap() == 2);

        let dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Other title".into(),
            description: None,
            genres: vec!["Anime".into()],
            ..Default::default()
        };

        let mfile = MediaFile::get_one(&mut tx, mfile_id).await.unwrap();

        // Match one child file from first media to a new one.
        let updated_media = MATCHER
            .match_to_result(&mut tx, mfile, dummy_external.clone())
            .await
            .unwrap();

        // we should now have two medias
        assert!(Media::get_all(&mut tx, library).await.unwrap().len() == 2);

        // now match second file, after this point we need to have only one media object in the
        // database.
        let mfile2 = MediaFile::get_one(&mut tx, mfile2_id).await.unwrap();

        let updated_media2 = MATCHER
            .match_to_result(&mut tx, mfile2, dummy_external.clone())
            .await
            .unwrap();

        assert!(updated_media == updated_media2);

        // we should now have one media
        assert_eq!(Media::get_all(&mut tx, library).await.unwrap().len(), 1);
    }

    /// Test refreshes metadata in-place for some media object. Refreshing only works if the new
    /// metadata has the same title and mediatype as the previous metadata. Otherwise it is
    /// rematched which will change the id of the object.
    #[tokio::test(flavor = "multi_thread")]
    async fn refresh_metadata() {
        let mut conn = dim_database::get_conn_memory()
            .await
            .expect("Failed to obtain a in-memory db pool.");
        let library = create_library(&mut conn).await;

        let mut lock = conn.writer.lock_owned().await;
        let mut tx = write_tx(&mut lock).await.unwrap();

        const MATCHER: MovieMatcher = MovieMatcher;

        let mfile_id = InsertableMediaFile {
            library_id: library,
            target_file: "test.mp4".into(),
            raw_name: "test".into(),
            ..Default::default()
        }
        .insert(&mut tx)
        .await
        .unwrap();

        let mfile = MediaFile::get_one(&mut tx, mfile_id).await.unwrap();

        // link two files to the same media.
        let mut dummy_external = ExternalMedia {
            external_id: "123".into(),
            title: "Test Title".into(),
            description: Some("test description".into()),
            release_date: Some(chrono::Utc.ymd(1983, 1, 10).and_hms(0, 0, 0)),
            posters: vec![],
            backdrops: vec![],
            genres: vec!["Comedy".into()],
            rating: Some(0.0),
            duration: None,
        };

        let media_id = MATCHER
            .match_to_result(&mut tx, mfile.clone(), dummy_external.clone())
            .await
            .unwrap();

        dummy_external.description = Some("new description".into());
        dummy_external.rating = Some(10.0);

        let refreshed_id = MATCHER
            .match_to_result(&mut tx, mfile.clone(), dummy_external.clone())
            .await
            .unwrap();

        // refreshing means that we just fetch updated metadata but dont rematch as its the same
        // media technically. as such the ids here should remain equal
        assert_eq!(media_id, refreshed_id);

        let media = Media::get(&mut tx, refreshed_id).await.unwrap();

        // we should see the new rating and description here
        assert_eq!(media.description, Some("new description".into()));
        assert_eq!(media.rating, Some(10.0));
    }
}
