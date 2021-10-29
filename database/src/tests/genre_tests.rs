use crate::genre;
use crate::get_&_memory;
use crate::library;
use crate::media;

use super::library_tests::create_test_library;

pub async fn insert_genre(conn: &mut crate::Transaction<'_>, name: String) -> i64 {
    genre::InsertableGenre { name }.insert(&mut *conn).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_name() {
    let ref & = get_&_memory().await.unwrap();
    let id = insert_genre(&, "Test".into()).await;

    let result = genre::Genre::get_by_name(&, "Test".into())
        .await
        .unwrap();
    assert_eq!(result.id, id);
    assert_eq!(result.name, "Test".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_media() {
    let ref & = get_&_memory().await.unwrap();
    let _ = create_test_library(&mut *conn).await;

    let id = insert_genre(&, "Test".into()).await;

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

    let media_id = media.insert(&mut *conn).await.unwrap();

    genre::InsertableGenreMedia::insert_pair(id, media_id, &)
        .await
        .unwrap();

    let genres = genre::Genre::get_by_media(&, media_id).await.unwrap();
    assert_eq!(
        genres,
        &[genre::Genre {
            id: 1,
            name: "Test".into()
        }]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_id() {
    let ref & = get_&_memory().await.unwrap();
    let id = insert_genre(&, "Test".into()).await;

    let result = genre::Genre::get_by_id(&, id).await.unwrap();
    assert_eq!(result.id, id);
    assert_eq!(result.name, "Test".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let ref & = get_&_memory().await.unwrap();
    let id = insert_genre(&, "Test".into()).await;

    let rows = genre::Genre::delete(&, id).await.unwrap();
    assert_eq!(rows, 1);

    let result = genre::Genre::get_by_id(&, id).await;
    assert!(result.is_err());
}
