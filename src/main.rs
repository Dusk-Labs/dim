use clap::App;
use clap::Arg;

/*
use rocket::config::Config;
use rocket::config::LogLevel;
use rocket::config::TlsConfig;
*/

use slog::error;
use slog::info;

use std::fs::create_dir_all;
use std::process;
use std::time::Duration;

use xtra::spawn::Tokio;

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
    let logger = build_logger();

    // never panics because we set a default value to metadata_dir
    let meta_dir = matches.value_of("metadata-dir").unwrap();
    let _ = create_dir_all(meta_dir);

    core::METADATA_PATH
        .set(meta_dir.to_owned())
        .expect("Failed to set METADATA_PATH");

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

    let rt = tokio::runtime::Runtime::new().expect("Failed to start the runtime for block_ons.");
    let handle = rt.handle().clone();

    tokio::runtime::Runtime::new()
        .expect("Failed to create a tokio runtime.")
        .block_on(async move {
            core::fetcher::tmdb_poster_fetcher(logger.clone()).await;

            info!(logger, "Starting the WS service on port 3012");
            let event_tx = core::start_event_server().await;

            let stream_manager = nightfall::StateManager::new(
                &mut Tokio::Global,
                matches.value_of("cache-dir").unwrap().to_string(),
                crate::streaming::FFMPEG_BIN.to_string(),
                logger.clone(),
            );

            let stream_manager_clone = stream_manager.clone();

            // GC the stream manager every 100ms
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(1000));
                interval.tick().await;

                loop {
                    interval.tick().await;
                    let _ = stream_manager_clone.garbage_collect().await.unwrap();
                }
            });

            if !matches.is_present("no-scanners") {
                info!(logger, "Transposing scanners from the netherworld...");
                core::run_scanners(logger.clone(), event_tx.clone()).await;
            }

            let key = matches.value_of("priv-key").map(ToString::to_string);
            /*
            let tls = matches
                .value_of("ssl-cert")
                .map(ToString::to_string)
                .and_then(|x| Some(TlsConfig::from_paths(x, key?)));
            */

            /*
            if tls.is_some() {
                info!(logger, "Enabled SSL... Standby for launch");
            } else {
                info!(logger, "Disabling SSL");
            }
            */

            /*
            let rocket_config = Config {
                tls,
                address: [0, 0, 0, 0].into(),
                port: 8000,
                log_level: LogLevel::Normal,
                ..Default::default()
            };
            */

            info!(logger, "Summoning Dim v{}...", clap::crate_version!());
            core::warp_core(logger, event_tx, stream_manager).await;
        });
}
