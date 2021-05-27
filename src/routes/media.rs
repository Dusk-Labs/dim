use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;

/*
use crate::scanners::movie::MovieScanner;
use crate::scanners::tv_show::TvShowScanner;
use crate::scanners::MediaScanner;

use crate::scanners::tmdb::MediaType as TmdbMediaType;
use crate::scanners::tmdb::Tmdb;
*/

use auth::Wrapper as Auth;
use database::{
    episode::Episode,
    genre::Genre,
    library::MediaType,
    media::{Media, UpdateMedia},
    mediafile::MediaFile,
    progress::Progress,
    season::Season,
};

use rocket::http::Status;
use rocket::State;

use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use futures::stream;
use futures::StreamExt;
use std::sync::{Arc, Mutex};

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
#[get("/<id>")]
pub async fn get_media_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    _user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let data = Media::get(&conn, id).await?;

    let duration = match MediaFile::get_of_media(&conn, &data).await {
        Ok(mut x) => x
            .pop()
            .and_then(|x| x.duration)
            .ok_or(errors::DimError::NoneError)?,
        Err(_) => 0,
    };

    let genres = Genre::get_by_media(&conn, data.id)
        .await?
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    let duration_pretty = match data.media_type {
        Some(MediaType::Movie) | Some(MediaType::Episode) | None => {
            format!("{} min", duration / 60)
        }
        Some(MediaType::Tv) => {
            let all_eps = Episode::get_all_of_tv(&conn, &data).await?;
            let total_eps = all_eps.len();
            let total_len: i32 = stream::iter(all_eps)
                .filter_map(|x| {
                    let x = x.clone();
                    async {
                        let x = x;
                        MediaFile::get_of_media(&conn, &x.media).await.ok()
                    }
                })
                .collect::<Vec<_>>()
                .await
                .iter()
                .filter(|x| !x.is_empty())
                .filter_map(|x| x.last().and_then(|x| x.duration))
                .sum();
            format!("{} episodes | {} hr", total_eps, total_len / 3600)
        }
    };

    // FIXME: Remove the duration tag once the UI transitioned to using duration_pretty
    Ok(json!({
        "id": data.id,
        "library_id": data.library_id,
        "name": data.name,
        "description": data.description,
        "rating": data.rating,
        "year": data.year,
        "added": data.added,
        "poster_path": data.poster_path,
        "backdrop_path": data.backdrop_path,
        "media_type": data.media_type,
        "genres": genres,
        "duration": duration,
        "duration_pretty": duration_pretty,
    }))
}

/// Method mapped to `GET /api/v1/media/<id>/info` returns extra information about the media object
/// such as casts, directors, and mediafiles. This method can only be accessed by authenticated
/// users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the media we want to query info of
/// * `_user` - Auth middleware
#[get("/<id>/info")]
pub async fn get_extra_info_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let media = Media::get(&conn, id).await?;

    match media.media_type {
        Some(MediaType::Movie) | Some(MediaType::Episode) | None => {
            get_for_streamable(conn, media, user).await
        }
        Some(MediaType::Tv) => get_for_show(conn, media, user).await,
    }
}

async fn get_for_streamable(
    conn: State<'_, DbConnection>,
    media: Media,
    user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let media_files = MediaFile::get_of_media(&conn, &media).await?;

    Ok(json!({
        "progress": Progress::get_for_media_user(&conn, user.0.claims.get_user(), media.id).await
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
) -> Result<JsonValue, errors::DimError> {
    let media_files = MediaFile::get_of_media(&conn, &media.media).await?;

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
    conn: State<'_, DbConnection>,
    media: Media,
    user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let seasons = Season::get_all(&conn, media.id).await?;

    let seasons: Vec<JsonValue> = stream::iter(seasons)
        .filter_map(|x| {
            let x = x.clone();
            async {
                let x = x;
                if let Ok(eps) = Episode::get_all_of_season(&conn, &x).await {
                    let mut episodes = vec![];
                    for i in eps {
                        if let Ok(y) = get_for_episode(&conn, i, &user).await {
                            episodes.push(y);
                        }
                    }
                    return Some((x, episodes));
                }
                None
            }
        })
        .map(|(s, e)| {
            json!({
                "id": s.id,
                "season_number": s.season_number,
                "name": if s.season_number == 0 {
                    "Extras".to_string()
                } else {
                    format!("Season {}", s.season_number)
                },
                "added": s.added,
                "poster": s.poster,
                "episodes": e
            })
        })
        .collect::<Vec<_>>()
        .await;

    Ok(json!({
        "seasons": seasons,
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
#[patch("/<id>", format = "application/json", data = "<data>")]
pub async fn update_media_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    data: Json<UpdateMedia>,
    _user: Auth,
) -> Result<Status, Status> {
    match data.update(&conn, id).await {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

/// Method mapped to `DELETE /api/v1/media/<id>` is used to delete a media entry for the library.
/// ONly authenticated users can query this.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the media we want to delete
/// * `_user` - auth middleware
#[delete("/<id>")]
pub async fn delete_media_by_id(
    conn: State<'_, DbConnection>,
    id: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    Media::delete(&conn, id).await?;
    Ok(Status::Ok)
}

/// Method mapped to `GET /api/v1/media/tmdb_search` is used to quickly search TMDB based on 3
/// params, one of which is optional. This is used client side in the rematch utility
///
/// # Arguments
/// * `query` - the query we want to send to tmdb, ie movie title, tv show title
/// * `year` - optional parameter specifying the release year of the media we want to look up
/// * `media_type` - parameter that tells us what media type we are querying, ie movie or tv show
#[get("/tmdb_search?<query>&<year>&<media_type>")]
pub fn tmdb_search(
    query: String,
    year: Option<i32>,
    media_type: String,
    _user: Auth,
) -> Result<JsonValue, errors::DimError> {
    /*
    let media_type = match media_type.as_ref() {
        "movie" => TmdbMediaType::Movie,
        "tv" => TmdbMediaType::Tv,
        _ => return Err(errors::DimError::InvalidMediaType),
    };

    let mut tmdb_session = Tmdb::new("38c372f5bc572c8aadde7a802638534e".to_string(), media_type);

    Ok(json!(tmdb_session.search_many(query, year, 15)))
    */
    todo!()
}

/// Method mapped to `PATCH /api/v1/media/<id>/match` used to rematch a media entry to a new tmdb
/// id passed in as the paramter `tmdb_id`.
///
/// # Arguments
/// * `conn` - database connection
/// * `log` - logger
/// * `event_tx` - websocket channel over which we dispatch a event notifying other clients of the
/// new metadata
/// * `id` - id of the media we want to rematch
/// * `tmdb_id` - the tmdb id of the proper metadata we want to fetch for the media
#[patch("/<id>/match?<tmdb_id>")]
pub async fn rematch(
    conn: State<'_, DbConnection>,
    log: State<'_, slog::Logger>,
    event_tx: State<'_, Arc<Mutex<EventTx>>>,
    id: i32,
    tmdb_id: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    /*
    let media = Media::get(&conn, id)?;
    let tx = event_tx.lock().unwrap();
    // let scanner = IterativeScanner::new(media.library_id, log.get().clone(), tx.clone())?;
    std::thread::spawn(move || {
        scanner.match_media_to_tmdb_id(media, tmdb_id);
    });
    Ok(Status::Ok)
    */
    Ok(Status::ServiceUnavailable)
}

/// Method mapped to `POST /api/v1/media/<id>/progress` is used to map progress for a certain media
/// to the user. This is useful for remembering progress for a movie etc.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` -
#[post("/<id>/progress?<offset>")]
pub async fn map_progress(
    conn: State<'_, DbConnection>,
    id: i32,
    offset: i32,
    user: Auth,
) -> Result<Status, errors::DimError> {
    Progress::set(&conn, offset, user.0.claims.get_user(), id).await?;
    Ok(Status::Ok)
}
