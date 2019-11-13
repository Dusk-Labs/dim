#![feature(result_map_or_else)]
use database::get_conn;
use pushevent::Event;
use slog::Logger;
use slog::{error, info};
use std::thread;

pub mod iterative_parser;
pub mod parser_daemon;
pub mod tmdb_api;

use crate::iterative_parser::IterativeScanner;
use crate::parser_daemon::ParserDaemon;
use crate::tmdb_api::Media;
use crate::tmdb_api::MediaType;

pub trait APIExec<'a> {
    fn new(api_key: &'a str) -> Self;
    fn search(&mut self, title: String, year: Option<i32>, media_type: MediaType) -> Option<Media>;
}

pub type EventTx = std::sync::mpsc::Sender<Event>;

pub fn start(library_id: i32, log: &Logger, tx: EventTx) -> std::result::Result<(), ()> {
    let mut threads = Vec::new();

    info!(log, "Scanning {}", library_id);
    if get_conn().is_ok() {
        let log_clone = log.clone();
        let tx_clone = tx.clone();
        threads.push(thread::spawn(move || {
            let scanner = IterativeScanner::new(library_id, log_clone, tx_clone).unwrap();
            scanner.start(None);
        }));

        let log_clone = log.clone();
        let tx_clone = tx.clone();
        threads.push(thread::spawn(move || {
            let daemon = ParserDaemon::new(library_id, log_clone, tx_clone).unwrap();
            daemon.start_daemon().unwrap();
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
