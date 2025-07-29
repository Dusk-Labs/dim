use crate::errors;
use crate::scanners::tmdb::MediaType as TmdbMediaType;
use crate::scanners::tmdb::Tmdb;
use crate::scanners::MetadataAgent;

use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

/// Method mapped to `GET /api/v1/external/tmdb_search` is used to quickly search TMDB based on 3
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
    let media_type = match media_type.as_ref() {
        "movie" => TmdbMediaType::Movie,
        "tv" => TmdbMediaType::Tv,
        _ => return Err(errors::DimError::InvalidMediaType),
    };

    let mut tmdb_session = Tmdb::new("38c372f5bc572c8aadde7a802638534e".to_string(), media_type);

    Ok(json!(tmdb_session.search_many(query, year, 15)))
}
