use diesel::prelude::*;
use lazy_static::lazy_static;
use rocket::http::Method;
use rocket::Request;
use rocket_contrib::databases::diesel;
use rocket_contrib::{json, json::JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket_slog::SlogFairing;
use slog::Logger;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use crate::routes;

#[database("openflix")]
pub struct DbConnection(PgConnection);

#[catch(503)]
fn service_not_available(_req: &Request) -> JsonValue {
    json!({
        "type": 503,
        "error": "Database is down"
    })
}

#[catch(404)]
fn service_not_found(_req: &Request) -> JsonValue {
    json!({
        "type": 404,
        "error": "Endpoint not found"
    })
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!({
        "type": 422,
        "error": "Invalid json supplied"
    })
}

lazy_static! {
    static ref LIB_SCANNERS: Mutex<HashMap<i32, std::thread::JoinHandle<()>>> =
        Mutex::new(HashMap::new());
}

pub type EventTx = std::sync::mpsc::Sender<pushevent::Event>;
fn run_scanners(log: Logger, tx: EventTx) {
    if let Ok(conn) = database::get_conn() {
        for lib in database::library::Library::get_all(&conn) {
            slog::slog_info!(log, "Starting scanner for {} with id: {}", lib.name, lib.id);
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

fn start_event_server(_log: Logger) -> EventTx {
    let server = pushevent::server::Server::new("0.0.0.0:3012");
    server.get_tx()
}

/**
fn set_panic_hook(logger: slog::Logger) {
    use slog::error;
    use std::panic;

    panic::set_hook(Box::new(|x| {
        if let Some(s) = x.payload().downcast_ref::<&str>() {
            error!(logger, "Panicd!!! {:?}", s);
        }
        println!("!!! DIM HAS PANIC'd, PLEASE PROVIDE LOG TO DEVELOPER !!!");
    }));
}
**/

fn build_logger(debug: bool) -> slog::Logger {
    use chrono::Utc;
    use slog::Drain;
    use slog_async::Async;
    use slog_json::Json as slog_json_default;
    use slog_term::{FullFormat, TermDecorator};
    use std::fs::{create_dir, File};
    let date_now = Utc::now();

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    let _ = create_dir("logs");
    let file = File::create(format!("logs/dim-log-{}.log", date_now.to_rfc3339()))
        .expect("Couldnt open log file");
    let json_drain = Mutex::new(slog_json_default::default(file)).map(slog::Fuse);

    if debug {
        return slog::Logger::root(slog::Duplicate::new(drain, json_drain).fuse(), slog::o!());
    }

    slog::Logger::root(json_drain, slog::o!())
}

pub(crate) fn rocket_pad(debug: bool) -> rocket::Rocket {
    let logger = build_logger(debug);

    let event_tx = start_event_server(logger.clone());
    run_scanners(logger.clone(), event_tx.clone());

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
        .register(catchers![
            service_not_found,
            service_not_available,
            unprocessable_entity
        ])
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

pub fn launch(debug: bool) {
    rocket_pad(debug).launch();

    for (_, thread) in LIB_SCANNERS.lock().unwrap().drain().take(1) {
        thread.join().unwrap();
    }
}
