//! Dim is a media manager written in rust.
//! It uses Diesel as the ORM and rocket for the http/s server
//!
//! The project is split up into several crates:
//! * [`database`](database) - Holds all the database models including some frequently used db operations
//! * [`routes`](routes) - All of the routes that we expose over http are stored in there
//! * [`scanners`](scanners) - The filesystem scanner and daemon code is located here
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
#![allow(opaque_hidden_inferred_bound)]

use std::fs::create_dir_all;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

/// Module contains our core initialization logic.
pub mod core;
/// Module contains all the error definitions used in dim, and returned by the web-service.
pub mod errors;
/// Module contains our external api interfaces
pub mod external;
/// Contains the code for fetching assets like posters and stills.
pub mod fetcher;
/// Inspect api for Result type
pub mod inspect;
/// Contains our custom logger for rocket
pub mod logger;
/// Sqlite CDC implementation
pub mod reactor;
/// Contains all of the routes exposed by the webapi.
pub mod routes;
/// New generation scanner infrastructure.
pub mod scanner;
/// Contains the fairing which tracks streams across rest api
pub mod stream_tracking;
/// Contains all the logic needed for streaming and on-the-fly transcoding.
pub mod streaming;
#[cfg(test)]
mod tests;
/// Tree-like structure for representing directories of files.
pub mod tree;
/// Various utilities
pub mod utils;
/// Websocket related logic.
pub mod websocket;

pub use routes::settings::get_global_settings;
pub use routes::settings::init_global_settings;
pub use routes::settings::set_global_settings;
pub use routes::settings::GlobalSettings;

/// Function builds a logger drain that drains to a json file located in logs/ and also to stdout.
pub fn setup_logging(_debug: bool) {
    let _ = create_dir_all("logs");

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let log_appender = tracing_appender::rolling::daily("./logs", "dim-log.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(log_appender);

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(fmt::layer().json().with_writer(non_blocking_file));

    let _ = tracing::subscriber::set_global_default(subscriber);
}

#[cfg(test)]
pub fn setup_test_logging() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            fmt::layer()
                .with_span_events(fmt::format::FmtSpan::CLOSE | fmt::format::FmtSpan::NEW)
                .with_writer(tracing_subscriber::fmt::TestWriter::new()),
        );

    let _ = tracing::subscriber::set_global_default(subscriber);
}
