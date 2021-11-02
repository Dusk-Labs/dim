use std::fs::create_dir_all;
use std::path::PathBuf;
use std::time::Duration;

use tracing::error;
use tracing::info;
use xtra::spawn::Tokio;

use dim::build_logger;
use dim::core;
use dim::routes::settings::GlobalSettings;
use dim::streaming;

use structopt::StructOpt;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "Dim", about = "Dim, a media manager fueled by dark forces.")]
#[structopt(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
#[structopt(rename_all = "kebab")]
struct Args {
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();
    let _ = create_dir_all(dim::utils::ffpath("config"));

    let config_path = args
        .config
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or(dim::utils::ffpath("config/config.toml").to_string());

    // initialize global settings.
    dim::init_global_settings(Some(config_path)).expect("Failed to initialize global settings.");

    let global_settings = dim::get_global_settings();

    // never panics because we set a default value to metadata_dir
    let _ = create_dir_all(global_settings.metadata_dir.clone());

    // set our jwt secret key
    let settings_clone = global_settings.clone();
    let secret_key = global_settings.secret_key.unwrap_or_else(move || {
        let secret_key = auth::generate_key();
        dim::set_global_settings(GlobalSettings {
            secret_key: Some(secret_key),
            ..settings_clone
        })
        .expect("Failed to save JWT secret_key.");
        secret_key
    });

    auth::set_jwt_key(secret_key);

    core::METADATA_PATH
        .set(global_settings.metadata_dir.clone())
        .expect("Failed to set METADATA_PATH");

    let logger = build_logger(global_settings.verbose);

    {
        let failed = streaming::ffcheck()
            .into_iter()
            .fold(false, |failed, item| match item {
                Ok(stdout) => {
                    info!("{}", stdout);
                    failed
                }

                Err(program) => {
                    error!("Could not find: {}", program);
                    true
                }
            });

        if failed {
            std::process::exit(1);
        }
    }

    nightfall::profiles::profiles_init(crate::streaming::FFMPEG_BIN.to_string());

    let async_main = async move {
        dim::fetcher::tmdb_poster_fetcher().await;

        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();

        let stream_manager = nightfall::StateManager::new(
            &mut Tokio::Global,
            global_settings.cache_dir.clone(),
            crate::streaming::FFMPEG_BIN.to_string(),
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
            info!("Transposing scanners from the netherworld...");
            core::run_scanners(event_tx.clone()).await;
        }

        info!("Summoning Dim v{}...", structopt::clap::crate_version!());

        let rt = tokio::runtime::Handle::current();

        core::warp_core(event_tx, stream_manager, rt, global_settings.port, event_rx).await;
    };

    tokio::runtime::Runtime::new()
        .expect("Failed to create a tokio runtime.")
        .block_on(async_main);
}
