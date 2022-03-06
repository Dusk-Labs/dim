#![allow(dead_code)]

use std::fs::OpenOptions;

use database::library::InsertableLibrary;
use database::mediafile::{MediaFile, UpdateMediaFile};

use sqlx::Acquire;

use tempfile::TempDir;

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

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

    path.canonicalize().unwrap()
}

struct ScannerMock {
    event_rx: UnboundedReceiver<String>,
    event_tx: UnboundedSender<String>,
    dbconn: database::DbConnection,
    tempdir: TempDir,
    lib_id: i64,
    media_files: Vec<MediaFile>,
    media_type: MediaType,
}

#[track_caller]
async fn insert_dummy_media(
    media_type: MediaType,
    db: database::DbConnection,
    name: &str,
) -> ScannerMock {
    let (ev_tx, _rx) = mpsc::unbounded_channel();
    let tempdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();

    let file_a_name = "abcdef_S1E1.mp4";
    let file_a = create_file(tempdir.path(), file_a_name);
    let file_a_path = file_a.to_string_lossy().to_string();

    let file_b_name = "abcdefasdasd.mp4";
    let file_b = create_file(tempdir.path(), file_b_name);
    let file_b_path = file_b.to_string_lossy().to_string();

    let i_lib = InsertableLibrary {
        name: format!("{name}"),
        locations: vec![tempdir.path().to_owned().to_string_lossy().to_string()],
        media_type,
    };

    let lib_id = {
        let writer = db.writer();
        let mut mutex_guard = writer.lock().await;
        let mut tx = mutex_guard.begin().await.unwrap();
        let lib_id = i_lib.insert(&mut tx).await.unwrap();
        tx.commit().await.unwrap();
        lib_id
    };

    let () = start(db.clone(), lib_id, ev_tx.clone()).await.unwrap();

    let lib = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = Library::get_one(&mut tx, lib_id).await.unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(lib.name, i_lib.name);
    assert_eq!(lib.locations, i_lib.locations);
    assert_eq!(lib.media_type, media_type);

    let media_file_a = {
        let mut tx = db.read().begin().await.unwrap();

        let lib = MediaFile::get_by_file(&mut tx, &file_a_path).await.unwrap();

        tx.commit().await.unwrap();

        lib
    };

    assert_eq!(media_file_a.target_file, file_a_path);

    let media_file_b = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = MediaFile::get_by_file(&mut tx, &file_b_path).await.unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(media_file_b.target_file, file_b_path);

    ScannerMock {
        event_rx: _rx,
        event_tx: ev_tx,
        dbconn: db,
        lib_id,
        media_files: vec![media_file_a, media_file_b],
        tempdir,
        media_type,
    }
}

#[track_caller]
async fn test_scanner_insert_and_patch_impl<F, Fut>(mock_impl: F)
where
    F: FnOnce() -> Fut,
    Fut: core::future::Future<Output = ScannerMock>,
{
    let ScannerMock {
        event_tx,
        event_rx: _ev_rx,
        dbconn,
        media_files,
        lib_id,
        tempdir: _tmpdir,
        ..
    } = mock_impl().await;

    let db = dbconn;

    let without_episode = media_files.iter().find(|mf| mf.episode.is_none()).unwrap();

    let with_episode = media_files.iter().find(|mf| mf.episode.is_some()).unwrap();

    {
        let mut update = UpdateMediaFile::default();
        update.episode = Some(123456789);

        let writer = db.writer();
        let mut mutex_guard = writer.lock().await;
        let mut tx = mutex_guard.begin().await.unwrap();

        update.update(&mut tx, without_episode.id).await.unwrap();

        tx.commit().await.unwrap();
    }

    {
        let mut update = UpdateMediaFile::default();
        update.episode = Some(123456789);

        let writer = db.writer();
        let mut mutex_guard = writer.lock().await;
        let mut tx = mutex_guard.begin().await.unwrap();

        update.update(&mut tx, with_episode.id).await.unwrap();

        tx.commit().await.unwrap();
    }

    let () = start_incremental(db.clone(), lib_id, event_tx)
        .await
        .unwrap();

    let media_file_a = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = MediaFile::get_one(&mut tx, without_episode.id)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(media_file_a.target_file, without_episode.target_file);
    assert_eq!(media_file_a.episode, Some(123456789));

    let media_file_b = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = MediaFile::get_one(&mut tx, with_episode.id).await.unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(media_file_b.target_file, with_episode.target_file);
    assert_eq!(media_file_b.episode, Some(123456789));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_scanner_insert_and_patch() {
    let conn = database::get_conn_memory().await.unwrap();

    test_scanner_insert_and_patch_impl(|| insert_dummy_media(MediaType::Tv, conn.clone(), "abc"))
        .await;

    test_scanner_insert_and_patch_impl(|| {
        insert_dummy_media(MediaType::Movie, conn.clone(), "def")
    })
    .await;
}
