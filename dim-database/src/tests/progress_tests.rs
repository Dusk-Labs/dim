use crate::episode;
use crate::get_conn_memory;
use crate::media;
use crate::progress;
use crate::season;
use crate::write_tx;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;
use super::user_tests::insert_user;

use std::time::SystemTime;

#[tokio::test(flavor = "multi_thread")]
async fn test_set_and_get_for_media_user() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _library = create_test_library(&mut tx).await;
    let user = insert_user(&mut tx).await;
    let media = insert_media(&mut tx).await;

    let result = progress::Progress::get_for_media_user(&mut tx, user.id, media)
        .await
        .unwrap();
    assert_eq!(result.delta, 0);
    assert_eq!(result.populated, 0);

    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let rows = progress::Progress::set(&mut tx, 100, user.id, media)
        .await
        .unwrap();
    assert_eq!(rows, 1);

    let result = progress::Progress::get_for_media_user(&mut tx, user.id, media)
        .await
        .unwrap();
    assert_eq!(result.delta, 100);
    assert!(result.populated <= ts);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_total_time_spent_watching() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _library = create_test_library(&mut tx).await;
    let user = insert_user(&mut tx).await;

    let result = progress::Progress::get_total_time_spent_watching(&mut tx, user.id)
        .await
        .unwrap();
    assert_eq!(result, 0);

    super::media_tests::insert_many(&mut tx, 10).await;

    for i in 1..=5 {
        let rows = progress::Progress::set(&mut tx, 100, user.id, i)
            .await
            .unwrap();
        assert_eq!(rows, 1);
    }

    let result = progress::Progress::get_total_time_spent_watching(&mut tx, user.id)
        .await
        .unwrap();
    assert_eq!(result, 500);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_total_for_tv() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let library = create_test_library(&mut tx).await;
    let user = insert_user(&mut tx).await;

    let tv = insert_media(&mut tx).await;

    let result = progress::Progress::get_total_for_tv(&mut tx, user.id, tv)
        .await
        .unwrap();
    assert_eq!(result, 0);

    let season = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&mut tx, tv)
    .await
    .unwrap();

    for i in 1..=12 {
        let episode = episode::InsertableEpisode {
            media: media::InsertableMedia {
                library_id: library,
                name: format!("TestEpisode{}", i),
                ..Default::default()
            },
            seasonid: season,
            episode: i,
        }
        .insert(&mut tx)
        .await
        .unwrap();

        progress::Progress::set(&mut tx, 100, user.id, episode)
            .await
            .unwrap();
    }

    let result = progress::Progress::get_total_for_tv(&mut tx, user.id, tv)
        .await
        .unwrap();
    assert_eq!(result, 12 * 100);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_continue_watching() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let library = create_test_library(&mut tx).await;
    let user = insert_user(&mut tx).await;

    super::media_tests::insert_many(&mut tx, 2).await;

    let season1 = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&mut tx, 1)
    .await
    .unwrap();

    let season2 = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&mut tx, 2)
    .await
    .unwrap();

    let episode1 = episode::InsertableEpisode {
        media: media::InsertableMedia {
            library_id: library,
            name: "TestEpisode1".into(),
            ..Default::default()
        },
        seasonid: season1,
        episode: 1,
    }
    .insert(&mut tx)
    .await
    .unwrap();

    let episode2 = episode::InsertableEpisode {
        media: media::InsertableMedia {
            library_id: library,
            name: "TestEpisode2".into(),
            ..Default::default()
        },
        seasonid: season2,
        episode: 1,
    }
    .insert(&mut tx)
    .await
    .unwrap();

    progress::Progress::set(&mut tx, 100, user.id, episode1)
        .await
        .unwrap();

    let result = progress::Progress::get_continue_watching(&mut tx, user.id, 2)
        .await
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 1);

    progress::Progress::set(&mut tx, 100, user.id, episode2)
        .await
        .unwrap();

    let result = progress::Progress::get_continue_watching(&mut tx, user.id, 2)
        .await
        .unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], 2);
}
