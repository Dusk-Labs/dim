use crate::get_&_memory;
use crate::season;
use crate::tv;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;
use super::tv_tests::insert_tv;

#[tokio::test(flavor = "multi_thread")]
async fn test_insert_and_get_methods() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;

    let result = season::Season::get_all(&, 1).await.unwrap();
    assert!(result.is_empty());

    let tv = insert_media(&mut *conn).await;
    tv::TVShow::insert(&, tv).await.unwrap();

    let result = season::Season::get(&, 1, 1).await;
    assert!(result.is_err());

    let _season = dbg!(season::InsertableSeason {
        season_number: 2,
        ..Default::default()
    }
    .insert(&, tv)
    .await
    .unwrap());

    let result = season::Season::get(&, 1, 2).await.unwrap();
    assert_eq!(result.season_number, 2);

    let result = season::Season::get_all(&, 1).await.unwrap();
    assert_eq!(result.len(), 1);

    let result = season::Season::get_first(&, 1).await.unwrap();
    assert_eq!(result.season_number, 2);

    let _season = dbg!(season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, tv)
    .await
    .unwrap());

    let result = dbg!(season::Season::get_all(&, 1).await.unwrap());
    let result = season::Season::get_first(&, 1).await.unwrap();
    assert_eq!(result.season_number, 1);

    let result = season::Season::get_by_id(&, _season).await.unwrap();
    assert_eq!(result.season_number, 1);

    let rows = season::Season::delete(&, 1, 2).await.unwrap();
    assert_eq!(rows, 1);

    let result = season::Season::get(&, 1, 2).await;
    assert!(result.is_err())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let tv = insert_tv(&mut *conn).await;

    let _season = dbg!(season::InsertableSeason {
        season_number: 2,
        ..Default::default()
    }
    .insert(&, tv)
    .await
    .unwrap());

    dbg!(season::Season::get_all(&, tv).await);

    let rows = season::UpdateSeason {
        season_number: Some(1),
        ..Default::default()
    }
    .update(&, _season)
    .await
    .unwrap();
    assert_eq!(rows, 1);

    let result = season::Season::get_by_id(&, _season).await.unwrap();
    assert_eq!(result.season_number, 1);
}
