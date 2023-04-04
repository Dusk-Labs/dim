use crate::get_conn_memory;
use crate::library;
use crate::write_tx;

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

pub async fn create_test_library(conn: &mut crate::Transaction<'_>) -> i64 {
    static _LIB: AtomicU64 = AtomicU64::new(0);
    let lib = library::InsertableLibrary {
        name: format!("test{}", _LIB.load(Ordering::Relaxed)),
        locations: vec![format!("/dev/null{}", _LIB.load(Ordering::Relaxed))],
        media_type: library::MediaType::Movie,
    };

    _LIB.fetch_add(1, Ordering::SeqCst);
    lib.insert(&mut *conn).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_insert() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = create_test_library(&mut tx).await;
    assert_eq!(id, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_one() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = create_test_library(&mut tx).await;

    let result = library::Library::get_one(&mut tx, id).await.unwrap();

    assert_eq!(result.media_type, library::MediaType::Movie);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    for _ in 0..10 {
        create_test_library(&mut tx).await;
    }

    let result = library::Library::get_all(&mut tx).await;

    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let id = create_test_library(&mut tx).await;

    library::Library::get_one(&mut tx, id).await.unwrap();

    let rows = library::Library::delete(&mut tx, id).await.unwrap();
    assert_eq!(rows, 1);
}
