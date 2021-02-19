use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

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

pub trait ScannerDaemon: MediaScanner {
    fn start_daemon(&self) -> nResult<()> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = <RecommendedWatcher as Watcher>::new(tx, Duration::from_secs(1))?;
        let log = self.logger_ref();

        watcher.watch(
            self.library_ref().location.as_str(),
            RecursiveMode::Recursive,
        )?;

        loop {
            match rx.recv() {
                Ok(DebouncedEvent::Create(path)) => self.handle_create(path),
                Ok(DebouncedEvent::Rename(from, to)) => self.handle_rename(from, to),
                Ok(DebouncedEvent::Remove(path)) => self.handle_remove(path),
                Ok(event) => debug!(log, "Tried to handle unmatched event {:?}", event),
                Err(e) => error!(log, "Received error: {:?}", e),
            }
        }
    }

    fn handle_create(&self, path: PathBuf) {
        let log = self.logger_ref();

        debug!(log, "Received handle_create event type: {:?}", path);

        if path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |e| {
                    <Self as MediaScanner>::SUPPORTED_EXTS.contains(&e)
                })
        {
            if let Err(e) = self.mount_file(path.clone()) {
                warn!(log, "Failed to mount file={:?} e={:?}", path, e);
                return;
            }
        } else if path.is_dir() {
            self.start(path.to_str());
        }

        self.fix_orphans();
    }

    fn handle_remove(&self, path: PathBuf) {
        debug!(self.logger_ref(), "Received handle remove {:?}", path);
    }

    fn handle_rename(&self, from: PathBuf, to: PathBuf) {
        debug!(
            self.logger_ref(),
            "Received handle rename {:?} -> {:?}", from, to
        );
    }
}
