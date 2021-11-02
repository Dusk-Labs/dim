pub mod base;
pub mod movie;
pub mod scanner_daemon;
pub mod tmdb;
pub mod tv_show;

use database::get_conn;
use database::library::Library;
use database::library::MediaType;
use tracing::info;
use tracing::instrument;

use crate::core::EventTx;

use once_cell::sync::OnceCell;
use walkdir::WalkDir;

use std::path::Path;
use std::path::PathBuf;
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

pub(super) static METADATA_EXTRACTOR: OnceCell<base::MetadataExtractor> = OnceCell::new();
pub(super) static METADATA_MATCHER: OnceCell<base::MetadataMatcher> = OnceCell::new();
pub(super) static SUPPORTED_EXTS: &[&str] = &["mp4", "mkv", "avi", "webm"];

pub fn get_extractor(_tx: &EventTx) -> &'static base::MetadataExtractor {
    let mut handle = xtra::spawn::Tokio::Global;

    METADATA_EXTRACTOR.get_or_init(|| base::MetadataExtractor::cluster(&mut handle, 4).1)
}

pub fn get_matcher(tx: &EventTx) -> &'static base::MetadataMatcher {
    let mut handle = xtra::spawn::Tokio::Global;

    METADATA_MATCHER.get_or_init(|| {
        let conn = database::try_get_conn().expect("Failed to grab a connection");
        base::MetadataMatcher::cluster(&mut handle, 6, conn.clone(), tx.clone()).1
    })
}

pub fn get_matcher_unchecked() -> &'static base::MetadataMatcher {
    METADATA_MATCHER.get().unwrap()
}

#[instrument(skip(paths))]
pub async fn start_custom<I, T>(
    library_id: i64,
    tx: EventTx,
    paths: I,
    media_type: MediaType,
) -> Result<(), self::base::ScannerError>
where
    I: Iterator<Item = T>,
    T: AsRef<Path>,
{
    info!(library_id = library_id, "Scanning library");

    tx.send(
        events::Message {
            id: library_id,
            event_type: events::PushEventType::EventStartedScanning,
        }
        .to_string(),
    )
    .unwrap();

    let _conn = get_conn().await.expect("Failed to grab the conn pool");

    let extractor = get_extractor(&tx);
    let matcher = get_matcher(&tx);

    let mut files = Vec::with_capacity(2048);
    for path in paths {
        let mut subfiles: Vec<PathBuf> = WalkDir::new(path)
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

        files.append(&mut subfiles);
    }

    let total_files = files.len();

    info!(
        "Walked library directory {}/{}/{}",
        module = "scanner",
        library_id = library_id,
        files = total_files,
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
        "Finished scanning library {}/{}/{}",
        library_id = library_id,
        files = total_files,
        duration = now.elapsed().as_secs(),
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

pub async fn start(library_id: i64, tx: EventTx) -> Result<(), self::base::ScannerError> {
    let conn = get_conn().await.expect("Failed to grab the conn pool");
    let lib = Library::get_one(&conn, library_id).await?;

    start_custom(library_id, tx, lib.locations.into_iter(), lib.media_type).await
}
