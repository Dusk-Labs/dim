extern crate clap;
extern crate crossbeam_channel;
extern crate notify;
extern crate torrent_name_parser;
extern crate diesel;

use std::thread;
use dim_database::get_conn;

pub mod api;
pub mod tmdb;
pub mod iterative_parser;

use crate::iterative_parser::start_iterative_parser;

pub fn start(library_id: i32) -> std::result::Result<(), &'static str> {
    let mut threads = Vec::new();

    println!("Scanning {}", library_id);
    if let Ok(_) = get_conn() {
        let library_id_ref = library_id.clone();
        threads.push(thread::spawn(move || {
            start_iterative_parser(library_id_ref);
        }));
    } else {
        println!("[SCANNERS] Failed to connect to db");
        return Err("[SCANNERS] Failed to connect to db")
    }
    
    for t in threads {
        t.join();
    }

    Ok(())
}
