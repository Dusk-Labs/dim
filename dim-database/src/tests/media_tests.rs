use crate::get_conn_memory;
use crate::library;
use crate::media;
use crate::write_tx;

use super::library_tests::create_test_library;
use super::mediafile_tests::insert_mediafile_with_mediaid;

pub async fn insert_media(conn: &mut crate::Transaction<'_>) -> i64 {
    let media = media::InsertableMedia {
        library_id: 1,
        name: "TestMedia".into(),
        description: None,
        rating: Some(10.0),
        year: Some(2020),
        added: "Test".into(),
        poster: None,
        backdrop: None,
        media_type: library::MediaType::Movie,
    };

    media.insert(&mut *conn).await.unwrap()
}

pub async fn insert_many(conn: &mut crate::Transaction<'_>, n: usize) {
    for i in 0..n {
        let media = media::InsertableMedia {
            library_id: 1,
            name: format!("TestMedia{}", i),
            description: None,
            rating: Some(10.0),
            year: Some(2020),
            added: "Test".into(),
            poster: None,
            backdrop: None,
            media_type: library::MediaType::Movie,
        };

        media.insert(&mut *conn).await.unwrap();
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;

    let media_id = insert_media(&mut tx).await;
    let media = media::Media::get(&mut tx, media_id).await.unwrap();
    assert_eq!(media.name, "TestMedia".to_string());
    assert_eq!(media.media_type, library::MediaType::Movie);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let library_id = create_test_library(&mut tx).await;

    let result = media::Media::get_all(&mut tx, library_id).await.unwrap();
    assert!(result.is_empty());

    insert_many(&mut tx, 10).await;
    let result = media::Media::get_all(&mut tx, library_id).await.unwrap();
    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_name_and_lib() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let library_id = create_test_library(&mut tx).await;

    let result = media::Media::get_by_name_and_lib(&mut tx, library_id, "TestMedia9").await;
    assert!(result.is_err());

    insert_many(&mut tx, 10).await;
    let result = media::Media::get_by_name_and_lib(&mut tx, library_id, "TestMedia9")
        .await
        .unwrap();
    assert_eq!(result.name, "TestMedia9".to_string());
    assert_eq!(result.library_id, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_of_mediafile() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;

    let result = media::Media::get_of_mediafile(&mut tx, 1).await;
    assert!(result.is_err());

    let media_id = insert_media(&mut tx).await;
    let mfile_id = insert_mediafile_with_mediaid(&mut tx, media_id).await;

    let _ = media::Media::get_of_mediafile(&mut tx, mfile_id)
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;
    let media_id = insert_media(&mut tx).await;

    let result = media::Media::delete(&mut tx, media_id).await.unwrap();
    assert_eq!(result, 1);

    let result = media::Media::get(&mut tx, media_id).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete_by_lib() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let library_id = create_test_library(&mut tx).await;
    insert_many(&mut tx, 10).await;

    let result = media::Media::get_all(&mut tx, library_id).await.unwrap();
    assert_eq!(result.len(), 10);

    let result = media::Media::delete_by_lib_id(&mut tx, library_id)
        .await
        .unwrap();
    assert_eq!(result, 10);

    let result = media::Media::get_all(&mut tx, library_id).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_blind_insert() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _library_id = create_test_library(&mut tx).await;

    let media = media::InsertableMedia {
        library_id: 1,
        name: "TestMedia".into(),
        description: None,
        rating: Some(10.0),
        year: Some(2020),
        added: "Test".into(),
        poster: None,
        backdrop: None,
        media_type: library::MediaType::Episode,
    };

    let result = media.clone().insert_blind(&mut tx).await.unwrap();
    assert_eq!(result, 1);

    let result = media.insert_blind(&mut tx).await.unwrap();
    assert_eq!(result, 2);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _library_id = create_test_library(&mut tx).await;

    let media = media::InsertableMedia {
        library_id: 1,
        name: "TestMedia".into(),
        description: None,
        rating: Some(10.0),
        year: Some(2020),
        added: "Test".into(),
        poster: None,
        backdrop: None,
        media_type: library::MediaType::Movie,
    };

    let media_id = media.insert(&mut tx).await.unwrap();

    let update = media::UpdateMedia {
        name: Some("TestMedia2".into()),
        rating: Some(5.0),
        ..Default::default()
    };

    let _ = update.update(&mut tx, media_id).await.unwrap();

    let result = media::Media::get(&mut tx, media_id).await.unwrap();
    assert_eq!(result.name, "TestMedia2".to_string());
    assert_eq!(result.rating, Some(5.0));
}
