use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors::*;
use crate::scanners::tmdb::MediaType as ExternalMediaType;
use crate::scanners::tmdb::Tmdb;
use crate::scanners::movie::MovieMatcher;

use database::library::MediaType;
use database::media::Media;
use database::mediafile::MediaFile;

use http::status::StatusCode;

const API_KEY: &str = "38c372f5bc572c8aadde7a802638534e";

pub mod filters {
    use auth::Wrapper as Auth;
    use crate::routes::global_filters::with_state;
    use crate::core::EventTx;
    use database::DbConnection;
    use serde::Deserialize;

    use warp::reject;
    use warp::Filter;

    pub fn rematch_media_by_id(
        conn: DbConnection,
        event_tx: EventTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct RouteArgs {
            external_id: i32,
            media_type: String,
        }

        warp::path!("api" / "v1" / "media" / i64 / "match")
            .and(warp::patch())
            .and(warp::query::query::<RouteArgs>())
            .and(with_state(conn))
            .and(with_state(event_tx))
            .and(auth::with_auth())
            .and_then(
                |id, RouteArgs {
                    external_id,
                    media_type
                }: RouteArgs,
                conn: DbConnection,
                event_tx: EventTx,
                _: Auth| async move {
                    super::rematch_media(conn, event_tx, id, external_id, media_type)
                        .await
                        .map_err(|e| reject::custom(e))
                })
    }
}

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

    Media::delete(&mut tx, id).await?;

    for orphan in orphans {
        let orphan = MediaFile::get_one(&mut tx, orphan).await?;
        match target_type {
            MediaType::Movie => {
                let matcher = MovieMatcher {
                    conn: &conn,
                    event_tx: &event_tx,
                };

                matcher.inner_match(result.clone().into(), &orphan, &mut tx, Some(id)).await?;
            }
            _ => {}
        }
    }

    tx.commit().await?;

    Ok(StatusCode::OK)
}
