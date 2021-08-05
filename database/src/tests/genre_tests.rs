use crate::genre;
use crate::get_conn_memory;
use crate::library;
use crate::media;

use super::library_tests::create_test_library;

pub async fn insert_genre(conn: &crate::DbConnection, name: String) -> i64 {
    genre::InsertableGenre { name }.insert(conn).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_name() {
    let ref conn = get_conn_memory().await.unwrap();
    let id = insert_genre(conn, "Test".into()).await;

    let result = genre::Genre::get_by_name(conn, "Test".into())
        .await
        .unwrap();
    assert_eq!(result.id, id);
    assert_eq!(result.name, "Test".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_media() {
    let ref conn = get_conn_memory().await.unwrap();
    let _ = create_test_library(conn).await;

    let id = insert_genre(conn, "Test".into()).await;

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

    let media_id = media.insert(conn).await.unwrap();

    genre::InsertableGenreMedia::insert_pair(id, media_id, conn)
        .await
        .unwrap();

    let genres = genre::Genre::get_by_media(conn, media_id).await.unwrap();
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
    let ref conn = get_conn_memory().await.unwrap();
    let id = insert_genre(conn, "Test".into()).await;

    let result = genre::Genre::get_by_id(conn, id).await.unwrap();
    assert_eq!(result.id, id);
    assert_eq!(result.name, "Test".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let ref conn = get_conn_memory().await.unwrap();
    let id = insert_genre(conn, "Test".into()).await;

    let rows = genre::Genre::delete(conn, id).await.unwrap();
    assert_eq!(rows, 1);

    let result = genre::Genre::get_by_id(conn, id).await;
    assert!(result.is_err());
}
