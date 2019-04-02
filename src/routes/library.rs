use crate::database::library::{InsertableLibrary, Library};
use crate::database::media::Media;
use crate::core::DbConnection;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn library_get(conn: DbConnection) -> Json<Vec<Library>> {
    Library::get_all(&conn)
}

#[post("/", format = "application/json", data = "<new_library>")]
pub fn library_post(
    conn: DbConnection,
    new_library: Json<InsertableLibrary>,
) -> Result<Status, Status> {
    match Library::new(&conn, new_library) {
        Ok(_) => Ok(Status::Created),
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
pub fn get_all_library(conn: DbConnection, id: i32) -> Result<Json<Vec<Media>>, Status> {
    match Library::get(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}
