use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors::*;
use crate::scanners::tmdb::MediaType as ExternalMediaType;
use crate::scanners::tmdb::Tmdb;
use crate::scanners::movie::MovieMatcher;

use auth::Wrapper as Auth;

use database::library::MediaType;
use database::media::Media;
use database::mediafile::MediaFile;

use http::status::StatusCode;

const API_KEY: &str = "38c372f5bc572c8aadde7a802638534e";

pub async fn rematch_media(
    conn: DbConnection,
    event_tx: EventTx,
    id: i64,
    external_id: i32,
    media_type: String,
) -> Result<impl warp::Reply, DimError> {
    // first fetch the data from tmdb
    let target_type = match media_type.to_lowercase().as_ref() {
        "movie" => ExternalMediaType::Movie,
        "tv" => ExternalMediaType::Tv,
        _ => return Err(DimError::InvalidMediaType),
    };

    let mut tmdb = Tmdb::new(API_KEY.into(), target_type);
    let result = tmdb
        .search_by_id(external_id)
        .await
        .map_err(|_| DimError::NotFoundError)?;

    // second decouple the media and its mediafiles.
    let mut tx = conn.write().begin().await?;

    let target = Media::get(&mut tx, id).await?;

    let orphans = match target.media_type {
        MediaType::Movie | MediaType::Episode => Media::decouple_mediafiles(&mut tx, id).await?,
        MediaType::Tv => unimplemented!(),
    };

    for orphan in orphans {
        let orphan = MediaFile::get_one(&mut tx, orphan).await?;
        match target_type {
            MediaType::Movie => {
                let matcher = MovieMatcher {
                    conn: &conn,
                    event_tx: &event_tx,
                };

                matcher.inner_match(result.clone().into(), &orphan, &mut tx).await?;
            }
            _ => {}
        }
    }

    tx.commit().await?;

    Ok(StatusCode::OK)
}
