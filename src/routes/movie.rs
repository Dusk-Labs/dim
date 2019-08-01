use crate::database::movie::{InsertableMovie, Movie};
use crate::database::media::Media;
use crate::core::DbConnection;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_movie_by_id(
    conn: DbConnection,
    id: i32
) -> Result<Json<Media>, Status> {
    match Movie::get(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/")]
pub fn get_movies(
    conn: DbConnection,
) -> Result<Json<Vec<Media>>, Status> {
    match Movie::get_all(&conn) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}
