//! Dim is a media manager written in rust.
//! It uses Diesel as the ORM and rocket for the http/s server
//!
//! The project is split up into several crates:
//! * [`auth`](auth) - Holds all the auth stuff that we might need
//! * [`database`](database) - Holds all the database models including some frequently used db operations
//! * [`events`](events) - Holds the events that we can dispatch over a websocket connection
//! * [`routes`](routes) - All of the routes that we expose over http are stored in there
//! * [`scanners`](scanners) - The filesystem scanner and daemon code is located here
//! * [`streaming`](streamer) - All streaming code is located here, including some wrappers around ffprobe and
//! ffmpeg that is used by several parts of dim
//!
//! # Building
//! Dim can easily be built with cargo build --release.
//! When built with --release, build.rs will compile the web ui and embed it into dim.
//!
//! # To run
//! Dim can be ran using docker, by pulling vgarleanu/dim-server, or locally.
//! If ran locally, make sure PostgreSQL is running with the password for postgres: dimpostgres
//!
//! # Testing
//! To test run `make test` in the root, or `cargo test` in the root of each module including the
//! root dir.
#![feature(
    proc_macro_hygiene,
    decl_macro,
    result_flattening,
    once_cell,
    type_ascription,
    result_into_ok_or_err,
    stmt_expr_attributes,
    with_options,
    map_first_last
)]
#![cfg_attr(debug_assertions, allow(unused_variables, unused_imports, dead_code))]

use cfg_if::cfg_if;
use chrono::Utc;

use slog::error;
use slog::info;
use slog::o;
use slog::warn;

use slog::Drain;
use slog::Duplicate;
use slog::Fuse;
use slog::Logger;

use slog_async::Async;
use slog_json::Json as slog_json_default;
use slog_term::FullFormat;
use slog_term::TermDecorator;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::File;
use std::process;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use xtra::spawn::Tokio;
use xtra::Actor;

/// Module contains our core initialization logic.
pub mod core;
/// Module contains all the error definitions used in dim, and returned by the web-service.
pub mod errors;
/// Contains our custom logger for rocket
pub mod logger;
/// Contains all of the routes exposed by the webapi.
pub mod routes;
/// Contains our media scanners and so on.
pub mod scanners;
/// Contains the fairing which tracks streams across rest api
pub mod stream_tracking;
/// Contains all the logic needed for streaming and on-the-fly transcoding.
pub mod streaming;
/// Various utilities
pub mod utils;
/// Websocket related logic.
pub mod websocket;

pub use routes::settings::get_global_settings;
pub use routes::settings::init_global_settings;
pub use routes::settings::set_global_settings;
pub use routes::settings::GlobalSettings;

/// Function builds a logger drain that drains to a json file located in logs/ and also to stdout.
pub fn build_logger(debug: bool) -> slog::Logger {
    let date_now = Utc::now();

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator)
        .use_original_order()
        .build()
        .fuse();

    let drain = Async::new(drain)
        .chan_size(2048)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .build()
        .fuse();

    let _ = create_dir_all("logs");

    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let file = File::create("./logs/dim-log.log")
                .expect("Couldnt open log file");
        } else {
            let file = File::create(format!("./logs/dim-log-{}.log", date_now.to_rfc3339()))
                .expect("Couldnt open log file");
        }
    }

    let json_drain = Async::new(slog_json_default::default(file).fuse())
        .chan_size(2048)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .build()
        .fuse();

    Logger::root(Duplicate::new(drain, json_drain).fuse(), o!())
}
