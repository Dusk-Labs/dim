use crate::get_conn_memory;
use crate::library;

async fn create_test_library(conn: &crate::DbConnection) -> i32 {
    let lib = library::InsertableLibrary {
        name: "test".into(),
        location: "/dev/null".into(),
        media_type: library::MediaType::Movie,
    };

    lib.insert(conn).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_insert() {
    let conn = get_conn_memory().await.unwrap();
    let id = create_test_library(&conn).await;
    assert_eq!(id, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_one() {
    let conn = get_conn_memory().await.unwrap();
    let id = create_test_library(&conn).await;

    let result = library::Library::get_one(&conn, id).await.unwrap();

    assert_eq!(result.name, "test".to_string());
    assert_eq!(result.location, "/dev/null".to_string());
    assert_eq!(result.media_type, library::MediaType::Movie);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all() {
    let conn = get_conn_memory().await.unwrap();
    for _ in 0..10 {
        create_test_library(&conn).await;
    }

    let result = library::Library::get_all(&conn).await;

    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let conn = get_conn_memory().await.unwrap();
    let id = create_test_library(&conn).await;

    library::Library::get_one(&conn, id).await.unwrap();

    let rows = library::Library::delete(&conn, id).await.unwrap();
    assert_eq!(rows, 1);
}
