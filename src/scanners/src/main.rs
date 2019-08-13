extern crate clap;
extern crate crossbeam_channel;
extern crate notify;
extern crate torrent_name_parser;

use clap::{App, Arg};
use notify::DebouncedEvent::{Create, Remove, Rename};
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use torrent_name_parser::Metadata;
use walkdir::WalkDir;
pub mod api;
pub mod tmdb;
use crate::api::MovieObject;

fn main() -> Result<()> {
    let matches = App::new("Media scanner made for dim")
        .arg(
            Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("DIRECTORY")
            .help("Specifies the path to start a scan daemon on")
            .required(true)
            .takes_value(true),
            )
        .arg(
            Arg::with_name("endpoint")
            .short("e")
            .long("endpoint")
            .help("Specifies the domain to return data to")
            .required(true)
            .takes_value(true),
            )
        .arg(
            Arg::with_name("auth-token")
            .short("a")
            .long("auth")
            .help("Specifies authentication token to use when returning data")
            .required(true)
            .takes_value(true),
            )
        .arg(
            Arg::with_name("type")
            .short("t")
            .long("type")
            .help("Specifies type of media we are scanning for")
            .required(true)
            .takes_value(true),
            )
        .get_matches();

    let path = String::from(matches.value_of("path").unwrap());
    let endpoint: i32 = String::from(matches.value_of("endpoint").unwrap()).parse().unwrap();
    let auth = String::from(matches.value_of("auth-token").unwrap());
    let media_type = String::from(matches.value_of("type").unwrap());

    let path_copy = path.clone();
    let endpoint_copy = endpoint.clone();
    let auth_copy = auth.clone();
    let media_type_copy = media_type.clone();

    thread::spawn(move || {
        init_scan(path_copy, auth_copy, endpoint_copy, media_type_copy);
    });

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(100))?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => handle_event(event),
            Err(e) => println!("err {:?}", e),
        };
    }
}

fn handle_event(event: notify::DebouncedEvent) {
    match event {
        Create(path) => handle_create(path),
        Remove(path) => handle_remove(path),
        Rename(a, b) => handle_rename(a, b),
        _ => {}
    }
}

fn handle_create(path: std::path::PathBuf) {
    println!("Create: {:?}", path);
}

fn handle_remove(path: std::path::PathBuf) {
    println!("Remove: {:?}", path);
}

fn handle_rename(a: std::path::PathBuf, b: std::path::PathBuf) {
    println!("Rename from {:?} to {:?}", a, b);
}

fn init_scan(path: String, auth: String, api: i32, media_type: String) {
    match media_type.as_str() {
        "movie" => scan_movies(path, auth, api),
        "tv" => scan_tv(path, auth, api),
        _ => {}
    }
}

fn scan_movies(path: String, auth: String, api: i32) {
    let metadata = enumerate_dir(path, api);
    for mut m in metadata {
        m.metadata_scan().unwrap();
        m.media_fetch().unwrap();
    }
}

fn scan_tv(_: String, _: String, _: i32) {
    println!("tv");
}

fn enumerate_dir(path: String, lib_id: i32) -> Vec<MovieObject<'static>> {
    let list: Vec<MovieObject> = WalkDir::new(path.as_str())
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| x.into_path())
        .filter(|x| x.extension().is_some())
        .filter(|x| {
            let ext = x.extension().unwrap();
            ["mkv", "mp4", "avi"].contains(&ext.to_str().unwrap())
        })
        .map(|x| MovieObject::new(x.file_name().unwrap().to_str().unwrap().to_owned(), lib_id))
        .collect::<Vec<_>>();
    list
}
