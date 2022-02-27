use std::fs::OpenOptions;

use database::{
    episode::Episode,
    get_conn_memory,
    library::InsertableLibrary,
    mediafile::{MediaFile, UpdateMediaFile},
};

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

    path
}

struct ScannerMock {
    event_rx: UnboundedReceiver<String>,
    event_tx: UnboundedSender<String>,
    dbconn: database::DbConnection,
    tempdir: TempDir,
    lib_id: i64,
    media_files: Vec<MediaFile>,
}

async fn test_scanner_insert_impl() -> ScannerMock {
    let (ev_tx, _rx) = mpsc::unbounded_channel();
    let mut db = database::get_conn_memory().await.unwrap();
    let dir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();

    let file_a_name = "abcdef_S1E1.mp4";
    let file_a = create_file(dir.path(), file_a_name);
    let file_a_path = file_a.to_string_lossy().to_string();

    let file_b_name = "abcdefasdasd.mp4";
    let file_b = create_file(dir.path(), file_b_name);
    let file_b_path = file_b.to_string_lossy().to_string();

    let i_lib = InsertableLibrary {
        name: format!("abc"),
        locations: vec![dir.path().to_owned().to_string_lossy().to_string()],
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

    let () = start(db.clone(), lib_id, ev_tx.clone()).await.unwrap();

    let lib = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = Library::get_one(&mut tx, lib_id).await.unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(lib.name, i_lib.name);
    assert_eq!(lib.locations, i_lib.locations);
    assert_eq!(lib.media_type, MediaType::Movie);

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
        tempdir: dir,
    }
}

async fn test_scanner_insert_and_patch_impl() {
    let ScannerMock {
        event_rx,
        event_tx,
        dbconn,
        tempdir,
        media_files,
        lib_id,
    } = test_scanner_insert_impl().await;

    let db = dbconn;

    let without_season = media_files.iter().find(|mf| mf.season.is_none()).unwrap();
    let with_season = media_files.iter().find(|mf| mf.season.is_some()).unwrap();

    {
        let mut update = UpdateMediaFile::default();
        update.season = Some(123456789);

        let writer = db.writer();
        let mut mutex_guard = writer.lock().await;
        let mut tx = mutex_guard.begin().await.unwrap();

        update.update(&mut tx, without_season.id).await.unwrap();

        tx.commit().await.unwrap();
    }

    {
        let mut update = UpdateMediaFile::default();
        update.episode = Some(123456789);

        let writer = db.writer();
        let mut mutex_guard = writer.lock().await;
        let mut tx = mutex_guard.begin().await.unwrap();

        update.update(&mut tx, with_season.id).await.unwrap();

        tx.commit().await.unwrap();
    }

    let () = start_incremental(db.clone(), lib_id, event_tx)
        .await
        .unwrap();

    let media_file_a = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = MediaFile::get_one(&mut tx, without_season.id)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(media_file_a.target_file, without_season.target_file);
    assert_eq!(media_file_a.season, Some(123456789));

    let media_file_b = {
        let mut tx = db.read().begin().await.unwrap();
        let lib = MediaFile::get_one(&mut tx, with_season.id).await.unwrap();
        tx.commit().await.unwrap();
        lib
    };

    assert_eq!(media_file_b.target_file, with_season.target_file);
    assert_eq!(media_file_b.episode, Some(123456789));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_scanner_insert_and_patch() {
    test_scanner_insert_and_patch_impl().await;
}
