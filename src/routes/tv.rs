use crate::core::DbConnection;
use dim_database::episode::{Episode, UpdateEpisode};
use dim_database::media::Media;
use dim_database::season::{Season, UpdateSeason};
use dim_database::tv::TVShow;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_tv_by_id(conn: DbConnection, id: i32) -> Result<Json<Media>, Status> {
    match TVShow::get(&conn, id) {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<id>/season")]
pub fn get_tv_seasons(conn: DbConnection, id: i32) -> Result<Json<Vec<Season>>, Status> {
    match Season::get_all(&conn, id) {
        Ok(data) => Ok(Json(data)),
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
        Ok(data) => Ok(Json(data)),
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

#[get("/<id>/season/<season_num>/episode/<ep_num>")]
pub fn get_episode_by_id(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    ep_num: i32,
) -> Result<Json<Episode>, Status> {
    match Episode::get(&conn, id, season_num, ep_num) {
        Ok(data) => Ok(Json(data)),
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
