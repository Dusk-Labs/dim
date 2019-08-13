use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::Request;
use rocket::Rocket;
use rocket_contrib::json::JsonValue;
use rocket_cors;
use rocket::http::Method;
use std::collections::HashMap;
use std::sync::Mutex;
use std::process::Command;
use std::process::Child;
use std::env::current_exe;

#[allow(unused_imports)]
use crate::routes;

embed_migrations!();

#[database("openflix")]
pub struct DbConnection(SqliteConnection);

lazy_static! {
    static ref SCANNERS: Mutex<HashMap<u32, Child>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub enum MediaType {
    Movie,
    TV,
}

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

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConnection::get_one(&rocket).expect("Database Connection Failed");
    let pragma = conn.execute("PRAGMA foreign_keys = ON");
    println!("Enable foreign key result {:?}", pragma);
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

pub fn spawn_scanner(id: u32, m_type: MediaType, path: String) -> Result<(), ()> {
    let scan_type = match m_type {
        MediaType::Movie => "movie",
        MediaType::TV => "tv",
    };

    let exe = match current_exe() {
        Ok(mut x) => { x.set_file_name("scanner"); x },
        Err(err) => { panic!("SpawnScanner panic'd with {:?}", err); },
    };

    let child = Command::new(exe.to_str().unwrap())
        .arg("--type")
        .arg(scan_type)
        .arg("--path")
        .arg(path)
        .arg("-e")
        .arg(id.to_string())
        .arg("--auth")
        .arg("token")
        .spawn()
        .expect("failed to execute child");
    
    SCANNERS.lock().unwrap().insert(id, child);

    Ok(())
}

pub fn stop_scanner(id: u32) -> Result<(), ()> {
    let mut lock = SCANNERS.lock().unwrap();
    let child = match lock.get_mut(&id) {
        Some(x) => x,
        None => { return Ok(()) }
    };

    match (*child).kill() {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn rocket() -> Rocket {
    let allowed_origins = rocket_cors::AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Patch].into_iter().map(From::from).collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();


    rocket::ignite()
        .attach(DbConnection::fairing())
        .attach(AdHoc::on_attach(
            "Running Database Migrations",
            run_db_migrations,
        ))
        .register(catchers![
            service_not_found,
            service_not_available,
            unprocessable_entity
        ])
        .mount(
            "/api/v1/library",
            routes![
                routes::library::library_get,
                routes::library::library_post,
                routes::library::library_delete,
                routes::library::get_all_library
            ],
        )
        .mount(
            "/api/v1/media",
            routes![
                routes::media::get_media_by_id,
                routes::media::insert_media_by_lib_id,
                routes::media::update_media_by_id,
                routes::media::delete_media_by_id,
            ],
        )
        .mount(
            "/api/v1/tv",
            routes![
                routes::tv::get_tv_by_id,
                routes::tv::get_tv_seasons,
                routes::tv::post_season_to_tv,
                routes::tv::get_season_by_num,
                routes::tv::patch_season_by_num,
                routes::tv::delete_season_by_num,
                routes::tv::post_episode_to_season,
                routes::tv::get_episode_by_id,
                routes::tv::patch_episode_by_id,
                routes::tv::delete_episode_by_id,
            ],
        )
        .attach(cors)
}
