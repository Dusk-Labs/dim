use crate::AppState;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::response::Response;
use axum::extract::Path;
use axum::extract::State;

use dim_core::scanner::movie;
use dim_core::scanner::parse_filenames;
use dim_core::scanner::tv_show;
use dim_core::scanner::MediaMatcher;
use dim_core::scanner::WorkUnit;

use super::media::MOVIES_PROVIDER;
use super::media::TV_PROVIDER;

use dim_database::DatabaseError;
use dim_database::library::MediaType;
use dim_database::mediafile::MediaFile;

use dim_extern_api::ExternalQueryIntoShow;

use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use http::StatusCode;
use tracing::error;
use tracing::info;

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum Error {
    /// No mediafiles.
    NoMediafiles,
    /// Invalid media type.
    InvalidMediaType,
    /// Not logged in.
    InvalidCredentials,
    /// database: {0}
    Database(#[from] DatabaseError),
    /// Failed to search for tmdb_id when rematching: {0}
    ExternalSearchError(String),
}

impl From<dim_core::scanner::error::Error> for Error {
    fn from(e: dim_core::scanner::error::Error) -> Self {
        Self::ExternalSearchError(format!("{:?}", e))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::ExternalSearchError(_) => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            Self::InvalidMediaType => {
                (StatusCode::NOT_ACCEPTABLE, self.to_string()).into_response()
            }
            Self::NoMediafiles => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            Self::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}

#[derive(Deserialize)]
pub struct RouteArgs {
    tmdb_id: String,
    media_type: String,
    mediafiles: Vec<i64>,
}

/// Method mapped to `GET /api/v1/mediafile/<id>` is used to get information about a mediafile by its id.
///
/// # Arguments
/// * `id` - id of the mediafile we want info about
pub async fn get_mediafile_info(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
    let mediafile = MediaFile::get_one(&mut tx, id)
        .await
        .map_err(DatabaseError::from)?;

    Ok(Json(&json!({
        "id": mediafile.id,
        "media_id": mediafile.media_id,
        "library_id": mediafile.library_id,
        "raw_name": mediafile.raw_name,
    })).into_response())
}

/// Method mapped to `PATCH /api/v1/mediafile/match` used to match a unmatched(orphan)
/// mediafile to a tmdb id.
///
/// # Arguments
/// * `conn` - database connection
/// * `log` - logger
/// * `event_tx` - websocket channel over which we dispatch a event notifying other clients of the
/// new metadata
///
/// * `mediafiles` - ids of the orphan mediafiles we want to rematch
/// * `tmdb_id` - the tmdb id of the proper metadata we want to fetch for the media
pub async fn rematch_mediafile(
    State(AppState { conn, .. }): State<AppState>,
    Json(route_args): Json<RouteArgs>,
) -> Result<impl IntoResponse, Error> {
    if route_args.mediafiles.is_empty() {
        return Err(Error::NoMediafiles.into());
    }

    let Ok(media_type): Result<MediaType, ()> = route_args.media_type.to_lowercase().try_into() else {
        return Err(Error::InvalidMediaType);
    };

    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;

    // FIXME: impl FromStr for MediaType
    let provider: Arc<dyn ExternalQueryIntoShow> = match media_type {
        MediaType::Movie => (*MOVIES_PROVIDER).clone(),
        MediaType::Tv => (*TV_PROVIDER).clone(),
        _ => return Err(Error::InvalidMediaType),
    };

    let matcher = match media_type {
        MediaType::Movie => Arc::new(movie::MovieMatcher) as Arc<dyn MediaMatcher>,
        MediaType::Tv => Arc::new(tv_show::TvMatcher) as Arc<dyn MediaMatcher>,
        _ => unreachable!(),
    };

    info!(?media_type, route_args.mediafiles = ?&route_args.mediafiles, "Rematching mediafiles");

    let mediafiles = MediaFile::get_many(&mut tx, &route_args.mediafiles).await.map_err(DatabaseError::from)?;

    provider.search_by_id(&route_args.tmdb_id).await.map_err(|e| {
        error!(?e, "Failed to search for tmdb_id when rematching.");
        Error::ExternalSearchError(e.to_string())
    })?;

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(DatabaseError::from)?;

    for mediafile in mediafiles {
        let Some((_, metadata)) = parse_filenames(IntoIterator::into_iter([&mediafile.target_file])).pop() else {
            continue;
        };

        matcher
            .match_to_id(
                &mut tx,
                provider.clone(),
                WorkUnit(mediafile.clone(), metadata),
                &route_args.tmdb_id,
            )
            .await
            .map_err(|e| {
                error!(?e, "Failed to match tmdb_id.");
                Error::ExternalSearchError(e.to_string())
            })?;
    }

    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(StatusCode::OK)
}
