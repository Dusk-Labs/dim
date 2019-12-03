use crate::routes;
use diesel::prelude::*;
use lazy_static::lazy_static;
use rocket::http::Method;
use rocket_contrib::databases::diesel;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket_slog::SlogFairing;
use slog::Logger;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[database("openflix")]
pub struct DbConnection(PgConnection);

impl AsRef<PgConnection> for DbConnection {
    fn as_ref(&self) -> &PgConnection {
        &*self
    }
}

pub type EventTx = std::sync::mpsc::Sender<pushevent::Event>;

lazy_static! {
    static ref LIB_SCANNERS: Mutex<HashMap<i32, std::thread::JoinHandle<()>>> =
        Mutex::new(HashMap::new());
}

pub(crate) fn run_scanners(log: Logger, tx: EventTx) {
    if let Ok(conn) = database::get_conn() {
        for lib in database::library::Library::get_all(&conn) {
            slog::info!(log, "Starting scanner for {} with id: {}", lib.name, lib.id);
            let log_clone = log.clone();
            let library_id = lib.id;
            let tx_clone = tx.clone();
            LIB_SCANNERS.lock().unwrap().insert(
                library_id,
                std::thread::spawn(move || {
                    scanners::start(library_id, &log_clone, tx_clone).unwrap();
                }),
            );
        }
    }
}

pub(crate) fn start_event_server(_log: Logger) -> EventTx {
    let server = pushevent::server::Server::new("0.0.0.0:3012");
    server.get_tx()
}

fn rocket_pad(logger: slog::Logger, event_tx: EventTx) -> rocket::Rocket {
    let fairing = SlogFairing::new(logger);

    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .attach(DbConnection::fairing())
        .attach(fairing)
        .mount(
            "/api/v1/",
            routes![
                routes::general::dashboard,
                routes::general::banners,
                routes::general::get_directory_structure,
                routes::stream::start_stream,
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
                routes::library::get_all_library
            ],
        )
        .mount(
            "/api/v1/media",
            routes![
                routes::media::get_media_by_id,
                routes::media::get_extra_info_by_id,
                routes::media::update_media_by_id,
                routes::media::delete_media_by_id,
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
        .mount("/api/v1/auth", routes![routes::auth::login])
        .attach(cors)
        .manage(Arc::new(Mutex::new(event_tx)))
}

pub fn launch(log: slog::Logger, event_tx: EventTx) {
    rocket_pad(log, event_tx).launch();

    for (_, thread) in LIB_SCANNERS.lock().unwrap().drain().take(1) {
        thread.join().unwrap();
    }
}
