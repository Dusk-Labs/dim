use crate::get_conn_memory;
use crate::season;
use crate::write_tx;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;
use super::tv_tests::insert_tv;

#[tokio::test(flavor = "multi_thread")]
async fn test_insert_and_get_methods() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();

    let _lib = create_test_library(&mut tx).await;

    let result = season::Season::get_all(&mut tx, 1).await.unwrap();
    assert!(result.is_empty());

    let tv = insert_media(&mut tx).await;

    let result = season::Season::get(&mut tx, 1, 1).await;
    assert!(result.is_err());

    let _season = dbg!(season::InsertableSeason {
        season_number: 2,
        ..Default::default()
    }
    .insert(&mut tx, tv)
    .await
    .unwrap());

    let result = season::Season::get(&mut tx, 1, 2).await.unwrap();
    assert_eq!(result.season_number, 2);

    let result = season::Season::get_all(&mut tx, 1).await.unwrap();
    assert_eq!(result.len(), 1);

    let result = season::Season::get_first(&mut tx, 1).await.unwrap();
    assert_eq!(result.season_number, 2);

    let _season = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&mut tx, tv)
    .await
    .unwrap();

    let _result = season::Season::get_all(&mut tx, 1).await.unwrap();
    let result = season::Season::get_first(&mut tx, 1).await.unwrap();
    assert_eq!(result.season_number, 1);

    let result = season::Season::get_by_id(&mut tx, _season).await.unwrap();
    assert_eq!(result.season_number, 1);

    let rows = season::Season::delete_by_id(&mut tx, _season)
        .await
        .unwrap();
    assert_eq!(rows, 1);

    let result = season::Season::get(&mut tx, 1, 1).await;
    assert!(result.is_err())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _lib = create_test_library(&mut tx).await;
    let tv = insert_tv(&mut tx).await;

    let _season = dbg!(season::InsertableSeason {
        season_number: 2,
        ..Default::default()
    }
    .insert(&mut tx, tv)
    .await
    .unwrap());

    season::Season::get_all(&mut tx, tv).await.unwrap();

    let rows = season::UpdateSeason {
        season_number: Some(1),
        ..Default::default()
    }
    .update(&mut tx, _season)
    .await
    .unwrap();
    assert_eq!(rows, 1);

    let result = season::Season::get_by_id(&mut tx, _season).await.unwrap();
    assert_eq!(result.season_number, 1);
}
