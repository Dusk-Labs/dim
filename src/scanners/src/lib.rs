extern crate rocket_slog;
#[macro_use]
extern crate slog;
extern crate clap;
extern crate crossbeam_channel;
extern crate diesel;
extern crate notify;
extern crate torrent_name_parser;

use dim_database::get_conn;
use rocket_slog::SyncLogger;
use std::thread;

pub mod api;
pub mod iterative_parser;
pub mod tmdb;

use crate::iterative_parser::start_iterative_parser;

pub fn start(library_id: i32, log: SyncLogger) -> std::result::Result<(), ()> {
    let mut threads = Vec::new();

    info!(log, "Scanning {}", library_id);
    if get_conn().is_ok() {
        let library_id_ref = library_id;
        threads.push(thread::spawn(move || {
            start_iterative_parser(library_id_ref, log);
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
