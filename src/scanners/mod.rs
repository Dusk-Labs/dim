pub mod iterative_parser;
pub mod parser_daemon;
pub mod tmdb_api;

use self::{
    iterative_parser::IterativeScanner,
    parser_daemon::ParserDaemon,
    tmdb_api::{Media, MediaType},
};
use pushevent::Event;

use database::get_conn;
use database::library;
use database::library::Library;

use slog::debug;
use slog::error;
use slog::info;
use slog::Logger;

use walkdir::WalkDir;

use std::path::PathBuf;
use std::thread;

pub trait APIExec<'a> {
    fn new(api_key: &'a str) -> Self;
    fn search(&mut self, title: String, year: Option<i32>, media_type: MediaType) -> Option<Media>;
    fn search_many(
        &mut self,
        title: String,
        year: Option<i32>,
        media_type: MediaType,
        result_num: usize,
    ) -> Vec<Media>;
    fn search_by_id(&mut self, id: i32, media_type: MediaType) -> Option<Media>;
}

#[derive(Debug)]
pub enum ScannerError {
    // Used when a scanner was invoked over a invalid library type.
    InvalidLibraryType {
        expected: library::MediaType,
        got: library::MediaType,
    },
    // Used when a internal db error occured.
    InternalDbError,
    // Used when a scanner was started for a non-existant library.
    LibraryDoesntExist(i32),
}

impl From<diesel::ConnectionError> for ScannerError {
    fn from(_: diesel::ConnectionError) -> Self {
        Self::InternalDbError
    }
}

pub trait MediaScanner: Sized {
    /// The media type that this scanner supports.
    const MEDIA_TYPE: library::MediaType;
    /// The file extensions that this scanner supports.
    const SUPPORTED_EXTS: &'static [&'static str] = &["mkv", "mp4", "avi"];

    /// Function initializes a new scanner.
    ///
    /// # Arguments
    /// `library_id` - the id of the library we want to scan for.
    /// `log` - stdout logger
    /// `event_tx` - event_tx channel over which we can dispatch ws events.
    ///
    /// # Errors
    /// Will return `Err(LibraryDoesntExist(..))` if library with `library_id` doesnt exist.
    /// Will return `Err(InternalDbError)` if we cant acquire a database connection.
    /// Will return `Err(InvalidLibraryType)` if the library with id `library_id` isnt of
    /// media_type `Self::MEDIA_TYPE`.
    fn new(library_id: i32, log: Logger, event_tx: EventTx) -> Result<Self, ScannerError> {
        let conn = get_conn()?;
        let lib = Library::get_one(&conn, library_id)
            .map_err(|_| ScannerError::LibraryDoesntExist(library_id))?;

        if lib.media_type != Self::MEDIA_TYPE {
            return Err(ScannerError::InvalidLibraryType {
                expected: Self::MEDIA_TYPE,
                got: lib.media_type,
            });
        }

        Ok(Self::new_unchecked(conn, lib, log, event_tx))
    }

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

    // Function parses metadata from file `file` and inserts the data into the database.
    //
    // # Arguments
    // `file` - A pathbuffer containing the path to a media file we are trying to insert into the
    // database. This file will *ALWAYS* have a extension that is in `Self::SUPPORTED_EXTS`.
    fn mount_file(&self, file: PathBuf) -> Result<(), ()>;
    fn fix_orphans(&self);

    /// Function will create a instance of `Self` containing the parameters passed in.
    fn new_unchecked(
        conn: database::DbConnection,
        lib: Library,
        log: Logger,
        event_tx: EventTx,
    ) -> Self;

    fn logger_ref(&self) -> &Logger;
    fn library_ref(&self) -> &Library;
}

pub type EventTx = std::sync::mpsc::Sender<Event>;

pub fn start(library_id: i32, log: &Logger, tx: EventTx) -> std::result::Result<(), ()> {
    info!(log, "Summoning scanner for Library with id: {}", library_id);

    let mut threads = Vec::new();
    if get_conn().is_ok() {
        let log_clone = log.clone();
        let tx_clone = tx.clone();

        threads.push(thread::spawn(move || {
            let log = log_clone.clone();
            IterativeScanner::new(library_id, log_clone, tx_clone).map_or_else(
                |e| {
                    error!(
                        log,
                        "IterativeScanner for lib: {} has failed to start with error: {:?}",
                        library_id,
                        e
                    )
                },
                |x| x.start(None),
            );
        }));

        let log_clone = log.clone();

        threads.push(thread::spawn(move || {
            let log = log_clone.clone();

            ParserDaemon::new(library_id, log_clone, tx).map_or_else(
                |e| {
                    error!(
                        log,
                        "ParserDaemon for lib: {} could not be created with error: {:?}",
                        library_id,
                        e
                    );
                },
                |x| {
                    let _ = x.start_daemon().map_err(|e| {
                        error!(log, "ParserDaemon::start_daemon failed with error: {:?}", e)
                    });
                },
            );
        }));
    } else {
        error!(log, "Failed to connect to db");
        return Err(());
    }

    for t in threads {
        t.join().unwrap_or(());
    }
    Ok(())
}
