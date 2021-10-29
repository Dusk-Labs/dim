use crate::episode;
use crate::get_&_memory;
use crate::media;
use crate::season;
use crate::tv;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

#[tokio::test(flavor = "multi_thread")]
async fn test_insert_get_and_delete() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let tv = insert_media(&mut *conn).await;
    tv::TVShow::insert(&, tv).await.unwrap();

    let season = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, tv)
    .await
    .unwrap();

    let _episode = episode::InsertableEpisode {
        media: media::InsertableMedia {
            library_id: _lib,
            name: "TestEpisode".into(),
            ..Default::default()
        },
        seasonid: season,
        episode: 2,
    }
    .insert(&mut *conn)
    .await
    .unwrap();

    let result = episode::Episode::get(&, tv, 1, 2).await.unwrap();
    assert_eq!(result.media.name, "TestEpisode".to_string());

    let rows = episode::Episode::delete(&, _episode).await.unwrap();
    assert_eq!(rows, 1);

    let result = episode::Episode::get(&, tv, 1, 2).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all_of_season() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let tv = insert_media(&mut *conn).await;
    tv::TVShow::insert(&, tv).await.unwrap();

    let season = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, tv)
    .await
    .unwrap();

    let result = episode::Episode::get_all_of_season(&, season)
        .await
        .unwrap();
    assert_eq!(result.len(), 0);

    for i in 1..=5 {
        let _episode = episode::InsertableEpisode {
            media: media::InsertableMedia {
                library_id: _lib,
                name: format!("TestEpisode{}", i),
                ..Default::default()
            },
            seasonid: season,
            episode: i,
        }
        .insert(&mut *conn)
        .await
        .unwrap();
    }

    let result = episode::Episode::get_all_of_season(&, season)
        .await
        .unwrap();
    assert_eq!(result.len(), 5);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_first_of_season() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let tv = insert_media(&mut *conn).await;
    tv::TVShow::insert(&, tv).await.unwrap();

    let season = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, tv)
    .await
    .unwrap();

    let result = episode::Episode::get_first_for_season(&, season).await;
    assert!(result.is_err());

    for i in 3..=5 {
        let _episode = episode::InsertableEpisode {
            media: media::InsertableMedia {
                library_id: _lib,
                name: format!("TestEpisode{}", i),
                ..Default::default()
            },
            seasonid: season,
            episode: i,
        }
        .insert(&mut *conn)
        .await
        .unwrap();
    }

    let result = episode::Episode::get_first_for_season(&, season)
        .await
        .unwrap();
    assert_eq!(result.episode, 3);

    for i in 1..=2 {
        let _episode = episode::InsertableEpisode {
            media: media::InsertableMedia {
                library_id: _lib,
                name: format!("TestEpisode{}", i),
                ..Default::default()
            },
            seasonid: season,
            episode: i,
        }
        .insert(&mut *conn)
        .await
        .unwrap();
    }

    let result = episode::Episode::get_first_for_season(&, season)
        .await
        .unwrap();
    assert_eq!(result.episode, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all_of_tv() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let tv = insert_media(&mut *conn).await;
    tv::TVShow::insert(&, tv).await.unwrap();

    let result = episode::Episode::get_all_of_tv(&, tv).await.unwrap();
    assert!(result.is_empty());

    for i in 1..=3 {
        let season = season::InsertableSeason {
            season_number: i,
            ..Default::default()
        }
        .insert(&, tv)
        .await
        .unwrap();

        for i in 1..=12 {
            static _CNT: AtomicU64 = AtomicU64::new(0);
            let _episode = episode::InsertableEpisode {
                media: media::InsertableMedia {
                    library_id: _lib,
                    name: format!("TestEpisode{}", _CNT.load(Ordering::Relaxed)),
                    ..Default::default()
                },
                seasonid: season,
                episode: i,
            }
            .insert(&mut *conn)
            .await
            .unwrap();

            _CNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    let result = episode::Episode::get_all_of_tv(&, tv).await.unwrap();
    assert_eq!(result.len(), 36);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update() {
    let ref & = get_&_memory().await.unwrap();
    let _lib = create_test_library(&mut *conn).await;
    let tv = insert_media(&mut *conn).await;
    tv::TVShow::insert(&, tv).await.unwrap();

    let season = season::InsertableSeason {
        season_number: 1,
        ..Default::default()
    }
    .insert(&, tv)
    .await
    .unwrap();

    let _episode = episode::InsertableEpisode {
        media: media::InsertableMedia {
            library_id: _lib,
            name: "TestEpisode".into(),
            ..Default::default()
        },
        seasonid: season,
        episode: 2,
    }
    .insert(&mut *conn)
    .await
    .unwrap();

    let rows = episode::UpdateEpisode {
        episode: Some(3),
        ..Default::default()
    }
    .update(&, _episode)
    .await
    .unwrap();

    assert_eq!(rows, 1);

    let result = episode::Episode::get(&, tv, season, 3).await.unwrap();
    assert_eq!(result.id, _episode);
}
