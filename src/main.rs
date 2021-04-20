use clap::App;
use clap::Arg;

use rocket::config::ConfigBuilder;
use rocket::config::Environment;
use rocket::config::LoggingLevel;

use slog::error;
use slog::info;
use slog::warn;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::process;
use std::time::Duration;

use xtra::spawn::Tokio;

use dim::bootstrap;
use dim::build_logger;
use dim::core;
use dim::streaming;

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
        &mut Tokio::Handle(&tokio_rt),
        matches.value_of("cache-dir").unwrap().to_string(),
        crate::streaming::FFMPEG_BIN.to_string(),
        logger.clone(),
    );

    let stream_manager_clone = stream_manager.clone();

    // GC the stream manager every 100ms
    tokio_rt.spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        interval.tick().await;

        loop {
            interval.tick().await;
            let _ = stream_manager_clone.garbage_collect().await.unwrap();
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
            cfg_if::cfg_if! {
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
