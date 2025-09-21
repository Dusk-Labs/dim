use crate::AppState;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Extension;

use chrono::Datelike;

use dim_core::scanner::movie;
use dim_core::scanner::parse_filenames;
use dim_core::scanner::tv_show;
use dim_core::scanner::MediaMatcher;
use dim_core::scanner::WorkUnit;
use dim_core::tree;

use dim_database::compact_mediafile::CompactMediafile;
use dim_database::episode::Episode;
use dim_database::genre::Genre;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::media::UpdateMedia;
use dim_database::mediafile::MediaFile;
use dim_database::progress::Progress;
use dim_database::user::User;
use dim_database::DatabaseError;

use dim_extern_api::tmdb::TMDBMetadataProvider;
use dim_extern_api::ExternalQueryIntoShow;

use dim_utils::json;
use dim_utils::secs_to_pretty;

use http::StatusCode;

use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

use tracing::error;
use tracing::info;

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum Error {
    /// Not Found.
    NotFoundError,
    /// Invalid media type.
    InvalidMediaType,
    /// Not logged in.
    InvalidCredentials,
    /// database: {0}
    Database(#[from] DatabaseError),
    /// Failed to search for tmdb_id when rematching: {0}
    ExternalSearchError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::NotFoundError => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::ExternalSearchError(_) => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            Self::InvalidMediaType => {
                (StatusCode::NOT_ACCEPTABLE, self.to_string()).into_response()
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

pub const API_KEY: &str = "38c372f5bc572c8aadde7a802638534e";
pub const MOVIES_PROVIDER: Lazy<Arc<dyn ExternalQueryIntoShow>> =
    Lazy::new(|| Arc::new(TMDBMetadataProvider::new(&API_KEY).movies()));
pub const TV_PROVIDER: Lazy<Arc<dyn ExternalQueryIntoShow>> =
    Lazy::new(|| Arc::new(TMDBMetadataProvider::new(&API_KEY).tv_shows()));

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
    Path(id): Path<i64>,
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
) -> Result<Response, Error> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
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
            dim_core::utils::codec_pretty(x.codec.as_deref().unwrap_or("Unknown"))
        );

        let audio_lang = x.audio_language.as_deref().unwrap_or("Unknown");
        let audio_codec = dim_core::utils::codec_pretty(x.audio.as_deref().unwrap_or("Unknown"));
        let audio_ch = dim_core::utils::channels_pretty(x.channels.unwrap_or(2));

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
    Ok(axum::response::Json(&json!({
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
    }))
    .into_response())
}

pub async fn get_media_files(
    Path(id): Path<i64>,
    State(AppState { conn, .. }): State<AppState>,
) -> Result<Response, Error> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
    let media_type = Media::media_mediatype(&mut tx, id).await?;

    let mediafiles = match media_type {
        MediaType::Tv => MediaFile::get_of_show(&mut tx, id).await?,
        MediaType::Episode | MediaType::Movie => MediaFile::get_of_media(&mut tx, id).await?,
    };

    Ok(axum::response::Json(json!(&mediafiles)).into_response())
}

/// # GET `/api/v1/media/<id>/tree`
/// Method mappedReturns a tree of mediafiles for a given media object.
///
/// # Authentication
/// Method requires standard authentication.
pub async fn get_mediafile_tree(
    Path(id): Path<i64>,
    State(AppState { conn, .. }): State<AppState>,
) -> Result<Response, Error> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
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
    struct TreeResponse {
        count: usize,
        files: Vec<tree::Entry<Record>>,
    }

    let entries = match entry {
        tree::Entry::Directory { files, .. } => files,
        _ => unreachable!(),
    };

    Ok(axum::response::Json(json!(&TreeResponse {
        files: entries,
        count,
    }))
    .into_response())
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
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
    Json(data): Json<UpdateMedia>,
) -> Result<impl IntoResponse, Error> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;
    let status = if data.update(&mut tx, id).await.is_ok() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_MODIFIED
    };

    tx.commit().await.map_err(DatabaseError::from)?;

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
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;
    Media::delete(&mut tx, id).await?;
    tx.commit().await.map_err(DatabaseError::from)?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct TmdbSearchParams {
    query: String,
    year: Option<i32>,
    media_type: String,
}

/// Method mapped to `GET /api/v1/media/tmdb_search` is used to quickly search TMDB based on 3
/// params, one of which is optional. This is used client side in the rematch utility
///
/// # Arguments
/// * `query` - the query we want to send to tmdb, ie movie title, tv show title
/// * `year` - optional parameter specifying the release year of the media we want to look up
/// * `media_type` - parameter that tells us what media type we are querying, ie movie or tv show
pub async fn tmdb_search(Query(params): Query<TmdbSearchParams>) -> Result<Response, Error> {
    let Ok(media_type) = params.media_type.to_lowercase().try_into() else {
        return Err(Error::InvalidMediaType);
    };

    let provider = match media_type {
        MediaType::Movie => (*MOVIES_PROVIDER).clone(),
        MediaType::Tv => (*TV_PROVIDER).clone(),
        _ => return Err(Error::InvalidMediaType),
    };

    let results = provider
        .search(&params.query, params.year)
        .await
        .map_err(|_| Error::NotFoundError)?;

    if results.is_empty() {
        return Err(Error::NotFoundError);
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

    Ok(axum::response::Json(json!(&resp)).into_response())
}

#[derive(Deserialize)]
pub struct ProgressParams {
    offset: i64,
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
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<ProgressParams>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;
    Progress::set(&mut tx, params.offset, user.id, id).await?;
    tx.commit().await.map_err(DatabaseError::from)?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct RematchMediaParams {
    external_id: String,
    media_type: String,
}

/// FIXME: Merge this function into rematch_mediafile as theyre functionally the same fucking thing
/// except here we are matching whole media objects rather than mediafiles. This was a different
/// api in the past because the scanner wasnt intelligent enough to decouple and clean up stale
/// media objects but now that it can do that we can just rematch a matched mediafile and it will
/// work as it should.
///
/// TODO: Add ability to specify overrides like episode and season ranges.
pub async fn rematch_media_by_id(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
    Json(params): Json<RematchMediaParams>,
) -> Result<impl IntoResponse, Error> {
    let Ok(media_type) = params.media_type.to_lowercase().try_into() else {
        return Err(Error::InvalidMediaType);
    };

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

    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;

    let mediafiles = match media_type {
        MediaType::Movie => MediaFile::get_of_media(&mut tx, id).await?,
        MediaType::Tv => MediaFile::get_of_show(&mut tx, id).await?,
        _ => unreachable!(),
    };

    let mediafile_ids = mediafiles.iter().map(|x| x.id).collect::<Vec<_>>();

    info!(?media_type, mediafiles = ?&mediafile_ids, "Rematching media");

    provider
        .search_by_id(&params.external_id)
        .await
        .map_err(|e| {
            error!(?e, "Failed to search for tmdb_id when rematching.");
            Error::ExternalSearchError(e.to_string())
        })?;

    drop(tx);

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(DatabaseError::from)?;

    for mediafile in mediafiles {
        let Some((_, metadata)) =
            parse_filenames(IntoIterator::into_iter([&mediafile.target_file])).pop()
        else {
            continue;
        };

        matcher
            .match_to_id(
                &mut tx,
                provider.clone(),
                WorkUnit(mediafile.clone(), metadata),
                &params.external_id,
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
