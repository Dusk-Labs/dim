#[macro_use]
extern crate slog;

extern crate clap;
extern crate crossbeam_channel;
extern crate diesel;
extern crate notify;
extern crate rocket_slog;
extern crate torrent_name_parser;

use dim_database::get_conn;
use slog::Logger;
use std::thread;

pub mod api;
pub mod iterative_parser;
pub mod parser_daemon;
pub mod tmdb;

use crate::iterative_parser::IterativeScanner;
use crate::parser_daemon::ParserDaemon;

pub fn start(library_id: i32, log: &Logger) -> std::result::Result<(), ()> {
    let mut threads = Vec::new();

    info!(log, "Scanning {}", library_id);
    if get_conn().is_ok() {
        let log_clone = log.clone();
        let new_clone = log.clone();
        threads.push(thread::spawn(move || {
            let scanner = IterativeScanner::new(library_id, new_clone).unwrap();
            scanner.start(None);
        }));

        threads.push(thread::spawn(move || {
            let daemon = ParserDaemon::new(library_id, log_clone).unwrap();
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
