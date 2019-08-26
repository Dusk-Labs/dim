use crate::core::DbConnection;
use crate::dim_database::media::{Media, UpdateMedia};
use crate::dim_database::mediafile::MediaFile;
use crate::dim_database::genre::Genre;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/<id>")]
pub fn get_media_by_id(conn: DbConnection, id: i32) -> Result<JsonValue, Status> {
    let data = match Media::get(&conn, id) {
        Ok(data) => data,
        Err(_) => return Err(Status::NotFound),
    };

    let duration = match MediaFile::get_of_media(&conn, &data) {
        Ok(x) => x.duration.unwrap(),
        Err(_) => 0,
    };

    let genres = Genre::get_by_media(&conn, data.id)
        .unwrap()
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();

    Ok(json!({
        "id": data.id,
        "library_id": data.library_id,
        "name": data.name,
        "description": data.description,
        "rating": data.rating,
        "year": data.year,
        "added": data.added,
        "poster_path": data.poster_path,
        "backdrop_path": data.backdrop_path,
        "media_type": data.media_type,
        "genres": genres,
        "duration": duration
    }))
}

#[patch("/<id>", format = "application/json", data = "<data>")]
pub fn update_media_by_id(
    conn: DbConnection,
    id: i32,
    data: Json<UpdateMedia>,
) -> Result<Status, Status> {
    match data.update(&conn, id) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

#[delete("/<id>")]
pub fn delete_media_by_id(conn: DbConnection, id: i32) -> Result<Status, Status> {
    match Media::delete(&conn, id) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotFound),
    }
}
