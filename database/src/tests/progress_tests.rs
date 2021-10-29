use crate::episode;
use crate::get_&_memory;
use crate::media;
use crate::progress;
use crate::season;
use crate::tv;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;
use super::user_tests::insert_user;

use std::time::SystemTime;

#[tokio::test(flavor = "multi_thread")]
async fn test_set_and_get_for_media_user() {
    let ref & = get_&_memory().await.unwrap();
    let library = create_test_library(&mut *conn).await;
    let user = insert_user(&mut *conn).await;
    let media = insert_media(&mut *conn).await;

    let result = progress::Progress::get_for_media_user(&, user.clone(), media)
        .await
        .unwrap();
    assert_eq!(result.delta, 0);
    assert_eq!(result.populated, 0);

    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let rows = progress::Progress::set(&, 100, user.clone(), media)
        .await
        .unwrap();
    assert_eq!(rows, 1);

    let result = progress::Progress::get_for_media_user(&, user.clone(), media)
        .await
        .unwrap();
    assert_eq!(result.delta, 100);
    assert!(result.populated <= ts);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_total_time_spent_watching() {
    let ref & = get_&_memory().await.unwrap();
    let library = create_test_library(&mut *conn).await;
    let user = insert_user(&mut *conn).await;

    let result = progress::Progress::get_total_time_spent_watching(&, user.clone())
        .await
        .unwrap();
    assert_eq!(result, 0);

    super::media_tests::insert_many(&, 10).await;

    for i in 1..=5 {
        let rows = progress::Progress::set(&, 100, user.clone(), i)
            .await
            .unwrap();
        assert_eq!(rows, 1);
    }

    let result = progress::Progress::get_total_time_spent_watching(&, user.clone())
        .await
        .unwrap();
    assert_eq!(result, 500);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_total_for_tv() {
    let ref & = get_&_memory().await.unwrap();
    let library = create_test_library(&mut *conn).await;
    let user = insert_user(&mut *conn).await;

    let tv = insert_media(&mut *conn).await;
    tv::TVShow::insert(&, tv).await.unwrap();

    let result = progress::Progress::get_total_for_tv(&, user.clone(), tv)
        .await
        .unwrap();
    assert_eq!(result, 0);

    let season = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, tv)
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
        .insert(&mut *conn)
        .await
        .unwrap();

        progress::Progress::set(&, 100, user.clone(), episode)
            .await
            .unwrap();
    }

    let result = progress::Progress::get_total_for_tv(&, user.clone(), tv)
        .await
        .unwrap();
    assert_eq!(result, 12 * 100);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_continue_watching() {
    let ref & = get_&_memory().await.unwrap();
    let library = create_test_library(&mut *conn).await;
    let user = insert_user(&mut *conn).await;

    super::media_tests::insert_many(&, 2).await;
    tv::TVShow::insert(&, 1).await.unwrap();
    tv::TVShow::insert(&, 2).await.unwrap();

    let season1 = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, 1)
    .await
    .unwrap();

    let season2 = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, 2)
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
    .insert(&mut *conn)
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
    .insert(&mut *conn)
    .await
    .unwrap();

    progress::Progress::set(&, 100, user.clone(), episode1)
        .await
        .unwrap();

    let result = progress::Progress::get_continue_watching(&, user.clone(), 2)
        .await
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, 1);

    progress::Progress::set(&, 100, user.clone(), episode2)
        .await
        .unwrap();

    let result = progress::Progress::get_continue_watching(&, user.clone(), 2)
        .await
        .unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].id, 2);
}
