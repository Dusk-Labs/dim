use crate::get_conn_memory;
use crate::progress;

use super::library_tests::create_test_library;
use super::media_tests::insert_media;
use super::user_tests::insert_user;

use std::time::SystemTime;

#[tokio::test(flavor = "multi_thread")]
async fn test_set_and_get_for_media_user() {
    let ref conn = get_conn_memory().await.unwrap();
    let library = create_test_library(conn).await;
    let user = insert_user(conn).await;
    let media = insert_media(conn).await;

    let result = progress::Progress::get_for_media_user(conn, user.clone(), media).await.unwrap();
    assert_eq!(result.delta, 0);
    assert_eq!(result.populated, 0);

    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let rows = progress::Progress::set(conn, 100, user.clone(), media).await.unwrap();
    assert_eq!(rows, 1);

    let result = progress::Progress::get_for_media_user(conn, user.clone(), media).await.unwrap();
    assert_eq!(result.delta, 100);
    assert!(result.populated <= ts);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_total_time_spent_watching() {
    let ref conn = get_conn_memory().await.unwrap();
    let library = create_test_library(conn).await;
    let user = insert_user(conn).await;

    let result = progress::Progress::get_total_time_spent_watching(conn, user.clone()).await.unwrap();
    assert_eq!(result, 0);

    super::media_tests::insert_many(conn, 10).await;

    for i in 1..=5 {
        let rows = progress::Progress::set(conn, 100, user.clone(), i).await.unwrap();
        assert_eq!(rows, 1);
    }

    let result = progress::Progress::get_total_time_spent_watching(conn, user.clone()).await.unwrap();
    assert_eq!(result, 500);
}
