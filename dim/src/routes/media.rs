use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;

use auth::Wrapper as Auth;

use std::convert::Infallible;
use std::sync::Arc;
use std::sync::Mutex;

use database::episode::Episode;
use database::genre::Genre;
use database::library::MediaType;
use database::media::Media;
use database::media::UpdateMedia;
use database::mediafile::MediaFile;
use database::progress::Progress;
use database::season::Season;
use database::tv::TVShow;

use futures::stream;
use futures::StreamExt;

use serde_json::json;
use serde_json::Value;

use warp::http::status::StatusCode;
use warp::reply;
use warp::Filter;

pub fn media_router(
    conn: DbConnection,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::get_media_by_id(conn.clone())
        .or(filters::get_extra_info_by_id(conn.clone()))
        .or(filters::update_media_by_id(conn.clone()))
        .or(filters::delete_media_by_id(conn.clone()))
        .or(filters::tmdb_search())
        .or(filters::map_progress(conn.clone()))
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use super::super::global_filters::with_state;
    use auth::Wrapper as Auth;
    use serde::Deserialize;

    use database::media::UpdateMedia;
    use database::DbConnection;

    pub fn get_media_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64)
            .and(warp::get())
            .and(with_state::<DbConnection>(conn))
            .and(auth::with_auth())
            .and_then(|id: i64, conn: DbConnection, user: Auth| async move {
                super::get_media_by_id(conn, id, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_extra_info_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64 / "info")
            .and(warp::get())
            .and(with_state::<DbConnection>(conn))
            .and(auth::with_auth())
            .and_then(|id: i64, conn: DbConnection, user: Auth| async move {
                super::get_extra_info_by_id(conn, id, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn update_media_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64)
            .and(warp::patch())
            .and(warp::body::json::<UpdateMedia>())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(super::update_media_by_id)
    }

    pub fn delete_media_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64)
            .and(warp::delete())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: Auth, conn: DbConnection| async move {
                super::delete_media_by_id(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn tmdb_search() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        #[derive(Deserialize)]
        struct RouteArgs {
            query: String,
            year: Option<i32>,
            media_type: String,
        }

        warp::path!("api" / "v1" / "media" / "tmdb_search")
            .and(warp::get())
            .and(warp::query::query::<RouteArgs>())
            .and(auth::with_auth())
            .and_then(
                |RouteArgs {
                     query,
                     year,
                     media_type,
                 }: RouteArgs,
                 auth: Auth| async move {
                    super::tmdb_search(query, year, media_type, auth)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn map_progress(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct RouteArgs {
            offset: i64,
        }

        warp::path!("api" / "v1" / "media" / i64 / "progress")
            .and(warp::post())
            .and(warp::query::query::<RouteArgs>())
            .and(with_state::<DbConnection>(conn))
            .and(auth::with_auth())
            .and_then(|id: i64, RouteArgs { offset }: RouteArgs, conn: DbConnection, auth: Auth| async move {
                super::map_progress(conn, id, offset, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

/// Method mapped to `GET /api/v1/media/<id>` returns info about a media based on the id queried.
/// This method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the media we want to query info of
/// * `_user` - Auth middleware
///
/// # Return Schema
/// ```text
/// {
///     "id": int,
///     "library_id": int,
///     "name": string,
///     "description": string,
///     "rating": int,
///     "year": int,
///     "added": string | date,
///     "poster_path": string | uri_path,
///     "backdrop_path": string | uri_path,
///     "media_type": string | enum,
///     "genres": [string],
///     "duration": int,
///     "duration_pretty": string,
/// }
/// ```
///
/// # Additional types
/// [`MediaType`](`database::library::MediaType`)
pub async fn get_media_by_id(
    conn: DbConnection,
    id: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let media = Media::get(&conn, id).await?;

    let media_id = match media.media_type {
        MediaType::Movie | MediaType::Episode => id,
        MediaType::Tv => Episode::get_first_for_show(&conn, id).await?.id,
    };

    let duration = match MediaFile::get_of_media(&conn, media_id).await {
        Ok(mut x) => x
            .pop()
            .and_then(|x| x.duration)
            .ok_or(errors::DimError::NoneError)?,
        Err(_) => 0,
    };

    let genres = Genre::get_by_media(&conn, id)
        .await?
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    let duration_pretty = match media.media_type {
        MediaType::Movie | MediaType::Episode => {
            format!("{} min", duration / 60)
        }
        MediaType::Tv => {
            let total_eps = TVShow::get_total_episodes(&conn, id).await?;
            let total_len = TVShow::get_total_duration(&conn, id).await?;
            format!("{} episodes | {} hr", total_eps, total_len / 3600)
        }
    };

    // FIXME: Remove the duration tag once the UI transitioned to using duration_pretty
    Ok(reply::json(&json!({
        "id": media.id,
        "library_id": media.library_id,
        "name": media.name,
        "description": media.description,
        "rating": media.rating,
        "year": media.year,
        "added": media.added,
        "poster_path": media.poster_path,
        "backdrop_path": media.backdrop_path,
        "media_type": media.media_type,
        "genres": genres,
        "duration": duration,
        "duration_pretty": duration_pretty,
    })))
}

/// Method mapped to `GET /api/v1/media/<id>/info` returns extra information about the media object
/// such as casts, directors, and mediafiles. This method can only be accessed by authenticated
/// users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the media we want to query info of
/// * `_user` - Auth middleware
pub async fn get_extra_info_by_id(
    conn: DbConnection,
    id: i64,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let media = Media::get(&conn, id).await?;

    match media.media_type {
        MediaType::Movie | MediaType::Episode => get_for_streamable(conn, id, user)
            .await
            .map(|x| reply::json(&x)),
        MediaType::Tv => get_for_show(conn, id, user).await.map(|x| reply::json(&x)),
    }
}

async fn get_for_streamable(
    conn: DbConnection,
    media_id: i64,
    user: Auth,
) -> Result<Value, errors::DimError> {
    let media_files = MediaFile::get_of_media(&conn, media_id).await?;

    Ok(json!({
        "progress": Progress::get_for_media_user(&conn, user.0.claims.get_user(), media_id).await
            .map(|x| x.delta)
            .unwrap_or(0),
        "versions": media_files.iter().map(|x| json!({
            "id": x.id,
            "file": x.target_file,
            "display_name": format!("{} - {} - {} - Library {}",
                                    x.codec.as_ref().unwrap_or(&"Unknown VC".to_string()),
                                    x.audio.as_ref().unwrap_or(&"Unknwon AC".to_string()),
                                    x.original_resolution.as_ref().unwrap_or(&"Unknown res".to_string()),
                                    x.library_id)
        })).collect::<Vec<_>>(),
    }))
}

async fn get_for_episode(
    conn: &DbConnection,
    media: Episode,
    user: &Auth,
) -> Result<Value, errors::DimError> {
    let media_files = MediaFile::get_of_media(&conn, media.id).await?;

    Ok(json!({
        "id": media.id,
        "progress": Progress::get_for_media_user(&conn, user.0.claims.get_user(), media.id)
            .await
            .map(|x| x.delta)
            .unwrap_or(0),
        "episode": media.episode,
        "description": media.media.description,
        "rating": media.media.rating,
        "backdrop": media.media.backdrop_path,
        "versions": media_files.iter().map(|x| json!({
            "id": x.id,
            "file": x.target_file,
            "display_name": format!("{} - {} - {} - Library {}",
                                    x.codec.as_ref().unwrap_or(&"Unknown VC".to_string()),
                                    x.audio.as_ref().unwrap_or(&"Unknwon AC".to_string()),
                                    x.original_resolution.as_ref().unwrap_or(&"Unknown res".to_string()),
                                    x.library_id)
        })).collect::<Vec<_>>(),
    }))
}

async fn get_for_show(
    conn: DbConnection,
    media_id: i64,
    user: Auth,
) -> Result<Value, errors::DimError> {
    let seasons = Season::get_all(&conn, media_id).await?;

    let mut result: Vec<Value> = Vec::new();

    for season in seasons.iter() {
        if let Ok(eps) = Episode::get_all_of_season(&conn, season.id).await {
            let mut episodes = vec![];
            for i in eps {
                if let Ok(y) = get_for_episode(&conn, i, &user).await {
                    episodes.push(y);
                }
            }

            result.push(json!({
                "id": season.id,
                "season_number": season.season_number,
                "name": if season.season_number == 0 {
                    "Extras".to_string()
                } else {
                    format!("Season {}", season.season_number)
                },
                "added": season.added,
                "poster": season.poster,
                "episodes": episodes
            }));
        }
    }

    Ok(json!({
        "seasons": result,
    }))
}

/// Method mapped to `PATCH /api/v1/media/<id>` is used to edit information about a media entry
/// manually. It is used in the web ui to manually edit metadata of a media.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the media we want to edit
/// * `data` - the info that we changed about the media entry
/// * `_user` - Auth middleware
pub async fn update_media_by_id(
    id: i64,
    data: UpdateMedia,
    _user: Auth,
    conn: DbConnection,
) -> Result<impl warp::Reply, Infallible> {
    let status = if data.update(&conn, id).await.is_ok() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_MODIFIED
    };

    Ok(status)
}

/// Method mapped to `DELETE /api/v1/media/<id>` is used to delete a media entry for the library.
/// ONly authenticated users can query this.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the media we want to delete
/// * `_user` - auth middleware
pub async fn delete_media_by_id(
    conn: DbConnection,
    id: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Media::delete(&conn, id).await?;
    Ok(StatusCode::OK)
}

/// Method mapped to `GET /api/v1/media/tmdb_search` is used to quickly search TMDB based on 3
/// params, one of which is optional. This is used client side in the rematch utility
///
/// # Arguments
/// * `query` - the query we want to send to tmdb, ie movie title, tv show title
/// * `year` - optional parameter specifying the release year of the media we want to look up
/// * `media_type` - parameter that tells us what media type we are querying, ie movie or tv show
pub async fn tmdb_search(
    query: String,
    year: Option<i32>,
    media_type: String,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    use crate::scanners::tmdb::Tmdb;
    use database::library::MediaType;

    let media_type = match media_type.as_ref() {
        "movie" => MediaType::Movie,
        "tv" => MediaType::Tv,
        _ => return Err(errors::DimError::InvalidMediaType),
    };

    let mut tmdb_session = Tmdb::new("38c372f5bc572c8aadde7a802638534e".to_string(), media_type);

    Ok(reply::json(
        &tmdb_session
            .search_by_name(query, year, None)
            .await
            .map_err(|_| errors::DimError::NotFoundError)?
            .into_iter()
            .map(Into::<crate::scanners::ApiMedia>::into)
            .collect::<Vec<_>>(),
    ))
}

/// Method mapped to `POST /api/v1/media/<id>/progress` is used to map progress for a certain media
/// to the user. This is useful for remembering progress for a movie etc.
///
/// # Arguments
/// * `id` - id of the media to modify
///
/// # Query params
/// * `offset` - offset in seconds
pub async fn map_progress(
    conn: DbConnection,
    id: i64,
    offset: i64,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Progress::set(&conn, offset, user.0.claims.get_user(), id).await?;
    Ok(StatusCode::OK)
}
