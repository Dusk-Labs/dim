use crate::core::DbConnection;
use crate::errors;

use dim_database::user::User;

use dim_database::episode::{Episode, UpdateEpisode};
use dim_database::season::{Season, UpdateSeason};

use warp::http::status::StatusCode;
use warp::reply;

pub mod filters {
    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use super::super::global_filters::with_auth;
    use super::super::global_filters::with_state;
    use dim_database::episode::UpdateEpisode;
    use dim_database::season::UpdateSeason;
    use dim_database::user::User;
    use dim_database::DbConnection;

    pub fn get_tv_seasons(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "tv" / i64 / "season")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: User, conn: DbConnection| async move {
                super::get_tv_seasons(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_season_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "season" / i64)
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: User, conn: DbConnection| async move {
                super::get_season_by_id(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn patch_season_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "season" / i64)
            .and(warp::patch())
            .and(warp::body::json::<UpdateSeason>())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |id: i64, data: UpdateSeason, auth: User, conn: DbConnection| async move {
                    super::patch_season_by_id(conn, id, data, auth)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn delete_season_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "season" / i64)
            .and(warp::delete())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: User, conn: DbConnection| async move {
                super::delete_season_by_id(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_season_episodes(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "season" / i64 / "episodes")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: User, conn: DbConnection| async move {
                super::get_season_episodes(conn, id, auth)
                    .await
                    .map_err(reject::custom)
            })
    }

    pub fn patch_episode_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "episode" / i64)
            .and(warp::patch())
            .and(warp::body::json::<UpdateEpisode>())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |id: i64, data: UpdateEpisode, auth: User, conn: DbConnection| async move {
                    super::patch_episode_by_id(conn, id, data, auth)
                        .await
                        .map_err(reject::custom)
                },
            )
    }

    pub fn delete_episode_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "episode" / i64)
            .and(warp::delete())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: User, conn: DbConnection| async move {
                super::delete_episode_by_id(conn, id, auth)
                    .await
                    .map_err(reject::custom)
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
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    Ok(reply::json(&Season::get_all(&mut tx, id).await?))
}

/// Method mapped to `GET /api/v1/tv/<id>/season/<season_num>` returns info about the season
/// <season_num> for tv show by <id>
///
/// # Arguments
/// * `id` - id of the tv show we want info about
/// * `season_num` - the season we want info about
pub async fn get_season_by_id(
    conn: DbConnection,
    id: i64,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    Ok(reply::json(&Season::get_by_id(&mut tx, id).await?))
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
/// `dim_database::season::UpdateSeason`.
pub async fn patch_season_by_id(
    conn: DbConnection,
    id: i64,
    data: UpdateSeason,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    data.update(&mut tx, id).await?;
    tx.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Method mapped to `DELETE /api/v1/tv/<id>/season/<season_num>` allows you to delete a season for
/// a particular tv show.
///
/// # Arguments
/// * `id` - id of the tv show.
/// * `season_num` - the season we want to remove
pub async fn delete_season_by_id(
    conn: DbConnection,
    id: i64,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    Season::delete_by_id(&mut tx, id).await?;
    tx.commit().await?;
    Ok(StatusCode::OK)
}

/// Method mapped to `GET /api/v1/episode/<id>` returns information
/// about a episode for a season.
///
/// # Arguments
/// * `id` - id of the episode.
pub async fn get_season_episodes(
    conn: DbConnection,
    season_id: i64,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    #[derive(serde::Serialize)]
    pub struct Record {
        pub id: i64,
        pub name: String,
        pub thumbnail_url: Option<String>,
        pub episode: i64,
    }

    let result = sqlx::query_as!(Record,
        r#"SELECT episode.id as "id!", _tblmedia.name, assets.local_path as thumbnail_url, episode.episode_ as "episode!"
        FROM episode
        INNER JOIN _tblmedia on _tblmedia.id = episode.id
        LEFT JOIN assets ON assets.id = _tblmedia.backdrop
        WHERE episode.seasonid = ?"#,
        season_id
    ).fetch_all(&mut tx).await?;

    Ok(reply::json(&result))
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
/// `dim_database::episode::UpdateEpisode`.
pub async fn patch_episode_by_id(
    conn: DbConnection,
    id: i64,
    episode: UpdateEpisode,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    episode.update(&mut tx, id).await?;
    tx.commit().await?;
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
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    Episode::delete(&mut tx, id).await?;
    tx.commit().await?;
    Ok(StatusCode::OK)
}
