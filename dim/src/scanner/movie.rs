#![allow(unstable_name_collisions)]

use crate::external::ExternalMedia;
use crate::external::ExternalQuery;
use crate::inspect::ResultExt;

use super::MediaMatcher;
use super::WorkUnit;

use async_trait::async_trait;
use chrono::prelude::Utc;
use chrono::Datelike;

use database::genre::Genre;
use database::genre::InsertableGenre;
use database::genre::InsertableGenreMedia;
use database::library::MediaType;
use database::media::InsertableMedia;
use database::media::Media;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;
use database::movie::Movie;
use database::Transaction;

use std::sync::Arc;
use tracing::error;

pub struct MovieMatcher;

impl MovieMatcher {
    /// Method will match a mediafile to a new media. Caller must ensure that the mediafile
    /// supplied is not coupled to a media object. If it is coupled we will assume that we can
    /// replace the metadata supplied to it.
    #[tracing::instrument(skip(self, tx))]
    async fn match_to_result<'life0>(
        &self,
        tx: &mut Transaction<'life0>,
        file: MediaFile,
        provided: ExternalMedia,
    ) -> Result<i64, Box<dyn std::error::Error>> {
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
        let media_id = if let Some(media_id) = file.media_id {
            // NOTE: We need to be careful here for this to work correctly when our parent media
            // is linked to multiple mediafiles.
            // FIXME: In theory this call should be very cheap, parents are unlikely have too many
            // entries but we should double check.
            let count = Movie::count_children(tx, media_id).await.inspect_err(
                |error| error!(?error, ?file.media_id, "Failed to get media children count."),
            )?;

            // If we are the only child we can just in-place modify the media object safely.
            if count == 1 {
                media
                    .insert_with_id(tx, media_id)
                    .await
                    .inspect_err(|error| {
                        error!(?error, ?media_id, "Failed to replace parent media entry.")
                    })?;

                Some(media_id)
            } else {
                None
            }
        } else {
            None
        };

        let media_id = if let Some(media_id) = media_id {
            media_id
        } else {
            // Maybe a media object that can be linked against this file already exists and we want
            // to bind to it?
            match Media::get_id_by_name(tx, &media.name)
                .await
                .inspect_err(|error| error!(?error, %media.name, "Failed to get a media by name"))?
            {
                Some(id) => id,
                None => media
                    .insert(tx)
                    .await
                    .inspect_err(|error| error!(?error, "Failed to insert media object."))?,
            }
        };

        // NOTE: We want to decouple this media from all genres and essentially rebuild the list.
        // Its a lot simpler than doing a diff-update but it might be more expensive.
        Genre::decouple_all(tx, media_id)
            .await
            .inspect_err(|error| error!(?error, "Failed to decouple genres from media."))?;

        for name in provided.genres {
            let genre = InsertableGenre { name }
                .insert(tx)
                .await
                .inspect_err(|error| error!(?error, "Failed to create or get genre."))?;

            // TODO: Recouple genres always otherwise rematching would get buggy genre lists
            InsertableGenreMedia::insert_pair(genre, media_id, tx)
                .await
                .inspect_err(
                    |error| error!(?error, %media_id, "Failed to attach genre to media object."),
                )?;
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
        .inspect_err(|error| {
            error!(?error, "Failed to update mediafile to point to new parent.")
        })?;

        Ok(media_id)
    }
}

#[async_trait]
impl MediaMatcher for MovieMatcher {
    async fn batch_match(self: Arc<Self>, provider: Arc<dyn ExternalQuery>, work: Vec<WorkUnit>) {
        let metadata_futs = work
            .into_iter()
            .map(|WorkUnit(file, metadata)| async {
                for meta in metadata {
                    match provider
                        .search(meta.name.as_ref(), meta.year.map(|x| x as _))
                        .await
                    {
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

#[cfg(test)]
mod tests {
    use super::super::tests::mediafile::create_library;

    use super::MovieMatcher;
    use crate::external::ExternalMedia;

    use chrono::TimeZone;
    use database::genre::Genre;
    use database::media::Media;
    use database::mediafile::InsertableMediaFile;
    use database::mediafile::MediaFile;
    use database::movie::Movie;
    use database::rw_pool::write_tx;

    #[tokio::test(flavor = "multi_thread")]
    async fn match_to_movie() {
        let mut conn = database::get_conn_memory()
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

        // we are essentially rematching a mediafile and since we are sure our parent has only one
        // child we can simply replace the parent
        assert_eq!(media_id, updated_media);

        let media_obj = Media::get(&mut tx, media_id).await.unwrap();
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
        let mut conn = database::get_conn_memory()
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
}
