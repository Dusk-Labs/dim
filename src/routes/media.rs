use crate::{
    core::{DbConnection, EventTx},
    errors,
    scanners::{
        iterative_parser::IterativeScanner,
        tmdb_api::{MediaType as TmdbMediaType, TMDbSearch},
        APIExec,
    },
};
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
use rocket::{http::Status, State};
use rocket_contrib::{
    json,
    json::{Json, JsonValue},
};
use rocket_slog::SyncLogger;
use std::sync::{Arc, Mutex};

/// Method mapped to `GET /api/v1/media/<id>` returns info about a media based on the id queried.
/// This method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the media we want to query info of
/// * `_user` - Auth middleware
#[get("/<id>")]
pub fn get_media_by_id(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let data = Media::get(conn.as_ref(), id)?;

    let duration = match MediaFile::get_of_media(conn.as_ref(), &data) {
        Ok(mut x) => x.pop()?.duration?,
        Err(_) => 0,
    };

    let genres = Genre::get_by_media(conn.as_ref(), data.id)?
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

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
pub fn get_extra_info_by_id(
    conn: DbConnection,
    id: i32,
    user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let media = Media::get(conn.as_ref(), id)?;

    match media.media_type {
        Some(MediaType::Movie) | Some(MediaType::Episode) | None => {
            get_for_streamable(conn, media, user)
        }
        Some(MediaType::Tv) => get_for_show(conn, media, user),
    }
}

fn get_for_streamable(
    conn: DbConnection,
    media: Media,
    user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let media_files = MediaFile::get_of_media(conn.as_ref(), &media)?;

    Ok(json!({
        "progress": Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), media.id).unwrap_or(0),
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

fn get_for_episode(
    conn: &DbConnection,
    media: Episode,
    user: &Auth,
) -> Result<JsonValue, errors::DimError> {
    let media_files = MediaFile::get_of_media(conn.as_ref(), &media.media)?;

    Ok(json!({
        "progress": Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), media.id).unwrap_or(0),
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

fn get_for_show(
    conn: DbConnection,
    media: Media,
    user: Auth,
) -> Result<JsonValue, errors::DimError> {
    Ok(json!({
        "seasons":
            Season::get_all(conn.as_ref(), media.id)?
                .into_iter()
                .filter_map(|x| Episode::get_all_of_season(&conn, &x).map(|y| (x, y)).ok())
                .map(|(x, y)| {
                    json!({
                        "id": x.id,
                        "season_number": x.season_number,
                        "added": x.added,
                        "poster": x.poster,
                        "episodes": y.into_iter().filter_map(|z| get_for_episode(&conn, z, &user).ok()).collect::<Vec<JsonValue>>()
                    })
                })
                .collect::<Vec<JsonValue>>()
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
pub fn update_media_by_id(
    conn: DbConnection,
    id: i32,
    data: Json<UpdateMedia>,
    _user: Auth,
) -> Result<Status, Status> {
    match data.update(conn.as_ref(), id) {
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
pub fn delete_media_by_id(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    Media::delete(conn.as_ref(), id)?;
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
) -> Result<JsonValue, errors::DimError> {
    let mut tmdb_session = TMDbSearch::new("38c372f5bc572c8aadde7a802638534e");

    let media_type = match media_type.as_ref() {
        "movie" => TmdbMediaType::Movie,
        "tv" => TmdbMediaType::Tv,
        _ => return Err(errors::DimError::InvalidMediaType),
    };

    Ok(json!(
        tmdb_session.search_many(query, year, media_type, 15usize)
    ))
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
pub fn rematch(
    conn: DbConnection,
    log: SyncLogger,
    event_tx: State<Arc<Mutex<EventTx>>>,
    id: i32,
    tmdb_id: i32,
) -> Result<Status, errors::DimError> {
    let media = Media::get(conn.as_ref(), id)?;
    let tx = event_tx.lock().unwrap();
    let scanner = IterativeScanner::new(media.library_id, log.get().clone(), tx.clone())?;
    std::thread::spawn(move || {
        scanner.match_media_to_tmdb_id(media, tmdb_id);
    });
    Ok(Status::Ok)
}

/// Method mapped to `PATCH /api/v1/mediafile/<id>/match` used to match a unmatched(orphan)
/// mediafile to a tmdb id.
///
/// # Arguments
/// * `conn` - database connection
/// * `log` - logger
/// * `event_tx` - websocket channel over which we dispatch a event notifying other clients of the
/// new metadata
/// * `id` - id of the orphan mediafile we want to rematch
/// * `tmdb_id` - the tmdb id of the proper metadata we want to fetch for the media
// Part of /api/v1/mediafile route
#[patch("/<id>/match?<tmdb_id>")]
pub fn rematch_mediafile(
    conn: DbConnection,
    log: SyncLogger,
    event_tx: State<Arc<Mutex<EventTx>>>,
    id: i32,
    tmdb_id: i32,
) -> Result<Status, errors::DimError> {
    let mediafile = MediaFile::get_one(conn.as_ref(), id)?;
    let tx = event_tx.lock().unwrap();
    let scanner = IterativeScanner::new(mediafile.library_id, log.get().clone(), tx.clone())?;
    std::thread::spawn(move || {
        scanner.match_mediafile_to_tmdb_id(mediafile, tmdb_id);
    });
    Ok(Status::Ok)
}

/// Method mapped to `POST /api/v1/media/<id>/progress` is used to map progress for a certain media
/// to the user. This is useful for remembering progress for a movie etc.
///
#[post("/<id>/progress?<offset>")]
pub fn map_progress(
    conn: DbConnection,
    id: i32,
    offset: i32,
    user: Auth,
) -> Result<Status, errors::DimError> {
    let _ = Progress::set(conn.as_ref(), offset, user.0.claims.get_user(), id)?;
    Ok(Status::Ok)
}
