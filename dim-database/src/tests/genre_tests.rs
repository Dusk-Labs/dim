use crate::genre;
use crate::get_conn_memory;
use crate::library;
use crate::media;
use crate::write_tx;

use super::library_tests::create_test_library;

pub async fn insert_genre(conn: &mut crate::Transaction<'_>, name: String) -> i64 {
    genre::InsertableGenre { name }
        .insert(&mut *conn)
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_name() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = insert_genre(&mut tx, "Test".into()).await;

    let result = genre::Genre::get_by_name(&mut tx, "Test".into())
        .await
        .unwrap();
    assert_eq!(result.id, id);
    assert_eq!(result.name, "Test".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_by_media() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _ = create_test_library(&mut tx).await;

    let id = insert_genre(&mut tx, "Test".into()).await;

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

    genre::InsertableGenreMedia::insert_pair(id, media_id, &mut tx)
        .await
        .unwrap();

    let genres = genre::Genre::get_by_media(&mut tx, media_id).await.unwrap();
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
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = insert_genre(&mut tx, "Test".into()).await;

    let result = genre::Genre::get_by_id(&mut tx, id).await.unwrap();
    assert_eq!(result.id, id);
    assert_eq!(result.name, "Test".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = insert_genre(&mut tx, "Test".into()).await;

    let rows = genre::Genre::delete(&mut tx, id).await.unwrap();
    assert_eq!(rows, 1);

    let result = genre::Genre::get_by_id(&mut tx, id).await;
    assert!(result.is_err());
}
