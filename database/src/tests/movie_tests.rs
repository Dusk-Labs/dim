use super::library_tests::create_test_library;
use crate::get_conn_memory;
use crate::movie;

#[tokio::test(flavor = "multi_thread")]
async fn test_insert() {
    let ref conn = get_conn_memory().await.unwrap();
    let _library = create_test_library(conn).await;

    let media_id = super::media_tests::insert_media(conn).await;
    movie::InsertableMovie::insert(conn, media_id)
        .await
        .unwrap();
}
