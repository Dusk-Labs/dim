use crate::core::DbConnection;
use crate::errors;
use crate::external::tmdb::TMDBMetadataProvider;
use crate::external::ExternalQueryIntoShow;
use crate::json;
use crate::tree;
use crate::utils::secs_to_pretty;

use chrono::Datelike;
use dim_database::user::User;

use dim_database::compact_mediafile::CompactMediafile;
use dim_database::episode::Episode;
use dim_database::genre::Genre;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::media::UpdateMedia;
use dim_database::mediafile::MediaFile;
use dim_database::progress::Progress;

use warp::http::status::StatusCode;
use warp::reply;

use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::Lazy;
use serde::Serialize;

pub const API_KEY: &str = "38c372f5bc572c8aadde7a802638534e";
pub const MOVIES_PROVIDER: Lazy<Arc<dyn ExternalQueryIntoShow>> =
    Lazy::new(|| Arc::new(TMDBMetadataProvider::new(&API_KEY).movies()));
pub const TV_PROVIDER: Lazy<Arc<dyn ExternalQueryIntoShow>> =
    Lazy::new(|| Arc::new(TMDBMetadataProvider::new(&API_KEY).tv_shows()));

pub mod filters {
    use dim_database::user::User;
    use warp::reject;
    use warp::Filter;

    use crate::routes::global_filters::with_auth;

    use super::super::global_filters::with_state;
    use serde::Deserialize;

    use dim_database::media::UpdateMedia;
    use dim_database::DbConnection;

    pub fn get_media_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64)
            .and(warp::get())
            .and(with_state::<DbConnection>(conn.clone()))
            .and(with_auth(conn))
            .and_then(|id: i64, conn: DbConnection, user: User| async move {
                super::get_media_by_id(conn, id, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_media_files(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64 / "files")
            .and(warp::get())
            .and(with_state::<DbConnection>(conn.clone()))
            .and(with_auth(conn))
            .and_then(|id: i64, conn: DbConnection, _user: User| async move {
                super::get_media_files(conn, id)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_mediafile_tree(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64 / "tree")
            .and(warp::get())
            .and(with_state::<DbConnection>(conn.clone()))
            .and(with_auth(conn))
            .and_then(|id, conn, _user| async move {
                super::get_mediafile_tree(conn, id)
                    .await
                    .map_err(reject::custom)
            })
    }

    pub fn update_media_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64)
            .and(warp::patch())
            .and(warp::body::json::<UpdateMedia>())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id, body, auth, conn| async move {
                super::update_media_by_id(id, body, auth, conn)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn delete_media_by_id(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "media" / i64)
            .and(warp::delete())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, auth: User, conn: DbConnection| async move {
                super::delete_media_by_id(conn, id, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn tmdb_search(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct RouteArgs {
            query: String,
            year: Option<i32>,
            media_type: String,
        }

        warp::path!("api" / "v1" / "media" / "tmdb_search")
            .and(warp::get())
            .and(warp::query::query::<RouteArgs>())
            .and(with_auth(conn))
            .and_then(
                |RouteArgs {
                     query,
                     year,
                     media_type,
                 }: RouteArgs,
                 auth: User| async move {
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
            .and(with_state::<DbConnection>(conn.clone()))
            .and(with_auth(conn))
            .and_then(|id: i64, RouteArgs { offset }: RouteArgs, conn: DbConnection, auth: User| async move {
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
/// [`MediaType`](`dim_database::library::MediaType`)
pub async fn get_media_by_id(
    conn: DbConnection,
    id: i64,
    user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    let media = Media::get(&mut tx, id).await?;

    let media_id = match media.media_type {
        MediaType::Movie | MediaType::Episode => id,
        MediaType::Tv => Episode::get_first_for_show(&mut tx, id).await?.id,
    };

    // TODO: at some point we want to issue a warning to the UI that none of the mediafiles with
    // this media have a duration (maybe because of corruption).
    let duration = match MediaFile::get_of_media(&mut tx, media_id).await {
        Ok(x) => x
            .iter()
            .filter_map(|x| x.duration)
            .collect::<Vec<_>>()
            .pop()
            .unwrap_or(0),
        Err(_) => 0,
    };

    let genres = Genre::get_by_media(&mut tx, id)
        .await?
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    let progress = match media.media_type {
        MediaType::Episode | MediaType::Movie => Progress::get_for_media_user(&mut tx, user.id, id)
            .await
            .map(|x| json!({"progress": x.delta}))
            .ok(),
        MediaType::Tv => {
            if let Ok(Some(ep)) = Episode::get_last_watched_episode(&mut tx, id, user.id).await {
                let (delta, duration) = Progress::get_progress_for_media(&mut tx, ep.id, user.id)
                    .await
                    .unwrap_or((0, 1));

                // NOTE: When we get to the last episode of a tv show we want to return the last
                // episode even if the client finished watching it.
                let next_episode = ep.get_next_episode(&mut tx).await;
                if (delta as f64 / duration as f64) > 0.90 && next_episode.is_ok() {
                    let next_episode = next_episode.unwrap();
                    let (delta, _duration) =
                        Progress::get_progress_for_media(&mut tx, ep.id, user.id)
                            .await
                            .unwrap_or((0, 1));

                    Some(json!({
                        "progress": delta,
                        "season": next_episode.get_season_number(&mut tx).await.unwrap_or(0),
                        "episode": next_episode.episode,
                        "play_btn_id": next_episode.id,
                    }))
                } else {
                    Some(json!({
                        "progress": delta,
                        "season": ep.get_season_number(&mut tx).await.unwrap_or(0),
                        "episode": ep.episode,
                        "play_btn_id": ep.id,
                    }))
                }
            } else {
                let ep = Episode::get_first_for_show(&mut tx, id).await?;
                Some(json!({
                    "progress": 0,
                    "season": ep.get_season_number(&mut tx).await.unwrap_or(0),
                    "episode": ep.episode,
                    "play_btn_id": ep.id,
                }))
            }
        }
    };

    fn mediafile_tags(x: &MediaFile) -> serde_json::Value {
        let video_tag = format!(
            "{} ({})",
            x.quality
                .as_ref()
                .map(|x| format!("{}p", x))
                .unwrap_or("Unknown".into()),
            crate::utils::codec_pretty(x.codec.as_deref().unwrap_or("Unknown"))
        );

        let audio_lang = x.audio_language.as_deref().unwrap_or("Unknown");
        let audio_codec = crate::utils::codec_pretty(x.audio.as_deref().unwrap_or("Unknown"));
        let audio_ch = crate::utils::channels_pretty(x.channels.unwrap_or(2));

        let audio_tag = format!("{} ({} {})", audio_lang, audio_codec, audio_ch);

        json!({
            "video": video_tag,
            "audio": audio_tag,
        })
    }

    let quality_tags = match media.media_type {
        MediaType::Episode | MediaType::Movie => json!({
                media.id.to_string(): MediaFile::get_of_media(&mut tx, media.id)
                    .await?
                    .first()
                    .map(mediafile_tags)
        }),
        MediaType::Tv => {
            let mut result = MediaFile::get_of_show(&mut tx, media.id).await?;

            result.dedup_by_key(|x| x.media_id);

            json!(result
                .iter()
                .map(|x| (x.media_id.unwrap(), mediafile_tags(x)))
                .collect::<HashMap<_, _>>())
        }
    };

    let season_episode_tag = match media.media_type {
        MediaType::Episode => {
            let result = Episode::get_season_episode_by_id(&mut tx, id).await?;
            Some(json!({
                "season": result.0,
                "episode": result.1,
            }))
        }
        _ => None,
    };

    const EPISODE_DONE_THRESH: f64 = 0.9;

    let next_episode_id = match Episode::get_by_id(&mut tx, id).await {
        Ok(x) => {
            let next_episode = if let Ok(x) = x.get_next_episode(&mut tx).await {
                Some(json!({
                    "next_episode_id": x.id,
                    "chapters": {
                        "credits": x.media.get_first_duration(&mut tx).await as f64 * EPISODE_DONE_THRESH
                    }
                }))
            } else {
                None
            };

            let prev_episode = x
                .get_prev_episode(&mut tx)
                .await
                .map(|x| json!({"prev_episode_id": x.id}))
                .ok();

            if next_episode.is_some() || prev_episode.is_some() {
                Some(json!({
                    ..?next_episode,
                    ..?prev_episode,
                }))
            } else {
                None
            }
        }
        Err(_) => None,
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
        "tags": quality_tags,
        ..?next_episode_id,
        ..?season_episode_tag,
        ..?progress
    })))
}

pub async fn get_media_files(
    conn: DbConnection,
    id: i64,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    let media_type = Media::media_mediatype(&mut tx, id).await?;

    let mediafiles = match media_type {
        MediaType::Tv => MediaFile::get_of_show(&mut tx, id).await?,
        MediaType::Episode | MediaType::Movie => MediaFile::get_of_media(&mut tx, id).await?,
    };

    Ok(reply::json(&mediafiles))
}

/// # GET `/api/v1/media/<id>/tree`
/// Method mappedReturns a tree of mediafiles for a given media object.
///
/// # Authentication
/// Method requires standard authentication.
pub async fn get_mediafile_tree(
    conn: DbConnection,
    id: i64,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    let media_type = Media::media_mediatype(&mut tx, id).await?;

    let mut mediafiles = match media_type {
        MediaType::Movie | MediaType::Episode => {
            CompactMediafile::all_for_media(&mut tx, id).await?
        }
        MediaType::Tv => CompactMediafile::all_for_tv(&mut tx, id).await?,
    };

    // we want to pre-sort to ensure our tree is somewhat ordered.
    mediafiles.sort_by(|a, b| a.target_file.cmp(&b.target_file));

    let count = mediafiles.len();

    #[derive(Serialize)]
    struct Record {
        id: i64,
        name: String,
        duration: Option<i64>,
        file: String,
    }

    let entry = tree::Entry::build_with(
        mediafiles,
        |x| {
            x.target_file
                .iter()
                .map(|x| x.to_string_lossy().to_string())
                .collect()
        },
        |k, v| Record {
            id: v.id,
            name: v.name,
            duration: v.duration,
            file: k.to_string(),
        },
    );

    #[derive(Serialize)]
    struct Response {
        count: usize,
        files: Vec<tree::Entry<Record>>,
    }

    let entries = match entry {
        tree::Entry::Directory { files, .. } => files,
        _ => unreachable!(),
    };

    Ok(reply::json(&Response {
        files: entries,
        count,
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
    _user: User,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    let status = if data.update(&mut tx, id).await.is_ok() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_MODIFIED
    };

    tx.commit().await?;

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
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    Media::delete(&mut tx, id).await?;
    tx.commit().await?;
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
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let Ok(media_type) = media_type.to_lowercase().try_into() else {
        return Err(errors::DimError::InvalidMediaType);
    };

    let provider = match media_type {
        MediaType::Movie => (*MOVIES_PROVIDER).clone(),
        MediaType::Tv => (*TV_PROVIDER).clone(),
        _ => return Err(errors::DimError::InvalidMediaType),
    };

    let results = provider
        .search(&query, year)
        .await
        .map_err(|_| errors::DimError::NotFoundError)?;

    if results.is_empty() {
        return Err(errors::DimError::NotFoundError);
    }

    let resp = results
        .into_iter()
        .map(|x| {
            json!({
                "id": x.external_id,
                "title": x.title,
                "year": x.release_date.map(|x| x.year()),
                "overview": x.description,
                "poster_path": x.posters.first(),
                "genres": x.genres,
                "rating": x.rating,
                "duration": x.duration.map(|x| secs_to_pretty(x.as_secs())),
            })
        })
        .collect::<Vec<_>>();

    Ok(reply::json(&resp))
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
    user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    Progress::set(&mut tx, offset, user.id, id).await?;
    tx.commit().await?;
    Ok(StatusCode::OK)
}
