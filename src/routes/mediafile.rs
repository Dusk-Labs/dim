use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;

use auth::Wrapper as Auth;
use database::mediafile::MediaFile;

use rocket::{http::Status, State};
use rocket_contrib::{json, json::JsonValue};
use std::sync::{Arc, Mutex};

/// Method mapped to `GET /api/v1/mediafile/<id>/` is used to get information about a mediafile by its id
#[get("/<id>")]
pub async fn get_mediafile_info(
    conn: State<'_, DbConnection>,
    log: State<'_, slog::Logger>,
    id: i32,
    _user: Auth,
) -> Result<JsonValue, errors::DimError> {
    let mediafile = MediaFile::get_one(&conn, id).await?;

    Ok(json!({
        "id": mediafile.id,
        "media_id": mediafile.media_id,
        "library_id": mediafile.library_id,
        "raw_name": mediafile.raw_name,
    }))
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
pub async fn rematch_mediafile(
    conn: State<'_, DbConnection>,
    log: State<'_, slog::Logger>,
    event_tx: State<'_, Arc<Mutex<EventTx>>>,
    id: i32,
    tmdb_id: i32,
) -> Result<Status, errors::DimError> {
    /*
    let mediafile = MediaFile::get_one(conn.as_ref(), id)?;
    let tx = event_tx.lock().unwrap();
    let scanner = IterativeScanner::new(mediafile.library_id, log.get().clone(), tx.clone())?;
    std::thread::spawn(move || {
        scanner.match_mediafile_to_tmdb_id(mediafile, tmdb_id);
    });
    Ok(Status::Ok)
    */
    Ok(Status::ServiceUnavailable)
}
