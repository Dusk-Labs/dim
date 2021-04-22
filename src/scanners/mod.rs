pub mod base;
pub mod movie;
//pub mod scanner_daemon;
pub mod tmdb;
pub mod tv_show;

use database::get_conn;
use database::library;
use database::library::Library;
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
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

/*
    /// Function starts listing all the files in the library directory and starts scanning them.
    fn start(&self, custom_path: Option<&str>) {
        let lib = self.library_ref();
        let log = self.logger_ref();
        // sanity check
        debug_assert!(lib.media_type == Self::MEDIA_TYPE);
        info!(
            log,
            "Enumerating files for library={} with media_type={:?}",
            lib.id,
            Self::MEDIA_TYPE
        );

        let path = custom_path.unwrap_or(lib.location.as_str());
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
                    .map_or(false, |e| Self::SUPPORTED_EXTS.contains(&e))
            })
            .map(|f| f.into_path())
            .collect();

        info!(
            log,
            "Scanning {} files for library {} of {:?}",
            files.len(),
            lib.id,
            Self::MEDIA_TYPE
        );

        // mount the files found into the database.
        // Essentially we extract the bare minimum information from each file such as its codec,
        // title, year and container, and insert it into the database as an orphan media file.
        for file in files {
            if let Err(e) = self.mount_file(file) {
                error!(log, "Failed to mount file into the database: {:?}", e);
            }
        }

        self.fix_orphans();
    }

pub fn start(library_id: i32, log: &Logger, tx: EventTx) -> Result<(), ()> {
    info!(log, "Summoning scanner for Library with id: {}", library_id);

    if get_conn().is_ok() {
        let log_clone = log.clone();
        let tx_clone = tx.clone();

        let log = log_clone.clone();
        if let Err(e) = scanner_from_library(library_id, log_clone, tx_clone) {
            error!(
                log,
                "Scanner for lib: {} has failed to start with error: {:?}", library_id, e
            )
        }
    } else {
        error!(log, "Failed to connect to db");
        return Err(());
    }

    Ok(())
}

fn scanner_from_library(lib_id: i32, log: Logger, tx: EventTx) -> Result<(), ScannerError> {
    use self::movie::MovieScanner;
    use self::tv_show::TvShowScanner;
    use database::library::MediaType;

    let conn = get_conn()?;
    let library = Library::get_one(&conn, lib_id)?;

    match library.media_type {
        MediaType::Movie => {
            let scanner = MovieScanner::new(conn, lib_id, log, tx)?;
            scanner.start(None);
            scanner.start_daemon()?;
        }
        MediaType::Tv => {
            let scanner = TvShowScanner::new(conn, lib_id, log, tx)?;
            scanner.start(None);
            scanner.start_daemon()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
*/
