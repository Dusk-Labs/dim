#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate serde;

extern crate dotenv;
extern crate rocket;

pub mod database;
pub mod routes;
pub mod schema;
#[macro_use]
pub mod macros;

use crate::database::media;
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::Request;
use rocket::Rocket;
use rocket_contrib::json::JsonValue;

#[allow(unused_imports)]
use routes::*;

embed_migrations!();

#[database("openflix")]
pub struct DbConnection(SqliteConnection);

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
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> Rocket {
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
            ],
        )
}

fn main() {
    rocket().launch();
}
