use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use super::movie::MovieScanner;
use super::tv_show::TvShowScanner;
use super::EventTx;
use super::MediaScanner;

use database::get_conn;
use database::library::Library;
use database::library::MediaType;

use slog::debug;
use slog::error;
use slog::warn;
use slog::Logger;

use notify::DebouncedEvent;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Result as nResult;
use notify::Watcher;

pub struct ParserDaemon {
    lib: Library,
    log: Logger,
    event_tx: EventTx,
}

impl ParserDaemon {
    pub fn new(library_id: i32, log: Logger, event_tx: EventTx) -> Result<Self, ()> {
        let conn = get_conn().expect("Failed to connect to postgres");

        if let Ok(lib) = Library::get_one(&conn, library_id) {
            return Ok(Self { lib, log, event_tx });
        }

        Err(())
    }

    pub fn start(&self) -> nResult<()> {
        match self.lib.media_type {
            MediaType::Tv => self.start_daemon::<TvShowScanner>(),
            MediaType::Movie => self.start_daemon::<MovieScanner>(),
            _ => unreachable!(),
        }
    }

    fn start_daemon<T: MediaScanner>(&self) -> nResult<()> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = <RecommendedWatcher as Watcher>::new(tx, Duration::from_secs(1))?;

        watcher.watch(&self.lib.location, RecursiveMode::Recursive)?;

        loop {
            match rx.recv() {
                Ok(event) => self.handle_event::<T>(event),
                Err(err) => error!(self.log, "Received error: {:?}", err),
            };
        }
    }

    fn handle_event<T: MediaScanner>(&self, event: notify::DebouncedEvent) {
        debug!(self.log, "Handling event: {:?}", event);
        match event {
            DebouncedEvent::Create(path) => self.handle_create::<T>(path),
            DebouncedEvent::Rename(from, to) => self.handle_rename(from, to),
            DebouncedEvent::Remove(path) => self.handle_remove(path),
            _ => {}
        }
    }

    fn handle_create<T: MediaScanner>(&self, path: PathBuf) {
        let parser = match T::new(self.lib.id, self.log.clone(), self.event_tx.clone()) {
            Ok(x) => x,
            Err(e) => {
                warn!(self.log, "Failed to start the scanner daemon e={:?}", e);
                return;
            }
        };

        debug!(self.log, "Received handle_create event type: {:?}", path);

        if path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |e| T::SUPPORTED_EXTS.contains(&e))
        {
            if let Err(e) = parser.mount_file(path.clone()) {
                warn!(self.log, "Failed to mount file={:?} e={:?}", path, e);
                return;
            }
        } else if path.is_dir() {
            parser.start(path.to_str());
        }

        parser.fix_orphans();
    }

    // Fuck. rename events get first handled as fucking remove events. What the fuck.
    fn handle_remove(&self, path: PathBuf) {
        debug!(self.log, "Received handle_remove event type: {:?}", path);
    }

    fn handle_rename(&self, from: PathBuf, to: PathBuf) {
        debug!(
            self.log,
            "Received handle_rename for file: {:?} to {:?}", from, to
        );
    }
}
