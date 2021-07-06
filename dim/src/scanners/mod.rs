pub mod base;
pub mod movie;
pub mod scanner_daemon;
pub mod tmdb;
pub mod tv_show;

use database::get_conn;
use database::library;
use database::library::Library;
use database::library::MediaType;
use database::mediafile::InsertableMediaFile;
use database::mediafile::MediaFile;

use crate::streaming::FFPROBE_BIN;
use crate::{core::EventTx, streaming::ffprobe::FFProbeCtx};

use torrent_name_parser::Metadata;

use slog::debug;
use slog::error;
use slog::info;
use slog::warn;
use slog::Logger;

use walkdir::WalkDir;

use std::fmt;
use std::lazy::SyncOnceCell;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiMedia {
    pub id: u64,
    pub title: String,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub poster_file: Option<String>,
    pub backdrop_file: Option<String>,
    pub genres: Vec<String>,
    pub rating: Option<i32>,
    pub seasons: Vec<ApiSeason>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiSeason {
    pub id: u64,
    pub name: Option<String>,
    pub poster_path: Option<String>,
    pub poster_file: Option<String>,
    pub season_number: u64,
    pub episodes: Vec<ApiEpisode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiEpisode {
    pub id: u64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub episode: Option<u64>,
    pub still: Option<String>,
    pub still_file: Option<String>,
}

pub(super) static METADATA_EXTRACTOR: SyncOnceCell<base::MetadataExtractor> = SyncOnceCell::new();
pub(super) static METADATA_MATCHER: SyncOnceCell<base::MetadataMatcher> = SyncOnceCell::new();
pub(super) static SUPPORTED_EXTS: &[&str] = &["mp4", "mkv", "avi", "webm"];

pub fn get_extractor(log: &slog::Logger, tx: &EventTx) -> &'static base::MetadataExtractor {
    let mut handle = xtra::spawn::Tokio::Global;

    METADATA_EXTRACTOR
        .get_or_init(|| base::MetadataExtractor::cluster(&mut handle, 4, log.clone()).1)
}

pub fn get_matcher(log: &slog::Logger, tx: &EventTx) -> &'static base::MetadataMatcher {
    let mut handle = xtra::spawn::Tokio::Global;

    METADATA_MATCHER.get_or_init(|| {
        let conn = database::try_get_conn().expect("Failed to grab a connection");
        base::MetadataMatcher::cluster(&mut handle, 6, log.clone(), conn.clone(), tx.clone()).1
    })
}

pub fn get_matcher_unchecked() -> &'static base::MetadataMatcher {
    METADATA_MATCHER.get().unwrap()
}

pub async fn start_custom<T: AsRef<Path>>(
    library_id: i64,
    log: slog::Logger,
    tx: EventTx,
    path: T,
    media_type: MediaType,
) -> Result<(), self::base::ScannerError> {
    info!(log, "Scanning library"; "mod" => "scanner", "library_id" => library_id);
    tx.send(
        events::Message {
            id: library_id,
            event_type: events::PushEventType::EventStartedScanning,
        }
        .to_string(),
    )
    .unwrap();

    let conn = get_conn().await.expect("Failed to grab the conn pool");

    let extractor = get_extractor(&log, &tx);
    let matcher = get_matcher(&log, &tx);

    let files: Vec<PathBuf> = WalkDir::new(path)
        // we want to follow all symlinks in case of complex dir structures
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        // ignore all hidden files.
        .filter(|f| {
            !f.path()
                .iter()
                .any(|s| s.to_str().map(|x| x.starts_with('.')).unwrap_or(false))
        })
        // check whether `f` has a supported extension
        .filter(|f| {
            f.path()
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |e| SUPPORTED_EXTS.contains(&e))
        })
        .map(|f| f.into_path())
        .collect();

    let total_files = files.len();

    info!(
        log,
        "Walked library directory";
        "mod" => "scanner",
        "library_id" => library_id,
        "files" => total_files,
    );

    let mut futures = Vec::new();
    let now = Instant::now();

    for file in files {
        futures.push(async move {
            if let Ok(mfile) = extractor.mount_file(file, library_id, media_type).await {
                match media_type {
                    MediaType::Movie => {
                        let _ = matcher.match_movie(mfile).await;
                    }
                    MediaType::Tv => {
                        let _ = matcher.match_tv(mfile).await;
                    }
                    _ => unreachable!(),
                }
            }
        })
    }

    futures::future::join_all(futures).await;
    info!(
        log,
        "Finished scanning library";
        "library_id" => library_id,
        "files" => total_files,
        "duration" => now.elapsed().as_secs(),
    );
    tx.send(
        events::Message {
            id: library_id,
            event_type: events::PushEventType::EventStoppedScanning,
        }
        .to_string(),
    )
    .unwrap();

    Ok(())
}

pub async fn start(
    library_id: i64,
    log: slog::Logger,
    tx: EventTx,
) -> Result<(), self::base::ScannerError> {
    let conn = get_conn().await.expect("Failed to grab the conn pool");
    let lib = Library::get_one(&conn, library_id).await?;
    let path = lib.location.as_str();
    start_custom(library_id, log, tx, path, lib.media_type).await
}
