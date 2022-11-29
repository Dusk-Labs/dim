use super::library_tests::create_test_library;
use crate::get_conn_memory;
use crate::write_tx;

#[tokio::test(flavor = "multi_thread")]
async fn test_insert() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let _library = create_test_library(&mut tx).await;

    let _media_id = super::media_tests::insert_media(&mut tx).await;
}
