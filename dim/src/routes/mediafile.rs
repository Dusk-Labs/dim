use crate::core::DbConnection;
use crate::errors;
use crate::errors::ErrorStatusCode;
use crate::external::ExternalQueryIntoShow;
use crate::scanner::movie;
use crate::scanner::parse_filenames;
use crate::scanner::tv_show;
use crate::scanner::MediaMatcher;
use crate::scanner::WorkUnit;

use super::media::MOVIES_PROVIDER;
use super::media::TV_PROVIDER;

use dim_database::library::MediaType;
use dim_database::mediafile::MediaFile;
use dim_database::user::User;

use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use warp::reject::Reject;
use warp::reply;

use http::StatusCode;
use tracing::error;
use tracing::info;

#[derive(Clone, Debug, thiserror::Error, Serialize, displaydoc::Display)]
pub enum Error {
    /// Supplied no mediafiles when rematching.
    NoMediafiles,
}

impl Reject for Error {}

impl ErrorStatusCode for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            &Error::NoMediafiles => StatusCode::BAD_REQUEST,
        }
    }
}

pub mod filters {
    use dim_database::user::User;
    use warp::reject;
    use warp::Filter;

    use crate::routes::global_filters::with_auth;

    use super::super::global_filters::with_state;
    use dim_database::DbConnection;

    use serde::Deserialize;

    pub fn get_mediafile_info(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "mediafile" / i64)
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: User, conn: DbConnection| async move {
                super::get_mediafile_info(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn rematch_mediafile(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RouteArgs {
            tmdb_id: String,
            media_type: String,
            mediafiles: Vec<i64>,
        }

        warp::path!("api" / "v1" / "mediafile" / "match")
            .and(warp::patch())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and(warp::body::json::<RouteArgs>())
            .and_then(
                |_auth: User,
                 conn: DbConnection,
                 RouteArgs {
                     tmdb_id,
                     media_type,
                     mediafiles,
                 }: RouteArgs| async move {
                    super::rematch_mediafile(conn, mediafiles, tmdb_id, media_type)
                        .await
                        .map_err(reject::custom)
                },
            )
    }
}

/// Method mapped to `GET /api/v1/mediafile/<id>` is used to get information about a mediafile by its id.
///
/// # Arguments
/// * `id` - id of the mediafile we want info about
pub async fn get_mediafile_info(
    conn: DbConnection,
    id: i64,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    let mediafile = MediaFile::get_one(&mut tx, id)
        .await
        .map_err(|_| errors::DimError::NotFoundError)?;

    Ok(reply::json(&json!({
        "id": mediafile.id,
        "media_id": mediafile.media_id,
        "library_id": mediafile.library_id,
        "raw_name": mediafile.raw_name,
    })))
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
    conn: DbConnection,
    mediafiles: Vec<i64>,
    external_id: String,
    media_type: String,
) -> Result<impl warp::Reply, errors::DimError> {
    if mediafiles.is_empty() {
        return Err(Error::NoMediafiles.into());
    }

    let Ok(media_type): Result<MediaType, ()> = media_type.to_lowercase().try_into() else {
        return Err(errors::DimError::InvalidMediaType);
    };

    let mut tx = conn.read().begin().await?;

    // FIXME: impl FromStr for MediaType
    let provider: Arc<dyn ExternalQueryIntoShow> = match media_type {
        MediaType::Movie => (*MOVIES_PROVIDER).clone(),
        MediaType::Tv => (*TV_PROVIDER).clone(),
        _ => return Err(errors::DimError::InvalidMediaType),
    };

    let matcher = match media_type {
        MediaType::Movie => Arc::new(movie::MovieMatcher) as Arc<dyn MediaMatcher>,
        MediaType::Tv => Arc::new(tv_show::TvMatcher) as Arc<dyn MediaMatcher>,
        _ => unreachable!(),
    };

    info!(?media_type, mediafiles = ?&mediafiles, "Rematching mediafiles");

    let mediafiles = MediaFile::get_many(&mut tx, &mediafiles).await?;

    provider.search_by_id(&external_id).await.map_err(|e| {
        error!(?e, "Failed to search for tmdb_id when rematching.");
        errors::DimError::ExternalSearchError(e)
    })?;

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;

    for mediafile in mediafiles {
        let Some((_, metadata)) = parse_filenames(IntoIterator::into_iter([&mediafile.target_file])).pop() else {
            continue;
        };

        matcher
            .match_to_id(
                &mut tx,
                provider.clone(),
                WorkUnit(mediafile.clone(), metadata),
                &external_id,
            )
            .await?;
    }

    tx.commit().await?;

    Ok(StatusCode::OK)
}
