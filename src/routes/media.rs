use crate::core::DbConnection;
use crate::errors;
use auth::Wrapper as Auth;
use database::genre::Genre;
use database::media::{Media, UpdateMedia};
use database::mediafile::MediaFile;
use rocket::http::Status;
use rocket_contrib::{
    json,
    json::{Json, JsonValue},
};

#[get("/<id>")]
pub fn get_media_by_id(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let data = Media::get(conn.as_ref(), id)?;

    let duration = match MediaFile::get_of_media(conn.as_ref(), &data) {
        Ok(mut x) => x.pop()?.duration?,
        Err(_) => 0,
    };

    let genres = Genre::get_by_media(conn.as_ref(), data.id)?
        .into_iter()
        .map(|x| x.name)
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

#[get("/<id>/info")]
pub fn get_extra_info_by_id(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let media = Media::get(conn.as_ref(), id)?;

    let media_files = MediaFile::get_of_media(conn.as_ref(), &media)?;

    Ok(json!({
        "versions": media_files.iter().map(|x| json!({
            "file": x.target_file,
            "display_name": format!("{} - {} - {} - Library {}",
                                    x.codec.as_ref().unwrap_or(&"Unknown VC".to_string()),
                                    x.audio.as_ref().unwrap_or(&"Unknwon AC".to_string()),
                                    x.original_resolution.as_ref().unwrap_or(&"Unknown res".to_string()),
                                    x.library_id)
        })).collect::<Vec<_>>(),
        "cast": [],
        "directors": []
    }))
}

#[patch("/<id>", format = "application/json", data = "<data>")]
pub fn update_media_by_id(
    conn: DbConnection,
    id: i32,
    data: Json<UpdateMedia>,
    _user: Auth,
) -> Result<Status, Status> {
    match data.update(conn.as_ref(), id) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

#[delete("/<id>")]
pub fn delete_media_by_id(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    Media::delete(conn.as_ref(), id)?;
    Ok(Status::Ok)
}
