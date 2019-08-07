use crate::core::DbConnection;
use crate::database::media::Media;
use crate::database::tv::TVShow;
//use crate::database::episode::{Episode, InsertableEpisode, UpdateEpisode};
//use crate::database::season::{Season, InsertableSeason, UpdateSeason};
use crate::database::season::{Season, InsertableSeason};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/<id>")]
pub fn get_tv_by_id(
    conn: DbConnection,
    id: i32
) -> Result<Json<Media>, Status> {
    match TVShow::get(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<id>/season")]
pub fn get_tv_seasons(
    conn: DbConnection,
    id: i32
) -> Result<Json<Vec<Season>>, Status> {
    match Season::get_all(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/<id>/season", format = "application/json", data = "<new_season>")]
pub fn post_season_to_tv(
    conn: DbConnection,
    id: i32,
    new_season: Json<InsertableSeason>
) -> Result<Status, Status> {
    match new_season.new(&conn, id) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<id>/season/<season_num>")]
pub fn get_season_by_id(
    conn: DbConnection,
    id: i32,
    season_num: i32
) -> Result<Json<Season>, Status> {
    match Season::get(&conn, id, season_num) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

/*
#[patch("/<id>", format = "application/json", data = "<data>")]
pub fn patch_season_by_id(
    conn: DbConnection,
    id: i32,
    data: Json<UpdateSeason>
) -> Result<Status, Status> {
    match data.update(&conn, id) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

#[delete("/<id>")]
pub fn delete_season_by_id(
    conn: DbConnection,
    id: i32
) -> Result<Status, Status> {
    match Season::delete(&conn, id) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotModified),
    }
}

#[post("/<id>/episode", format = "application/json", data = "<episode>")]
pub fn post_episode_to_season(
    conn: DbConnection,
    id: i32,
    episode: Json<InsertableEpisode>
) -> Result<JsonValue, Status> {
    match episode.insert(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

// /api/v1/episode endpoint
// NOTE: might want to separate these into separate files

#[get("/<id>")]
pub fn get_episode_by_id(
    conn: DbConnection,
    id: i32
) -> Result<Json<Episode>, Status> {
    match Episode::get(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[patch("/<id>", format = "application/json", data = "<episode>")]
pub fn patch_episode_by_id(
    conn: DbConnection,
    id: i32,
    episode: Json<UpdateEpisode>
) -> Result<Status, Status> {
    match epsiode.update(&conn, id) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

#[delete("/<id>")]
pub fn delete_episode_by_id(
    conn: DbConnection,
    id: i32
) -> Result<Status, Status> {
    match Episode::delete(&conn, id) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotModified),
    }
}*/
