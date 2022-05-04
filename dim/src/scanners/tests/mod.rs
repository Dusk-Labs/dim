#![allow(dead_code)]

use async_trait::async_trait;
use database::mediafile::{MediaFile, UpdateMediaFile};

use futures::StreamExt;
use sqlx::Acquire;

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use super::*;

mod fs;

#[async_trait]
trait DbExt {
    async fn insert_library(&mut self, lib: &fs::LibDir) -> i64;
    async fn get_library(&self, lib_id: i64) -> Library;
    async fn get_media_file(&self, file: &fs::File) -> MediaFile;
}

#[async_trait]
impl DbExt for database::DbConnection {
    #[track_caller]
    async fn insert_library(&mut self, lib: &fs::LibDir) -> i64 {
        let writer = self.writer();
        let mut mutex_guard = writer.lock().await;
        let mut tx = mutex_guard.begin().await.unwrap();
        let lib_id = lib.as_insertable_library().insert(&mut tx).await.unwrap();
        tx.commit().await.unwrap();
        lib_id
    }

    #[track_caller]
    async fn get_library(&self, lib_id: i64) -> Library {
        let mut tx = self.read().begin().await.unwrap();
        let lib = Library::get_one(&mut tx, lib_id).await.unwrap();
        tx.commit().await.unwrap();
        lib
    }

    async fn get_media_file(&self, file: &fs::File) -> MediaFile {
        let mut tx = self.read().begin().await.unwrap();
        let file = MediaFile::get_by_file(&mut tx, &file.path_as_string)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        file
    }
}

struct ScannerMock<T> {
    event_rx: UnboundedReceiver<String>,
    event_tx: UnboundedSender<String>,
    dbconn: database::DbConnection,
    library: T,
}

/// Create a library of the provided `media_type` and called `name` with two files. insert the library into the provided database connection and perform a scan of it.
///
/// one file has a TV episode prefix "_S1E1.mp4" and the other does not.
///
#[track_caller]
async fn insert_dummy_media(
    media_type: MediaType,
    mut db: database::DbConnection,
    name: &str,
) -> ScannerMock<fs::LibDir> {
    let (ev_tx, _rx) = mpsc::unbounded_channel();

    let mut libdir = fs::LibDir::new(name.into(), media_type);
    let file_a = libdir.create_file("abcdef_S1E1.mp4").clone();
    let file_b = libdir.create_file("abcdefasdasd.mp4").clone();

    libdir.lib_id = db.insert_library(&libdir).await;
    let lib_id = libdir.lib_id;

    let () = super::start(db.clone(), lib_id, ev_tx.clone())
        .await
        .unwrap();

    let lib = db.get_library(lib_id).await;
    assert_eq!(lib.name, name);
    assert_eq!(lib.locations.len(), 1);
    assert_eq!(lib.media_type, media_type);

    let mf_a = db.get_media_file(&file_a).await;
    assert_eq!(mf_a.target_file, file_a.path_as_string);

    let mf_b = db.get_media_file(&file_b).await;
    assert_eq!(mf_b.target_file, file_b.path_as_string);

    ScannerMock {
        event_rx: _rx,
        event_tx: ev_tx,
        dbconn: db,
        library: libdir,
    }
}

async fn find_map<I, M, MFut, T, F>(iter: I, map: M, pred: F) -> Option<T>
where
    I: IntoIterator,
    M: Fn(I::Item) -> MFut,
    MFut: std::future::Future<Output = T>,
    F: Fn(&T) -> bool,
{
    let mut item = None;

    for elem in iter {
        let elem = map(elem).await;

        if pred(&elem) {
            item = Some(elem);
            break;
        }
    }

    item
}

/// create a scanner mock and poke out some metadata about the media files, then rescan and see if it heals.
#[track_caller]
async fn test_scanner_insert_and_patch_impl<F, Fut>(mock_impl: F)
where
    F: FnOnce() -> Fut,
    Fut: core::future::Future<Output = ScannerMock<fs::LibDir>>,
{
    let ScannerMock {
        event_tx,
        event_rx: _ev_rx,
        dbconn: db,
        library,
        ..
    } = mock_impl().await;

    let without_episode = find_map(
        library.files.iter(),
        |file| db.get_media_file(file),
        |mf| mf.episode.is_none(),
    )
    .await
    .unwrap();

    let with_episode = find_map(
        library.files.iter(),
        |file| db.get_media_file(file),
        |mf| mf.episode.is_some(),
    )
    .await
    .unwrap();

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

    let () = start_incremental(db.clone(), library.lib_id, event_tx)
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
