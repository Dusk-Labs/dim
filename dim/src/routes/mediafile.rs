use crate::core::DbConnection;
use crate::errors;

use auth::Wrapper as Auth;
use database::mediafile::MediaFile;

use serde_json::json;
use warp::http::status::StatusCode;
use warp::reply;

pub mod filters {
    use warp::reject;
    use warp::Filter;

    use super::super::global_filters::with_state;
    use auth::Wrapper as Auth;
    use database::DbConnection;

    use serde::Deserialize;

    pub fn get_mediafile_info(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "mediafile" / i64)
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: Auth, conn: DbConnection| async move {
                super::get_mediafile_info(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn rematch_mediafile(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct RouteArgs {
            tmdb_id: i32,
            media_type: String,
        }

        warp::path!("api" / "v1" / "mediafile" / i64 / "match")
            .and(warp::patch())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and(warp::query::query::<RouteArgs>())
            .and_then(
                |id: i64,
                 _auth: Auth,
                 conn: DbConnection,

                 RouteArgs {
                     tmdb_id,
                     media_type,
                 }: RouteArgs| async move {
                    super::rematch_mediafile(conn, id, tmdb_id, media_type)
                        .await
                        .map_err(|e| reject::custom(e))
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
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let mediafile = MediaFile::get_one(&conn, id)
        .await
        .map_err(|_| errors::DimError::NotFoundError)?;

    Ok(reply::json(&json!({
        "id": mediafile.id,
        "media_id": mediafile.media_id,
        "library_id": mediafile.library_id,
        "raw_name": mediafile.raw_name,
    })))
}

/// Method mapped to `PATCH /api/v1/mediafile/<id>/match` used to match a unmatched(orphan)
/// mediafile to a tmdb id.
///
/// # Arguments
/// * `conn` - database connection
/// * `log` - logger
/// * `event_tx` - websocket channel over which we dispatch a event notifying other clients of the
/// new metadata
///
/// * `id` - id of the orphan mediafile we want to rematch
/// * `tmdb_id` - the tmdb id of the proper metadata we want to fetch for the media
pub async fn rematch_mediafile(
    conn: DbConnection,
    id: i64,
    tmdb_id: i32,
    media_type: String,
) -> Result<impl warp::Reply, errors::DimError> {
    use crate::scanners::tmdb::Tmdb;
    use database::library::MediaType;

    let mediafile = MediaFile::get_one(&conn, id).await?;
    let matcher = crate::scanners::get_matcher_unchecked();

    let mut tmdb = match media_type.to_lowercase().as_ref() {
        "movie" => Tmdb::new("38c372f5bc572c8aadde7a802638534e".into(), MediaType::Movie),
        "tv" => Tmdb::new("38c372f5bc572c8aadde7a802638534e".into(), MediaType::Tv),
        _ => return Err(errors::DimError::InvalidMediaType),
    };

    let result = tmdb
        .search_by_id(tmdb_id)
        .await
        .map_err(|_| errors::DimError::NotFoundError)?;

    match media_type.to_lowercase().as_ref() {
        "movie" => {
            matcher
                .match_movie_to_result(mediafile, result.into())
                .await?
        }
        "tv" => matcher.match_tv_to_result(mediafile, result.into()).await?,
        _ => unreachable!(),
    }

    Ok(StatusCode::OK)
}
