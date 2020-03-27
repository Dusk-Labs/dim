use super::{iterative_parser::IterativeScanner, EventTx};
use database::{get_conn, library::Library};
use notify::{DebouncedEvent::*, RecommendedWatcher, RecursiveMode, Result as nResult, Watcher};
use slog::{debug, error, Logger};
use std::{path::PathBuf, time::Duration, sync::mpsc};

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

    pub fn start_daemon(&self) -> nResult<()> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = <RecommendedWatcher as Watcher>::new(tx, Duration::from_secs(1))?;

        watcher.watch(&self.lib.location, RecursiveMode::Recursive)?;

        loop {
            match rx.recv() {
                Ok(event) => self.handle_event(event),
                Err(err) => error!(self.log, "Received error: {:?}", err),
            };
        }
    }

    fn handle_event(&self, event: notify::DebouncedEvent) {
        debug!(self.log, "Handling event: {:?}", event);
        match event {
            Create(path) => self.handle_create(path),
            Rename(from, to) => self.handle_rename(from, to),
            Remove(path) => self.handle_remove(path),
            _ => {}
        }
    }

    fn handle_create(&self, path: PathBuf) {
        let parser =
            IterativeScanner::new(self.lib.id, self.log.clone(), self.event_tx.clone()).unwrap();

        debug!(self.log, "Received handle_create event type: {:?}", path);

        if path.is_file()
            && [".avi", ".mkv", ".mp4"].contains(&path.extension().unwrap().to_str().unwrap())
        {
            parser.mount_file(path.to_string_lossy()).unwrap();
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
