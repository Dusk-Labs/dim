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
#[patch("/<id>/match?<tmdb_id>&<media_type>")]
pub async fn rematch_mediafile(
    conn: State<'_, DbConnection>,
    log: State<'_, slog::Logger>,
    id: i32,
    tmdb_id: i32,
    media_type: String,
) -> Result<Status, errors::DimError> {
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

    Ok(Status::Ok)
}
