#![feature(result_map_or_else)]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate reqwest;
extern crate crossbeam_channel;
extern crate diesel;
extern crate notify;
extern crate rocket_slog;
extern crate torrent_name_parser;
extern crate rayon;

use dim_database::get_conn;
use slog::Logger;
use std::thread;

pub mod api;
pub mod iterative_parser;
pub mod parser_daemon;
pub mod tmdb;

use crate::iterative_parser::IterativeScanner;
use crate::parser_daemon::ParserDaemon;

pub type EventTx = std::sync::mpsc::Sender<dim_events::server::EventType>;

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
