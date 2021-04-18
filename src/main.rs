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
    rustc_private,
    proc_macro_hygiene,
    decl_macro,
    try_trait,
    negative_impls,
    result_flattening
)]
#![forbid(missing_docs)]
#![cfg_attr(debug_assertions, allow(unused_variables, unused_imports, dead_code))]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rust_embed;

use cfg_if::cfg_if;
use chrono::Utc;

use clap::App;
use clap::Arg;

use rocket::config::ConfigBuilder;
use rocket::config::Environment;
use rocket::config::LoggingLevel;

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

/// Module contains a lot of the bootstrapping code that we use on first run of dim.
mod bootstrap;
/// Module contains our core initialization logic.
mod core;
/// Module contains all the error definitions used in dim, and returned by the web-service.
mod errors;
/// Contains all of the routes exposed by the webapi.
mod routes;
/// Contains our media scanners and so on.
mod scanners;
/// Contains the fairing which tracks streams across rest api
mod stream_tracking;
/// Contains all the logic needed for streaming and on-the-fly transcoding.
mod streaming;
/// Contains unit tests.
mod tests;
/// Websocket related logic.
mod websocket;

/// Function builds a logger drain that drains to a json file located in logs/ and also to stdout.
fn build_logger(_debug: bool) -> slog::Logger {
    let date_now = Utc::now();

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

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

    let json_drain = Mutex::new(slog_json_default::default(file)).map(Fuse);

    Logger::root(Duplicate::new(drain, json_drain).fuse(), o!())
}

fn main() {
    let matches = App::new("Dim")
        .version(clap::crate_version!())
        .about("Dim, a media manager fueled by dark forces.")
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debug mode? Print all logs to stdout"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Specify the port to use for the HTTP/S service"),
        )
        .arg(
            Arg::with_name("priv-key")
                .takes_value(true)
                .long("priv-key")
                .help("Path to the private key to use with the ssl module"),
        )
        .arg(
            Arg::with_name("ssl-cert")
                .takes_value(true)
                .long("ssl-cert")
                .help("Path to the SSL certificate we want to use"),
        )
        .arg(
            Arg::with_name("cache-dir")
                .takes_value(true)
                .long("cache-dir")
                .default_value("/tmp/streaming_cache")
                .help("Path where all transcoder data is kept"),
        )
        .arg(
            Arg::with_name("metadata-dir")
                .takes_value(true)
                .long("metadata-dir")
                .default_value("./metadata")
                .help("Path where all metadata is kept, such as posters and backdrops"),
        )
        .arg(
            Arg::with_name("no-scanners")
                .long("no-scan")
                .help("Disable the library scanners on boot"),
        );

    let matches = matches.get_matches();
    let debug = cfg!(debug_assertions) || matches.is_present("debug");
    let logger = build_logger(debug);

    bootstrap::bootstrap(logger.clone());

    // never panics because we set a default value to metadata_dir
    let meta_dir = matches.value_of("metadata-dir").unwrap();
    let _ = create_dir_all(meta_dir);

    core::METADATA_PATH
        .set(meta_dir.to_owned())
        .expect("Failed to set METADATA_PATH");

    core::tmdb_poster_fetcher(logger.clone());

    {
        // We check if ffmpeg and ffprobe binaries exist and exit gracefully if they dont exist.
        let mut bucket: Vec<Box<str>> = Vec::new();
        if let Err(why) = streaming::ffcheck(&mut bucket) {
            eprintln!("Could not find: {}", why);
            error!(logger, "Could not find: {}", why);
            process::exit(1);
        }

        for item in bucket.iter() {
            info!(logger, "\n{}", item);
        }
    }

    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    info!(logger, "Starting the WS service on port 3012");
    let event_tx = tokio_rt.block_on(core::start_event_server());

    let stream_manager = nightfall::StateManager::new(
        matches.value_of("cache-dir").unwrap().to_string(),
        crate::streaming::FFMPEG_BIN.to_string(),
        logger.clone()
    )
    .create(None)
    .spawn(&mut Tokio::Handle(&tokio_rt));

    let stream_manager_clone = stream_manager.clone();

    // GC the stream manager every 100ms
    tokio_rt.spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        interval.tick().await;

        loop {
            interval.tick().await;
            let _ = stream_manager_clone
                .send(nightfall::GarbageCollect)
                .await
                .expect("The Stream manager has crashed.");
        }
    });

    if !matches.is_present("no-scanners") {
        info!(logger, "Transposing scanners from the netherworld...");
        core::run_scanners(logger.clone(), event_tx.clone());
    }

    // NOTE: By default rocket starts on port 8000, maybe we should have it be default 8000 but accept a
    // custom port over cmd args?
    let env = if cfg!(debug_assertions) {
        Environment::Development
    } else {
        Environment::Production
    };

    let mut rocket_config = ConfigBuilder::new(env)
        .address("0.0.0.0")
        .port(8000)
        .log_level(LoggingLevel::Off)
        .extra("databases", {
            let mut db_conf = HashMap::new();
            let mut m = HashMap::new();
            cfg_if! {
                if #[cfg(feature = "postgres")] {
                    m.insert("url", "postgres://postgres:dimpostgres@127.0.0.1/dim");
                    db_conf.insert("dimpostgres", m);
                } else {
                    m.insert("url", "./dim.db");
                    db_conf.insert("dimpostgres", m);
                }
            }
            db_conf
        })
        .finalize()
        .unwrap();

    if let Some(cert) = matches.value_of("ssl-cert") {
        if let Some(key) = matches.value_of("priv-key") {
            rocket_config.set_tls(cert, key).map_or_else(
                |e| info!(logger, "Disabling SSL because {:?}", e),
                |_| info!(logger, "Enabled SSL... Standby for launch"),
            );
        }
    } else {
        warn!(logger, "Disabling SSL explicitly...");
    }

    info!(logger, "Summoning Dim v{}...", clap::crate_version!());
    core::launch(
        logger,
        event_tx,
        rocket_config,
        stream_manager,
        tokio_rt.handle().clone(),
    );
}
