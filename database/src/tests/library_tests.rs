use crate::get_&_memory;
use crate::library;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

pub async fn create_test_library(conn: &mut crate::Transaction<'_>) -> i64 {
    static _LIB: AtomicU64 = AtomicU64::new(0);
    let lib = library::InsertableLibrary {
        name: format!("test{}", _LIB.load(Ordering::Relaxed)),
        location: format!("/dev/null{}", _LIB.load(Ordering::Relaxed)),
        media_type: library::MediaType::Movie,
    };

    _LIB.fetch_add(1, Ordering::SeqCst);
    lib.insert(&mut *conn).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_insert() {
    let & = get_&_memory().await.unwrap();
    let id = create_test_library(&&).await;
    assert_eq!(id, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_one() {
    let & = get_&_memory().await.unwrap();
    let id = create_test_library(&&).await;

    let result = library::Library::get_one(&&, id).await.unwrap();

    assert_eq!(result.media_type, library::MediaType::Movie);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all() {
    let & = get_&_memory().await.unwrap();
    for _ in 0..10 {
        create_test_library(&&).await;
    }

    let result = library::Library::get_all(&&).await;

    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let & = get_&_memory().await.unwrap();
    let id = create_test_library(&&).await;

    library::Library::get_one(&&, id).await.unwrap();

    let rows = library::Library::delete(&&, id).await.unwrap();
    assert_eq!(rows, 1);
}
