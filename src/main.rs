use slog::error;
use slog::info;

use std::fs::create_dir_all;
use std::path::PathBuf;
use std::time::Duration;

use xtra::spawn::Tokio;

use dim::build_logger;
use dim::core;
use dim::streaming;

use structopt::StructOpt;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "Dim", about = "Dim, a media manager fueled by dark forces.")]
#[structopt(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
#[structopt(rename_all = "kebab")]
struct Args {
    #[structopt(short, long, parse(from_os_str), default_value = "config.json")]
    config: PathBuf,
}

fn main() {
    let args = Args::from_args();

    let logger = build_logger();

    // never panics because we set a default value to metadata_dir
    let _ = create_dir_all(args.metadata_dir.clone());

    core::METADATA_PATH
        .set(args.metadata_dir.to_string_lossy().to_string())
        .expect("Failed to set METADATA_PATH");

    // initialize global settings.
    dim::init_global_settings(Some(args.config.to_string_lossy().to_string()))
        .expect("Failed to initialize global settings.");

    let global_settings = dim::get_global_settings();

    let logger = build_logger(global_settings.verbose);

    // never panics because we set a default value to metadata_dir
    let _ = create_dir_all(&global_settings.metadata_dir);

    core::METADATA_PATH
        .set(global_settings.metadata_dir.clone());

    {
        let failed = streaming::ffcheck()
            .into_iter()
            .fold(false, |failed, item| match item {
                Ok(stdout) => {
                    info!(logger, "{}", stdout);
                    failed
                }

                Err(program) => {
                    error!(logger, "Could not find: {}", program);
                    true
                }
            });

        if failed {
            std::process::exit(1);
        }
    }

    let async_main = async move {
        core::fetcher::tmdb_poster_fetcher(logger.clone()).await;

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

        info!(logger, "Summoning Dim v{}...", structopt::clap::crate_version!());

        let rt = tokio::runtime::Handle::current();

        core::warp_core(
            logger,
            event_tx,
            stream_manager,
            rt,
            args.port
        )
        .await;
    };

    tokio::runtime::Runtime::new()
        .expect("Failed to create a tokio runtime.")
        .block_on(async_main);
}
