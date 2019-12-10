#![feature(rustc_private)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

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
use slog::Drain;
use slog_async::Async;
use slog_json::Json as slog_json_default;
use slog_term::{FullFormat, TermDecorator};
use std::fs::{create_dir, File};
use std::sync::Mutex;

pub mod core;
pub mod errors;
pub mod macros;
pub mod routes;
pub mod schema;
pub mod tests;

const VERSION: &str = "0.0.3";
const DESCRIPTION: &str = "Dim, a media manager fueled by dark forces.";

fn build_logger(_debug: bool) -> slog::Logger {
    let date_now = Utc::now();

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    let _ = create_dir("logs");
    let file = File::create(format!("logs/dim-log-{}.log", date_now.to_rfc3339()))
        .expect("Couldnt open log file");
    let json_drain = Mutex::new(slog_json_default::default(file)).map(slog::Fuse);

    return slog::Logger::root(slog::Duplicate::new(drain, json_drain).fuse(), slog::o!());
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
        );

    let matches = matches.get_matches();
    let debug = if cfg!(debug_assertions) {
        true
    } else {
        matches.is_present("debug")
    };

    let logger = build_logger(debug);

    {
        let mut bucket: Vec<Box<str>> = Vec::new();
        if let Err(why) = streamer::ffcheck(&mut bucket) {
            eprintln!("Could not find: {}", why);
            slog::error!(logger, "Could not find: {}", why);
            std::process::exit(1);
        }

        for item in bucket.iter() {
            slog::info!(logger, "\n{}", item);
        }
    }

    slog::info!(logger, "Starting the WS service on port 3012");
    let event_tx = core::start_event_server(logger.clone());

    slog::info!(logger, "Booting scanners up");
    core::run_scanners(logger.clone(), event_tx.clone());

    let rocket_config = ConfigBuilder::new(Environment::Development)
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

    slog::info!(logger, "Booting Dim... Standby...");
    core::launch(logger, event_tx, rocket_config);
}
