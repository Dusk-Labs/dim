use super::library_tests::create_test_library;
use crate::get_&_memory;
use crate::movie;

#[tokio::test(flavor = "multi_thread")]
async fn test_insert() {
    let ref & = get_&_memory().await.unwrap();
    let _library = create_test_library(&mut *conn).await;

    let media_id = super::media_tests::insert_media(&mut *conn).await;
    movie::InsertableMovie::insert(&, media_id)
        .await
        .unwrap();
}
