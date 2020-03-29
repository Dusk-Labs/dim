use crate::{routes, scanners};
use diesel::prelude::*;
use lazy_static::lazy_static;
use rocket::http::Method;
use rocket_contrib::databases::diesel;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket_slog::SlogFairing;
use slog::{error, Logger};
use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[database("dimpostgres")]
pub struct DbConnection(PgConnection);

impl AsRef<PgConnection> for DbConnection {
    fn as_ref(&self) -> &PgConnection {
        &*self
    }
}

pub type EventTx = mpsc::Sender<pushevent::Event>;

lazy_static! {
    /// Holds a map of all threads keyed against the library id that they were started for
    static ref LIB_SCANNERS: Mutex<HashMap<i32, thread::JoinHandle<()>>> =
        Mutex::new(HashMap::new());
}

/// Function dumps a list of all libraries in the database and starts a scanner for each which
/// monitors for new files using fsnotify. It also scans all orphans on boot.
///
/// # Arguments
/// * `log` - Logger to which to log shit
/// * `tx` - this is the websocket channel to which we can send websocket events to which get
/// dispatched to clients.
pub(crate) fn run_scanners(log: Logger, tx: EventTx) {
    if let Ok(conn) = database::get_conn_logged(&log) {
        for lib in database::library::Library::get_all(&conn) {
            slog::info!(log, "Starting scanner for {} with id: {}", lib.name, lib.id);
            let log_clone = log.clone();
            let library_id = lib.id;
            let tx_clone = tx.clone();

            // NOTE: Its a good idea to just let the binary panic if the LIB_SCANNERS cannot be
            //       locked. This is because such a error is unrecoverable.
            LIB_SCANNERS.lock().unwrap().insert(
                library_id,
                thread::spawn(move || {
                    let _ = scanners::start(library_id, &log_clone, tx_clone).map_err(|x| {
                        error!(
                            log_clone,
                            "A scanner thread has returned with error: {:?}", x
                        )
                    });
                }),
            );
        }
    }
}

/// Function spins up a new Websocket server which we use to dispatch events over to clients
/// discriminated by a URI
// TODO: Handle launch failures and fallback to a new port.
// TODO: Store the port of the server in a dynamic config which can be queried by clients in case
// the port changes as we dont want this hardcoded in.
pub(crate) fn start_event_server(_log: Logger, host: &'static str) -> EventTx {
    let server = pushevent::server::Server::new(host);
    server.get_tx()
}

pub fn rocket_pad(
    logger: slog::Logger,
    event_tx: EventTx,
    config: rocket::config::Config,
) -> rocket::Rocket {
    let fairing = SlogFairing::new(logger);

    // At the moment we dont really care if cors access is global so we create CORS options to
    // target every route.
    let allowed_origins = AllowedOrigins::all();
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Options,
            Method::Get,
            Method::Post,
            Method::Delete,
            Method::Patch,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::custom(config)
        .attach(DbConnection::fairing())
        .attach(fairing)
        .mount(
            "/",
            routes![routes::r#static::index, routes::r#static::dist_file,],
        )
        .mount(
            "/api/v1/",
            routes![
                routes::general::dashboard,
                routes::general::banners,
                routes::general::get_directory_structure,
                routes::general::get_root_directory_structure,
                routes::stream::return_manifest,
                routes::stream::return_static,
                routes::general::search,
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
            routes![routes::media::rematch_mediafile,],
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
            "/api/v1/auth",
            routes![
                routes::auth::login,
                routes::auth::register,
                routes::auth::whoami,
                routes::auth::admin_exists,
                routes::auth::get_all_invites,
                routes::auth::generate_invite
            ],
        )
        .attach(cors)
        .manage(Arc::new(Mutex::new(event_tx)))
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
pub fn launch(log: slog::Logger, event_tx: EventTx, config: rocket::config::Config) {
    rocket_pad(log, event_tx, config).launch();

    // Join all threads started by dim, which usually are scanner/daemon threads
    for (_, thread) in LIB_SCANNERS.lock().unwrap().drain().take(1) {
        thread.join().unwrap();
    }
}
