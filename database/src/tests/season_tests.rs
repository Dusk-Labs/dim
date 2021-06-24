use crate::get_conn_memory;
use crate::season;
use crate::tv;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;

#[tokio::test(flavor = "multi_thread")]
async fn test_insert_and_get_methods() {
    let ref conn = get_conn_memory().await.unwrap();
    let _lib = create_test_library(conn).await;

    let result = season::Season::get_all(conn, 1).await.unwrap();
    assert!(result.is_empty());

    let tv = insert_media(conn).await;
    tv::TVShow::insert(conn, tv).await.unwrap();

    let result = season::Season::get(conn, 1, 1).await;
    assert!(result.is_err());

    let _season = dbg!(season::InsertableSeason {
        season_number: 2,
        ..Default::default()
    }
    .insert(conn, tv)
    .await
    .unwrap());

    let result = season::Season::get(conn, 1, 2).await.unwrap();
    assert_eq!(result.season_number, 2);

    let result = season::Season::get_all(conn, 1).await.unwrap();
    assert_eq!(result.len(), 1);

    let result = season::Season::get_first(conn, 1).await.unwrap();
    assert_eq!(result.season_number, 2);

    let _season = dbg!(season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(conn, tv)
    .await
    .unwrap());

    let result = dbg!(season::Season::get_all(conn, 1).await.unwrap());
    let result = season::Season::get_first(conn, 1).await.unwrap();
    assert_eq!(result.season_number, 1);

    let result = season::Season::get_by_id(conn, _season).await.unwrap();
    assert_eq!(result.season_number, 1);

    let rows = season::Season::delete(conn, 1, 2).await.unwrap();
    assert_eq!(rows, 1);

    let result = season::Season::get(conn, 1, 2).await;
    assert!(result.is_err())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update() {
    let ref conn = get_conn_memory().await.unwrap();
    let _lib = create_test_library(conn).await;
    let tv = insert_media(conn).await;
    tv::TVShow::insert(conn, tv).await.unwrap();

    let _season = dbg!(season::InsertableSeason {
        season_number: 2,
        ..Default::default()
    }
    .insert(conn, tv)
    .await
    .unwrap());

    let rows = season::UpdateSeason {
        season_number: Some(1),
        ..Default::default()
    }
    .update(conn, tv, 2)
    .await
    .unwrap();
    assert_eq!(rows, 1);

    let result = season::Season::get_by_id(conn, _season).await.unwrap();
    assert_eq!(result.season_number, 1);
}
