pub mod iterative_parser;
pub mod parser_daemon;
pub mod tmdb_api;

use self::{
    iterative_parser::IterativeScanner,
    parser_daemon::ParserDaemon,
    tmdb_api::{Media, MediaType},
};
use database::get_conn;
use pushevent::Event;
use slog::{error, info, Logger};
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
