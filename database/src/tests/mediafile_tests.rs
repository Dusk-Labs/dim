use crate::get_conn_memory;
use crate::mediafile;
use crate::write_tx;

use super::library_tests::create_test_library;

pub async fn insert_mediafile(conn: &mut crate::Transaction<'_>) -> i64 {
    let mfile = mediafile::InsertableMediaFile {
        library_id: 1,
        target_file: "/dev/null".into(),
        raw_name: "Test".into(),
        ..Default::default()
    };

    mfile.insert(&mut *conn).await.unwrap()
}

pub async fn insert_mediafile_with_mediaid(
    conn: &mut crate::Transaction<'_>,
    media_id: i64,
) -> i64 {
    let mfile = mediafile::InsertableMediaFile {
        library_id: 1,
        target_file: "/dev/null".into(),
        raw_name: "Test".into(),
        media_id: Some(media_id),
        ..Default::default()
    };

    mfile.insert(&mut *conn).await.unwrap()
}

pub async fn insert_many_mediafile(conn: &mut crate::Transaction<'_>, n: usize) {
    for i in 0..n {
        let mfile = mediafile::InsertableMediaFile {
            library_id: 1,
            target_file: format!("/dev/null/{}", i),
            raw_name: "Test".into(),
            ..Default::default()
        };

        mfile.insert(&mut *conn).await.unwrap();
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_lib() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = create_test_library(&mut tx).await;

    let result = mediafile::MediaFile::get_by_lib(&mut tx, id).await.unwrap();
    assert!(result.is_empty());

    let _mfile_id = insert_mediafile(&mut tx).await;

    let result = mediafile::MediaFile::get_by_lib(&mut tx, id).await.unwrap();
    assert_eq!(result.len(), 1);

    insert_many_mediafile(&mut tx, 9).await;

    let result = mediafile::MediaFile::get_by_lib(&mut tx, id).await.unwrap();
    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_lib_null_media() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = create_test_library(&mut tx).await;

    let _mfile_id = insert_mediafile(&mut tx).await;
    let result = mediafile::MediaFile::get_by_lib_null_media(&mut tx, id)
        .await
        .unwrap();

    assert_eq!(result[0].media_id, None);

    // TODO: check that mfiles with media_id dont get returned
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_one() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _id = create_test_library(&mut tx).await;

    let mfile_id = insert_mediafile(&mut tx).await;

    let result = mediafile::MediaFile::get_one(&mut tx, mfile_id)
        .await
        .unwrap();

    assert_eq!(result.raw_name, "Test".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_exists_by_file() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;

    assert!(!mediafile::MediaFile::exists_by_file(&mut tx, "/dev/null").await);

    let _ = insert_mediafile(&mut tx).await;
    assert!(mediafile::MediaFile::exists_by_file(&mut tx, "/dev/null").await);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_file() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;
    let _ = insert_mediafile(&mut tx).await;

    let result = mediafile::MediaFile::get_by_file(&mut tx, "/dev/null")
        .await
        .unwrap();
    assert_eq!(result.target_file, "/dev/null".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deletes() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let lib_id = create_test_library(&mut tx).await;
    let id = insert_mediafile(&mut tx).await;

    let rows = mediafile::MediaFile::delete(&mut tx, id).await.unwrap();
    assert_eq!(rows, 1);

    insert_many_mediafile(&mut tx, 10).await;
    let rows = mediafile::MediaFile::delete_by_lib_id(&mut tx, lib_id)
        .await
        .unwrap();
    assert_eq!(rows, 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;
    let id = insert_mediafile(&mut tx).await;

    let update = mediafile::UpdateMediaFile {
        raw_name: Some("test2".into()),
        duration: Some(3),
        ..Default::default()
    };

    update.update(&mut tx, id).await.unwrap();

    let mfile = mediafile::MediaFile::get_one(&mut tx, id).await.unwrap();
    assert_eq!(mfile.raw_name, "test2".to_string());
    assert_eq!(mfile.duration, Some(3));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_of_media() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;
    let media_id = super::media_tests::insert_media(&mut tx).await;
    let mfile = insert_mediafile_with_mediaid(&mut tx, media_id).await;

    let result = mediafile::MediaFile::get_of_media(&mut tx, media_id)
        .await
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].media_id, Some(media_id));
    assert_eq!(result[0].id, mfile);
}
