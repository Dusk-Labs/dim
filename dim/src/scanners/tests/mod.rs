use std::fs::OpenOptions;

use database::{get_conn_memory, library::InsertableLibrary};
use sqlx::Acquire;
use tokio::sync::mpsc;

use super::*;

fn create_file(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_owned();
    path.push(name);

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .unwrap();

    path
}

async fn test_insert_impl() {
    let (tx, _rx) = mpsc::unbounded_channel();
    let mut db = database::get_conn_memory().await.unwrap();
    let dir = tempfile::tempdir().unwrap();

    let file_a = create_file(dir.path(), "abcdef_S1E1");
    let file_b = create_file(dir.path(), "abcdefasdasd");

    let i_lib = InsertableLibrary {
        name: format!("abc"),
        locations: vec![
            file_a.to_string_lossy().to_string(),
            file_b.to_string_lossy().to_string(),
        ],
        media_type: MediaType::Movie,
    };

    let lib_id = {
        let writer = db.writer();
        let mut mutex_guard = writer.lock().await;
        let mut tx = mutex_guard.begin().await.unwrap();
        let lib_id = i_lib.insert(&mut tx).await.unwrap();
        tx.commit().await.unwrap();
        lib_id
    };

    let () = start(db.clone(), lib_id, tx).await.unwrap();

    let lib = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = Library::get_one(&mut tx, lib_id).await.unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(lib.name, i_lib.name);
    assert_eq!(lib.locations, i_lib.locations);
    assert_eq!(lib.media_type, MediaType::Movie);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_insert() {
    test_insert_impl().await;
}
