use crate::core::DbConnection;
use crate::database::episode::{Episode, InsertableEpisode, UpdateEpisode};
use crate::database::media::Media;
use crate::database::season::{InsertableSeason, Season, UpdateSeason};
use crate::database::tv::TVShow;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_tv_by_id(conn: DbConnection, id: i32) -> Result<Json<Media>, Status> {
    match TVShow::get(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<id>/season")]
pub fn get_tv_seasons(conn: DbConnection, id: i32) -> Result<Json<Vec<Season>>, Status> {
    match Season::get_all(&conn, id) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/<id>/season", format = "application/json", data = "<new_season>")]
pub fn post_season_to_tv(
    conn: DbConnection,
    id: i32,
    new_season: Json<InsertableSeason>,
) -> Result<Status, Status> {
    match new_season.new(&conn, id) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<id>/season/<season_num>")]
pub fn get_season_by_num(
    conn: DbConnection,
    id: i32,
    season_num: i32,
) -> Result<Json<Season>, Status> {
    match Season::get(&conn, id, season_num) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[patch(
    "/<id>/season/<season_num>",
    format = "application/json",
    data = "<data>"
)]
pub fn patch_season_by_num(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    data: Json<UpdateSeason>,
) -> Result<Status, Status> {
    match data.update(&conn, id, season_num) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

#[delete("/<id>/season/<season_num>")]
pub fn delete_season_by_num(
    conn: DbConnection,
    id: i32,
    season_num: i32,
) -> Result<Status, Status> {
    match Season::delete(&conn, id, season_num) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotModified),
    }
}

#[post(
    "/<id>/season/<season_num>/episode",
    format = "application/json",
    data = "<episode>"
)]
pub fn post_episode_to_season(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    episode: Json<InsertableEpisode>,
) -> Result<Status, Status> {
    match episode.insert(&conn, id, season_num) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotFound),
    }
}

// NOTE: might want to separate these into separate files

#[get("/<id>/season/<season_num>/episode/<ep_num>")]
pub fn get_episode_by_id(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    ep_num: i32,
) -> Result<Json<Episode>, Status> {
    match Episode::get(&conn, id, season_num, ep_num) {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::NotFound),
    }
}

#[patch(
    "/<id>/season/<season_num>/episode/<ep_num>",
    format = "application/json",
    data = "<episode>"
)]
pub fn patch_episode_by_id(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    ep_num: i32,
    episode: Json<UpdateEpisode>,
) -> Result<Status, Status> {
    match episode.update(&conn, id, season_num, ep_num) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

#[delete("/<id>/season/<season_num>/episode/<ep_num>")]
pub fn delete_episode_by_id(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    ep_num: i32,
) -> Result<Status, Status> {
    match Episode::delete(&conn, id, season_num, ep_num) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotModified),
    }
}
