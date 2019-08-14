extern crate clap;
extern crate crossbeam_channel;
extern crate notify;
extern crate torrent_name_parser;

use crate::api::MovieObject;
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
use dim_database::{Library};

pub mod api;
pub mod tmdb;

fn start(library_id: i32) -> Result<(), ()> {
}
