use clap::App;
use clap::Arg;

use rocket::config::Config;
use rocket::config::LogLevel;
use rocket::config::TlsConfig;

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
            Arg::with_name("config")
                .long("config")
                .short("c")
                .takes_value(true)
                .help("Path to the dim configuration file. (default: `./config.json`)"),
        );

    let matches = matches.get_matches();

    // initialize global settings.
    dim::init_global_settings(matches.value_of("config").map(ToString::to_string))
        .expect("Failed to initialize global settings.");

    let global_settings = dim::get_global_settings();

    let logger = build_logger(global_settings.verbose);

    // never panics because we set a default value to metadata_dir
    let _ = create_dir_all(&global_settings.metadata_dir);

    core::METADATA_PATH
        .set(global_settings.metadata_dir.clone())
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
            core::tmdb_poster_fetcher(logger.clone()).await;

            info!(logger, "Starting the WS service on port 3012");
            let event_tx = core::start_event_server().await;

            let stream_manager = nightfall::StateManager::new(
                &mut Tokio::Global,
                global_settings.cache_dir.clone(),
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

            if !global_settings.quiet_boot {
                info!(logger, "Transposing scanners from the netherworld...");
                core::run_scanners(logger.clone(), event_tx.clone()).await;
            }

            let key = global_settings.priv_key.clone();
            let tls = global_settings
                .ssl_cert
                .clone()
                .and_then(|x| Some(TlsConfig::from_paths(x, key?)));

            if tls.is_some() {
                info!(logger, "Enabled SSL... Standby for launch");
            } else {
                info!(logger, "Disabling SSL");
            }

            let rocket_config = Config {
                tls,
                address: [0, 0, 0, 0].into(),
                port: 8000,
                log_level: LogLevel::Debug,
                ..Default::default()
            };

            info!(logger, "Summoning Dim v{}...", clap::crate_version!());
            core::launch(logger, event_tx, rocket_config, stream_manager, handle).await;
        });
}
