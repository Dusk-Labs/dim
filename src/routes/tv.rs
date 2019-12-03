use crate::core::DbConnection;
use crate::errors;
use auth::Wrapper as Auth;
use database::episode::{Episode, UpdateEpisode};
use database::media::Media;
use database::season::{Season, UpdateSeason};
use database::tv::TVShow;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_tv_by_id(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<Json<Media>, errors::DimError> {
    Ok(Json(TVShow::get(conn.as_ref(), id)?))
}

#[get("/<id>/season")]
pub fn get_tv_seasons(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<Json<Vec<Season>>, errors::DimError> {
    Ok(Json(Season::get_all(conn.as_ref(), id)?))
}

#[get("/<id>/season/<season_num>")]
pub fn get_season_by_num(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    _user: Auth,
) -> Result<Json<Season>, errors::DimError> {
    Ok(Json(Season::get(conn.as_ref(), id, season_num)?))
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
    _user: Auth,
) -> Result<Status, errors::DimError> {
    data.update(conn.as_ref(), id, season_num)?;
    Ok(Status::NoContent)
}

#[delete("/<id>/season/<season_num>")]
pub fn delete_season_by_num(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    Season::delete(conn.as_ref(), id, season_num)?;
    Ok(Status::Ok)
}

#[get("/<id>/season/<season_num>/episode/<ep_num>")]
pub fn get_episode_by_id(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    ep_num: i32,
    _user: Auth,
) -> Result<Json<Episode>, errors::DimError> {
    Ok(Json(Episode::get(conn.as_ref(), id, season_num, ep_num)?))
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
    _user: Auth,
) -> Result<Status, errors::DimError> {
    episode.update(conn.as_ref(), id, season_num, ep_num)?;
    Ok(Status::NoContent)
}

#[delete("/<id>/season/<season_num>/episode/<ep_num>")]
pub fn delete_episode_by_id(
    conn: DbConnection,
    id: i32,
    season_num: i32,
    ep_num: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    Episode::delete(conn.as_ref(), id, season_num, ep_num)?;
    Ok(Status::Ok)
}
