use crate::database::media::{InsertableMedia, UpdateMedia, Media};
use crate::core::DbConnection;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_media_by_id(conn: DbConnection, id: i32) -> Result<Json<Media>, Status> {
    match Media::get(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/", format = "application/json", data = "<data>")]
pub fn insert_media_by_lib_id(
    conn: DbConnection,
    data: Json<InsertableMedia>,
) -> Result<Status, Status> {
    match Media::new(&conn, data) {
        Ok(_) => Ok(Status::Created),
        Err(_) => Err(Status::NotFound),
    }
}

#[patch("/<id>", format = "application/json", data = "<data>")]
pub fn update_media_by_id(
    conn: DbConnection,
    id: i32,
    data: Json<UpdateMedia>
) -> Result<Status, Status> {
    match Media::update(&conn, id, data) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotFound)
    }
}
