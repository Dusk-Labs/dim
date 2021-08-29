use crate::core::DbConnection;
use crate::errors;

use auth::Wrapper as Auth;

use database::episode::{Episode, UpdateEpisode};
use database::media::Media;
use database::season::{Season, UpdateSeason};

use warp::http::status::StatusCode;
use warp::reply;
use warp::Filter;

pub mod filters {
    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use super::super::global_filters::with_state;
    use auth::Wrapper as Auth;
    use database::episode::UpdateEpisode;
    use database::season::UpdateSeason;
    use database::DbConnection;

    pub fn get_tv_seasons(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "tv" / i64 / "season")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: Auth, conn: DbConnection| async move {
                super::get_tv_seasons(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_season_by_num(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "tv" / i64 / "season" / i64)
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |id: i64, season_num: i64, auth: Auth, conn: DbConnection| async move {
                    super::get_season_by_num(conn, id, season_num, auth)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn patch_season_by_num(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "season" / i64)
            .and(warp::patch())
            .and(warp::body::json::<UpdateSeason>())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |id: i64, data: UpdateSeason, auth: Auth, conn: DbConnection| async move {
                    super::patch_season_by_num(conn, id, data, auth)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn delete_season_by_num(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "tv" / i64 / "season" / i64)
            .and(warp::delete())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |id: i64, season_num: i64, auth: Auth, conn: DbConnection| async move {
                    super::delete_season_by_num(conn, id, season_num, auth)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn get_episode_by_num(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "episode" / i64)
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: Auth, conn: DbConnection| async move {
                super::get_episode_by_id(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn patch_episode_by_num(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "episode" / i64)
            .and(warp::patch())
            .and(warp::body::json::<UpdateEpisode>())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |id: i64, data: UpdateEpisode, auth: Auth, conn: DbConnection| async move {
                    super::patch_episode_by_id(conn, id, data, auth)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn delete_episode_by_num(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "episode" / i64)
            .and(warp::delete())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: Auth, conn: DbConnection| async move {
                super::delete_episode_by_id(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

/// Method mapped to `GET /api/v1/tv/<id>/season` returns all seasons for TV Show mapped to the id
/// passed in.
///
/// # Arguments
/// * `id` - id of the tv show we want info about
pub async fn get_tv_seasons(
    conn: DbConnection,
    id: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(&Season::get_all(&conn, id).await?))
}

/// Method mapped to `GET /api/v1/tv/<id>/season/<season_num>` returns info about the season
/// <season_num> for tv show by <id>
///
/// # Arguments
/// * `id` - id of the tv show we want info about
/// * `season_num` - the season we want info about
pub async fn get_season_by_num(
    conn: DbConnection,
    id: i64,
    season_num: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(&Season::get(&conn, id, season_num).await?))
}

/// Method mapped to `PATCH /api/v1/tv/<id>/season/<season_num>` allows you to patch in info about
/// the season <season_num>.
///
/// # Route Arguments
/// * `id` - the id of the tv show.
/// * `season_num` - the season we want to edit.
///
/// # Data
/// This route additionally requires you to pass in a json object by the format of
/// `database::season::UpdateSeason`.
pub async fn patch_season_by_num(
    conn: DbConnection,
    id: i64,
    data: UpdateSeason,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    data.update(&conn, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Method mapped to `DELETE /api/v1/tv/<id>/season/<season_num>` allows you to delete a season for
/// a particular tv show.
///
/// # Arguments
/// * `id` - id of the tv show.
/// * `season_num` - the season we want to remove
pub async fn delete_season_by_num(
    conn: DbConnection,
    id: i64,
    season_num: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Season::delete(&conn, id, season_num).await?;
    Ok(StatusCode::OK)
}

/// Method mapped to `GET /api/v1/episode/<id>` returns information
/// about a episode for a season.
///
/// # Arguments
/// * `id` - id of the episode.
pub async fn get_episode_by_id(
    conn: DbConnection,
    id: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(&Episode::get_by_id(&conn, id).await?))
}

/// TODO: Move all of these into a unified update interface for media items
/// Method mapped to `PATCH /api/v1/episode/<id>` lets you patch
/// information about a episode.
///
/// # Arguments
/// * `id` - id of a episode.
///
/// # Data
/// This route additionally requires you to pass in a json object by the format of
/// `database::episode::UpdateEpisode`.
pub async fn patch_episode_by_id(
    conn: DbConnection,
    id: i64,
    episode: UpdateEpisode,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    episode.update(&conn, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Method mapped to `DELETE /api/v1/episode/<id>` allows you to
/// delete a episode belonging to some season.
///
/// # Arguments
/// * `id` - id an episode to delete
pub async fn delete_episode_by_id(
    conn: DbConnection,
    id: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Episode::delete(&conn, id).await?;
    Ok(StatusCode::OK)
}
