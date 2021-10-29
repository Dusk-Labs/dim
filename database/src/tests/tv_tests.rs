use crate::get_&_memory;
use crate::library;
use crate::media;
use crate::tv;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;

pub async fn insert_tv(conn: &mut crate::Transaction<'_>) -> i64 {
    let media = media::InsertableMedia {
        library_id: 1,
        name: "TestMedia".into(),
        description: None,
        rating: Some(10),
        year: Some(2020),
        added: "Test".into(),
        poster: None,
        backdrop: None,
        media_type: library::MediaType::Movie,
    };

    let id = media.insert(&mut *conn).await.unwrap();
    tv::TVShow::insert(&, id).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_insert_get_all() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let media = insert_media(&mut *conn).await;

    let result = tv::TVShow::get_all(&mut *conn).await.unwrap();
    assert!(result.is_empty());

    let id = tv::TVShow::insert(&, media).await.unwrap();
    assert_eq!(id, media);

    let result = tv::TVShow::get_all(&mut *conn).await.unwrap();
    assert_eq!(result.len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_upgrade() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let media = insert_media(&mut *conn).await;

    let result = tv::TVShow { id: media }.upgrade(&mut *conn).await;
    assert!(result.is_err());

    let _id = tv::TVShow::insert(&, media).await.unwrap();
    let result = tv::TVShow { id: media }.upgrade(&mut *conn).await;

    assert!(result.is_ok());
}
