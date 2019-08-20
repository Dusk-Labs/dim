use crate::core::DbConnection;
use dim_database::library::{InsertableLibrary, Library};
use dim_database::media::Media;
use rocket::http::Status;
use rocket_contrib::json::Json;
use std::thread;
use dim_scanners;

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
            println!("Starting scanner thread");
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

#[get("/<id>/media")]
pub fn get_all_library(conn: DbConnection, id: i32) -> Result<Json<Vec<Media>>, Status> {
    match Library::get(&conn, id) {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::NotFound),
    }
}
