use super::super::mediafile::Error as CreatorError;
use super::super::mediafile::InsertBatch;
use super::super::mediafile::MediafileCreator;
use super::super::parse_filenames;

use dim_database::library::InsertableLibrary;
use dim_database::library::MediaType;
use dim_database::mediafile::InsertableMediaFile;
use dim_database::mediafile::MediaFile;

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use itertools::Itertools;

use std::future::Future;

use core::pin::Pin;

use futures::FutureExt;

use new_xtra::spawn::Tokio;
use new_xtra::Actor;

pub(crate) async fn create_library(conn: &mut dim_database::DbConnection) -> i64 {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.unwrap();

    let id = InsertableLibrary {
        name: "Tests".to_string(),
        locations: vec![],
        media_type: MediaType::Movie,
    }
    .insert(&mut tx)
    .await
    .expect("Failed to create test library.");

    tx.commit().await.expect("Failed to commit test library.");

    id
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_mediafile() {
    let files = (0..512)
        .map(|i| format!("Movie{i}.mkv"))
        .collect::<Vec<String>>();
    let (_tempdir, files) = super::temp_dir_symlink(files.into_iter(), super::TEST_MP4_PATH);

    let mut conn = dim_database::get_conn_memory()
        .await
        .expect("Failed to obtain a in-memory db pool.");
    let library = create_library(&mut conn).await;

    let mut instance = MediafileCreator::new(conn.clone(), library).await;

    let parsed = parse_filenames(files.iter());

    assert_eq!(parsed.len(), files.len());

    let insertable_futures =
        parsed
            .into_iter()
            .map(|(path, meta)| instance.construct_mediafile(path, meta[0].clone()).boxed())
            .chunks(5)
            .into_iter()
            .map(|chunk| chunk.collect())
            .collect::<Vec<
                Vec<
                    Pin<Box<dyn Future<Output = Result<InsertableMediaFile, CreatorError>> + Send>>,
                >,
            >>();

    let mut insertables = vec![];

    for chunk in insertable_futures.into_iter() {
        let results: Vec<Result<InsertableMediaFile, CreatorError>> =
            futures::future::join_all(chunk).await;

        for result in results {
            insertables.push(result.expect("Failed to create insertable."));
        }
    }

    let mut mediafiles = vec![];

    for chunk in insertables.chunks(128) {
        mediafiles.append(
            &mut instance
                .insert_batch(chunk.iter())
                .await
                .expect("Failed to insert batch."),
        );
    }

    // We should have inserted all the files as they dont exist in the database.
    assert_eq!(mediafiles.len(), files.len());

    // All the files in `insertables` should already exist in the database, thus this should return
    // `0`.
    for chunk in insertables.chunks(128) {
        let files = instance
            .insert_batch(chunk.iter())
            .await
            .expect("Failed to insert batch.");

        assert_eq!(files.len(), 0);
    }

    // At this point we should have 512 files in the database.
    let mut tx = conn.read().begin().await.unwrap();
    let files_in_db = MediaFile::get_by_lib_null_media(&mut tx, library)
        .await
        .expect("Failed to get mediafiles.");
    assert_eq!(files_in_db.len(), files.len());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_multiple_instances() {
    let files = (0..1024)
        .map(|i| format!("Movie{i}.mkv"))
        .collect::<Vec<String>>();
    let (_tempdir, files) = super::temp_dir_symlink(files.into_iter(), super::TEST_MP4_PATH);

    let mut conn = dim_database::get_conn_memory()
        .await
        .expect("Failed to obtain a in-memory db pool.");
    let library = create_library(&mut conn).await;

    let instance = MediafileCreator::new(conn.clone(), library).await;

    let parsed = parse_filenames(files.iter());

    assert_eq!(parsed.len(), files.len());

    let mut insertables = vec![];

    for mut chunk in parsed
        .into_iter()
        .map(|(path, meta)| instance.construct_mediafile(path, meta[0].clone()))
        .chunks(16)
        .into_iter()
        .map(|ch| ch.collect::<FuturesUnordered<_>>())
    {
        while let Some(res) = chunk.next().await {
            insertables.push(res.expect("Failed to create insertable."));
        }
    }

    let mut instances = vec![];

    for _ in 0..8 {
        let addr = MediafileCreator::new(conn.clone(), library)
            .await
            .create(None)
            .spawn(&mut Tokio::Global);
        instances.push(addr);
    }

    let mut insert_futures = vec![];

    for (chunk, addr) in insertables.chunks(128).zip(instances.iter().cycle()) {
        let addr = addr.clone();
        insert_futures.push(async move {
            let chunk_len = chunk.len();
            let result = addr
                .send(InsertBatch(chunk.into_iter().cloned().collect()))
                .await
                .expect("Addr got dropped")
                .expect("Failed to insert batch");

            assert_eq!(result.len(), chunk_len);

            result
        });
    }

    let mediafiles = futures::future::join_all(insert_futures)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    assert_eq!(mediafiles.len(), files.len());
}
