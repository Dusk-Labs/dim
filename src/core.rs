use crate::logger::RequestLogger;
use crate::routes;
use crate::scanners;
use crate::stream_tracking::StreamTracking;

use rocket::fairing::Fairing;
use rocket::http::Method;
use rocket_contrib::databases::diesel;
use rocket_contrib::helmet::SpaceHelmet;

use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;
use rocket_cors::CorsOptions;

use cfg_if::cfg_if;
use diesel::prelude::*;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

use slog::debug;
use slog::error;
use slog::info;
use slog::Logger;

use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub type StateManager = nightfall::StateManager;
pub type DbConnection = database::DbConnection;
pub type EventTx = UnboundedSender<String>;

/// Hacky type we use to implement clone on deref types.
#[derive(Clone, Debug)]
pub struct CloneOnDeref<T> {
    inner: T,
}

impl<T: Clone> CloneOnDeref<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> T {
        self.inner.clone()
    }
}

unsafe impl<T: Send> Send for CloneOnDeref<T> {}
unsafe impl<T: Clone> Sync for CloneOnDeref<T> {}

/// Path to where metadata is stored and should be fetched to.
pub static METADATA_PATH: OnceCell<String> = OnceCell::new();
// NOTE: While the sender is wrapped in a Mutex, we dont really care as wel copy the inner type at
// some point anyway.
/// Contains the tx channel over which we can send images to be cached locally.
pub static METADATA_FETCHER_TX: OnceCell<CloneOnDeref<UnboundedSender<String>>> = OnceCell::new();

/// Function dumps a list of all libraries in the database and starts a scanner for each which
/// monitors for new files using fsnotify. It also scans all orphans on boot.
///
/// # Arguments
/// * `log` - Logger to which to log shit
/// * `tx` - this is the websocket channel to which we can send websocket events to which get
/// dispatched to clients.
pub async fn run_scanners(log: Logger, tx: EventTx) {
    if let Ok(conn) = database::get_conn_logged(&log) {
        for lib in database::library::Library::get_all(&conn).await {
            slog::info!(log, "Starting scanner for {} with id: {}", lib.name, lib.id);
            let log_clone = log.clone();
            let library_id = lib.id;
            let tx_clone = tx.clone();
            let media_type = lib.media_type;

            tokio::spawn(scanners::start(library_id, log_clone.clone(), tx_clone));

            let log_clone = log.clone();
            let library_id = lib.id;
            let tx_clone = tx.clone();
            let media_type = lib.media_type;
            tokio::spawn(async move {
                let watcher = scanners::scanner_daemon::FsWatcher::new(
                    log_clone, library_id, media_type, tx_clone,
                );

                watcher
                    .start_daemon()
                    .await
                    .expect("Something went wrong with the fs-watcher");
            });
        }
    }
}

pub async fn tmdb_poster_fetcher(log: Logger) {
    let (tx, mut rx): (UnboundedSender<String>, UnboundedReceiver<String>) = unbounded_channel();

    tokio::spawn(async move {
        while let Some(url) = rx.recv().await {
            match reqwest::get(url.as_str()).await {
                Ok(resp) => {
                    if let Some(fname) = resp.url().path_segments().and_then(|segs| segs.last()) {
                        let meta_path = METADATA_PATH.get().unwrap();
                        let mut out_path = PathBuf::from(meta_path);
                        out_path.push(fname);

                        debug!(log, "Caching {} -> {:?}", url, out_path);

                        if let Ok(mut file) = File::create(out_path) {
                            if let Ok(bytes) = resp.bytes().await {
                                let mut content = Cursor::new(bytes);
                                if let Err(e) = copy(&mut content, &mut file) {
                                    error!(log, "Failed to cache {} locally, e={:?}", url, e);
                                }
                            }
                        }
                    }
                }
                Err(e) => error!(log, "Failed to cache {} locally, e={:?}", url, e),
            }
        }
    });

    METADATA_FETCHER_TX
        .set(CloneOnDeref::new(tx))
        .expect("Failed to set METADATA_FETCHER_TX");
}

/// Function spins up a new Websocket server which we use to dispatch events over to clients
/// discriminated by a URI
// TODO: Handle launch failures and fallback to a new port.
// TODO: Store the port of the server in a dynamic config which can be queried by clients in case
// the port changes as we dont want this hardcoded in.
pub async fn start_event_server() -> EventTx {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(crate::websocket::serve(
        "0.0.0.0:3012",
        tokio::runtime::Handle::current(),
        rx,
    ));

    tx
}

pub async fn rocket_pad(
    logger: slog::Logger,
    event_tx: EventTx,
    config: rocket::config::Config,
    stream_manager: StateManager,
    handle: tokio::runtime::Handle,
) -> rocket::Rocket<rocket::Build> {
    // At the moment we dont really care if cors access is global so we create CORS options to
    // target every route.
    let allowed_origins = AllowedOrigins::all();
    let cors = CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let stream_tracking = StreamTracking::default();

    rocket::custom(config)
        .attach(SpaceHelmet::default())
        .attach(cors)
        .attach(RequestLogger::new(logger.clone()))
        .register(
            "/api",
            catchers![
                routes::catchers::not_found,
                routes::catchers::internal_server_error
            ],
        )
        .mount(
            "/",
            routes![
                routes::statik::get_image,
                routes::statik::index_redirect,
                routes::statik::dist_asset,
                routes::statik::dist_static,
                routes::statik::react_routes,
            ],
        )
        .mount(
            "/api/v1/",
            routes![
                routes::dashboard::dashboard,
                routes::dashboard::banners,
                routes::general::get_directory_structure,
                routes::general::get_root_directory_structure,
                routes::general::search,
            ],
        )
        .mount(
            "/api/v1/stream",
            routes![
                routes::stream::return_manifest,
                routes::stream::get_chunk,
                routes::stream::get_init,
                routes::stream::get_subtitle,
                routes::stream::should_client_hard_seek,
                routes::stream::session_get_stderr,
                routes::stream::kill_session,
            ],
        )
        .mount(
            "/api/v1/library",
            routes![
                routes::library::library_get,
                routes::library::get_self,
                routes::library::library_post,
                routes::library::library_delete,
                routes::library::get_all_library,
                routes::library::get_all_unmatched_media,
            ],
        )
        .mount(
            "/api/v1/media",
            routes![
                routes::media::get_media_by_id,
                routes::media::get_extra_info_by_id,
                routes::media::update_media_by_id,
                routes::media::delete_media_by_id,
                routes::media::tmdb_search,
                routes::media::map_progress,
            ],
        )
        .mount(
            "/api/v1/mediafile",
            routes![
                routes::mediafile::get_mediafile_info,
                routes::mediafile::rematch_mediafile,
            ],
        )
        .mount(
            "/api/v1/tv",
            routes![
                routes::tv::get_tv_by_id,
                routes::tv::get_tv_seasons,
                routes::tv::get_season_by_num,
                routes::tv::patch_season_by_num,
                routes::tv::delete_season_by_num,
                routes::tv::get_episode_by_id,
                routes::tv::patch_episode_by_id,
                routes::tv::delete_episode_by_id,
            ],
        )
        .mount(
            "api/v1/auth",
            routes![
                routes::auth::login,
                routes::auth::register,
                routes::auth::whoami,
                routes::auth::admin_exists,
                routes::auth::get_all_invites,
                routes::auth::generate_invite
            ],
        )
        .mount(
            "/api/v1/user",
            routes![
                routes::settings::get_user_settings,
                routes::settings::post_user_settings,
            ],
        )
        .mount(
            "/api/v1/host",
            routes![
                routes::settings::http_get_global_settings,
                routes::settings::http_set_global_settings,
            ],
        )
        .manage(logger)
        .manage(Arc::new(Mutex::new(event_tx)))
        .manage(database::get_conn().expect("Failed to get db connection"))
        .manage(stream_tracking)
        .manage(stream_manager)
        .manage(handle)
}

/// Method launch
/// This method created a new rocket pad and launches it using the configuration passed in. This
/// function returns once the server has finished running and all the scanner threads have been
/// joined.
///
/// # Arguments
/// * `log` - a Logger object which will be propagated to subsequent modules which can use this as
///           a sink for logs.
/// * `event_tx` - This is the tx channel over which modules in dim can dispatch websocket events.
/// * `config` - Specifies the configuration we'd like to pass to our rocket_pad.
pub async fn launch(
    log: slog::Logger,
    event_tx: EventTx,
    config: rocket::config::Config,
    stream_manager: StateManager,
    handle: tokio::runtime::Handle,
) {
    let error = rocket_pad(log, event_tx, config, stream_manager, handle)
        .await
        .launch()
        .await;

    if let Err(e) = error {
        panic!("Launch error: {}", e);
    }

    println!("Good bye ;3");
}
