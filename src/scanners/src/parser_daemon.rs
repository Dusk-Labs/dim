use crate::iterative_parser::IterativeScanner;
use crate::EventTx;
use crossbeam_channel::unbounded;
use dim_database::{get_conn, library::Library};
use notify::event::{EventKind::*, ModifyKind::*};
use notify::{RecommendedWatcher, RecursiveMode, Result as nResult, Watcher};
use slog::Logger;
use std::path::PathBuf;
use std::time::Duration;

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
        // create channel to send and receive events over
        let (tx, rx) = unbounded();
        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
        //
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(&self.lib.location, RecursiveMode::Recursive)?;

        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.
        loop {
            match rx.recv() {
                Ok(event) => self.handle_event(event.unwrap()),
                Err(err) => error!(self.log, "Received error: {:?}", err),
            };
        }
    }

    fn handle_event(&self, event: notify::Event) {
        debug!(self.log, "Handling event: {:?}", event);
        match event.kind {
            Create(_) => self.handle_create(event.paths),
            Modify(kind) => {
                if let Name(_) = kind {
                    self.handle_rename(event.paths)
                }
            }
            Remove(_) => self.handle_remove(event.paths),
            _ => {}
        }
    }

    fn handle_create(&self, paths: Vec<PathBuf>) {
        for path in paths {
            let parser = IterativeScanner::new(self.lib.id, self.log.clone(), self.event_tx.clone()).unwrap();

            debug!(self.log, "Received handle_create event type: {:?}", path);

            if path.is_file()
                && [".avi", ".mkv", ".mp4"].contains(&path.extension().unwrap().to_str().unwrap())
            {
                parser.mount_file(path).unwrap();
            } else if path.is_dir() {
                parser.start(path.to_str());
            }

            parser.fix_orphans();
        }
    }

    // Fuck. rename events get first handled as fucking remove events. What the fuck.
    fn handle_remove(&self, paths: Vec<PathBuf>) {
        debug!(self.log, "Received handle_remove event type: {:?}", paths);
    }

    fn handle_rename(&self, paths: Vec<PathBuf>) {
        debug!(self.log, "Received handle_rename event type: {:?}", paths);
    }
}
