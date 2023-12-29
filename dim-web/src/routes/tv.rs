use crate::AppState;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;

use dim_database::episode::{Episode, UpdateEpisode};
use dim_database::season::{Season, UpdateSeason};
use dim_database::DatabaseError;

use http::StatusCode;

use serde_json::json;

use super::auth::AuthError;

/// Method mapped to `GET /api/v1/tv/<id>/season` returns all seasons for TV Show mapped to the id
/// passed in.
///
/// # Arguments
/// * `id` - id of the tv show we want info about
pub async fn get_tv_seasons(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AuthError> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
    Ok(axum::response::Json(json!(&Season::get_all(&mut tx, id).await?)).into_response())
}

/// Method mapped to `GET /api/v1/season/<id>` returns info about the season by `id`
///
/// # Arguments
/// * `id` - id of the season we want info about
pub async fn get_season_by_id(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AuthError> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
    Ok(axum::response::Json(json!(&Season::get_by_id(&mut tx, id).await?)).into_response())
}

/// Method mapped to `PATCH /api/v1/season/<id>` allows you to patch in info about
/// the season.
///
/// # Route Arguments
/// * `id` - the id of the season.
///
/// # Data
/// This route additionally requires you to pass in a json object by the format of
/// `dim_database::season::UpdateSeason`.
pub async fn patch_season_by_id(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
    Json(season): Json<UpdateSeason>,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;
    season.update(&mut tx, id).await?;
    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Method mapped to `GET /api/v1/episode/<id>` returns information
/// about a episode for a season.
///
/// # Arguments
/// * `id` - id of the episode.
pub async fn get_season_episodes(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AuthError> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
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
        id
    ).fetch_all(&mut *tx).await.unwrap_or_default();

    Ok(axum::response::Json(json!(&result)).into_response())
}

/// Method mapped to `DELETE /api/v1/season/<id>` allows you to delete a season for
/// a particular tv show.
///
/// # Arguments
/// * `id` - id of the season.
pub async fn delete_season_by_id(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;
    Season::delete_by_id(&mut tx, id).await?;
    tx.commit().await.map_err(DatabaseError::from)?;
    Ok(StatusCode::OK)
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
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
    Json(episode): Json<UpdateEpisode>,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;
    episode.update(&mut tx, id).await?;
    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Method mapped to `DELETE /api/v1/episode/<id>` allows you to
/// delete a episode belonging to some season.
///
/// # Arguments
/// * `id` - id an episode to delete
pub async fn delete_episode_by_id(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;
    Episode::delete(&mut tx, id).await?;
    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(StatusCode::OK)
}
