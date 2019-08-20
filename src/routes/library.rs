use crate::core::DbConnection;
use dim_database::library::{InsertableLibrary, Library};
use dim_database::media::Media;
use rocket::http::Status;
use rocket_contrib::json::Json;
use dim_scanners;
use std::collections::HashMap;

#[get("/")]
pub fn library_get(conn: DbConnection) -> Json<Vec<Library>> {
    Json(Library::get_all(&conn))
}

#[post("/", format = "application/json", data = "<new_library>")]
pub fn library_post(
    conn: DbConnection,
    new_library: Json<InsertableLibrary>,
) -> Result<Status, Status> {
    match new_library.new(&conn) {
        Ok(id) => {
            std::thread::spawn(move || {
                dim_scanners::start(id).unwrap();
            });
            Ok(Status::Created)
        },
        Err(_) => Err(Status::NotImplemented),
    }
}

#[delete("/<id>")]
pub fn library_delete(conn: DbConnection, id: i32) -> Result<Status, Status> {
    match Library::delete(&conn, id) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<id>")]
pub fn get_self(conn: DbConnection, id: i32) -> Result<Json<Library>, Status> {
    match Library::get_one(&conn, id) {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<id>/media")]
pub fn get_all_library(conn: DbConnection, id: i32) -> Result<Json<HashMap<String, Vec<Media>>>, Status> {
    let mut result: HashMap<String, Vec<Media>> = HashMap::new();
    if let Ok(lib) = Library::get_one(&conn, id) {
        match Library::get(&conn, id) {
            Ok(data) => {
                result.insert(lib.name, data);
                Ok(Json(result))
            },
            Err(_) => Err(Status::NotFound),
        }
    } else {
        Err(Status::NotFound)
    }
}
