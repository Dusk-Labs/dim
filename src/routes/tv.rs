use crate::core::DbConnection;
use crate::errors;

use auth::Wrapper as Auth;

use database::episode::{Episode, UpdateEpisode};
use database::media::Media;
use database::season::{Season, UpdateSeason};
use database::tv::TVShow;

use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub async fn get_tv_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    _user: Auth,
) -> Result<Json<Media>, errors::DimError> {
    Ok(Json(TVShow::get(&conn, id).await?))
}

#[get("/<id>/season")]
pub async fn get_tv_seasons(
    conn: State<'_, DbConnection>,
    id: i32,
    _user: Auth,
) -> Result<Json<Vec<Season>>, errors::DimError> {
    Ok(Json(Season::get_all(&conn, id).await?))
}

#[get("/<id>/season/<season_num>")]
pub async fn get_season_by_num(
    conn: State<'_, DbConnection>,
    id: i32,
    season_num: i32,
    _user: Auth,
) -> Result<Json<Season>, errors::DimError> {
    Ok(Json(Season::get(&conn, id, season_num).await?))
}

#[patch(
    "/<id>/season/<season_num>",
    format = "application/json",
    data = "<data>"
)]
pub async fn patch_season_by_num(
    conn: State<'_, DbConnection>,
    id: i32,
    season_num: i32,
    data: Json<UpdateSeason>,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    data.into_inner().update(&conn, id, season_num).await?;
    Ok(Status::NoContent)
}

#[delete("/<id>/season/<season_num>")]
pub async fn delete_season_by_num(
    conn: State<'_, DbConnection>,
    id: i32,
    season_num: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    Season::delete(&conn, id, season_num).await?;
    Ok(Status::Ok)
}

#[get("/<id>/season/<season_num>/episode/<ep_num>")]
pub async fn get_episode_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    season_num: i32,
    ep_num: i32,
    _user: Auth,
) -> Result<Json<Episode>, errors::DimError> {
    Ok(Json(Episode::get(&conn, id, season_num, ep_num).await?))
}

#[patch(
    "/<id>/season/<season_num>/episode/<ep_num>",
    format = "application/json",
    data = "<episode>"
)]
pub async fn patch_episode_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    season_num: i32,
    ep_num: i32,
    episode: Json<UpdateEpisode>,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    episode.update(&conn, id, season_num, ep_num).await?;
    Ok(Status::NoContent)
}

#[delete("/<id>/season/<season_num>/episode/<ep_num>")]
pub async fn delete_episode_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    season_num: i32,
    ep_num: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    Episode::delete(&conn, id, season_num, ep_num).await?;
    Ok(Status::Ok)
}
