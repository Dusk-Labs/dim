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

#![feature(rustc_private, proc_macro_hygiene, decl_macro, try_trait)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rust_embed;

use chrono::Utc;
use clap::{App, Arg};
use rocket::config::{ConfigBuilder, Environment, LoggingLevel};
use slog::{error, info, o, warn, Drain, Duplicate, Fuse, Logger};
use slog_async::Async;
use slog_json::Json as slog_json_default;
use slog_term::{FullFormat, TermDecorator};
use std::{
    fs::{create_dir, File},
    process,
    sync::Mutex,
};

pub mod core;
pub mod errors;
pub mod macros;
pub mod routes;
pub mod scanners;
pub mod schema;
pub mod streaming;
pub mod tests;

const VERSION: &str = "0.0.4";
const DESCRIPTION: &str = "Dim, a media manager fueled by dark forces.";

/// Function builds a logger drain that drains to a json file located in logs/ and also to stdout.
fn build_logger(_debug: bool) -> slog::Logger {
    let date_now = Utc::now();

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    let _ = create_dir("logs");
    let file = File::create(format!("logs/dim-log-{}.log", date_now.to_rfc3339()))
        .expect("Couldnt open log file");

    let json_drain = Mutex::new(slog_json_default::default(file)).map(Fuse);

    Logger::root(Duplicate::new(drain, json_drain).fuse(), o!())
}

fn main() {
    let matches = App::new("Dim")
        .version(VERSION)
        .about(DESCRIPTION)
        .author("Valerian G.")
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
            Arg::with_name("no-scanners")
                .long("no-scan")
                .help("Disable the library scanners on boot"),
        );

    let matches = matches.get_matches();
    let debug = cfg!(debug_assertions) || matches.is_present("debug");
    let logger = build_logger(debug);

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

    info!(logger, "Starting the WS service on port 3012");
    let event_tx = core::start_event_server(logger.clone(), "0.0.0.0:3012");

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
        .workers(64)
        .log_level(LoggingLevel::Off)
        .extra("databases", {
            let mut db_conf = std::collections::HashMap::new();
            let mut m = std::collections::HashMap::new();
            m.insert("url", "postgres://postgres:dimpostgres@127.0.0.1/dim");
            db_conf.insert("dimpostgres", m);
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

    info!(logger, "Summoning Dim using the {} spell...", VERSION);
    core::launch(logger, event_tx, rocket_config);
}
