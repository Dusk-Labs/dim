use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors::*;
use crate::scanners::base::patch_tv_metadata;
use crate::scanners::movie::MovieMatcher;
use crate::scanners::tmdb::MediaType as ExternalMediaType;
use crate::scanners::tmdb::Tmdb;
use crate::scanners::tv_show::TvShowMatcher;

use database::library::MediaType;
use database::media::Media;
use database::mediafile::MediaFile;

use http::status::StatusCode;

const API_KEY: &str = "38c372f5bc572c8aadde7a802638534e";

pub mod filters {
    use crate::core::EventTx;
    use crate::routes::global_filters::with_auth;
    use crate::routes::global_filters::with_state;
    use database::user::User;
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
            .and(with_state(conn.clone()))
            .and(with_state(event_tx))
            .and(with_auth(conn))
            .and_then(
                |id,
                 RouteArgs {
                     external_id,
                     media_type,
                 }: RouteArgs,
                 conn: DbConnection,
                 event_tx: EventTx,
                 _: User| async move {
                    super::rematch_media(conn, event_tx, id, external_id, media_type)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
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
    let mut result: crate::scanners::ApiMedia = tmdb
        .search_by_id(external_id)
        .await
        .map_err(|_| DimError::NotFoundError)?
        .into();

    if let ExternalMediaType::Tv = target_type {
        let mut seasons: Vec<crate::scanners::ApiSeason> = tmdb
            .get_seasons_for(result.id)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();

        for season in seasons.iter_mut() {
            season.episodes = tmdb
                .get_episodes_for(result.id, season.season_number)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect();
        }

        result.seasons = seasons;
    }

    // second decouple the media and its mediafiles.
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = database::write_tx(&mut lock).await?;

    let target = Media::get(&mut tx, id).await?;

    use database::episode::Episode;

    let orphans = match target.media_type {
        MediaType::Movie | MediaType::Episode => Media::decouple_mediafiles(&mut tx, id).await?,
        MediaType::Tv => {
            let mut orphans = vec![];
            for episode in Episode::get_all_of_tv(&mut tx, id).await? {
                orphans.append(&mut Media::decouple_mediafiles(&mut tx, episode.id).await?);
                Episode::delete(&mut tx, id).await?;
            }

            orphans
        }
    };

    Media::delete(&mut tx, id).await?;

    for orphan in orphans {
        let mut orphan = MediaFile::get_one(&mut tx, orphan).await?;
        match target_type {
            MediaType::Movie => {
                let matcher = MovieMatcher {
                    conn: &conn,
                    event_tx: &event_tx,
                };

                matcher
                    .inner_match(result.clone(), &orphan, &mut tx, Some(id))
                    .await?;
            }
            MediaType::Tv => {
                let matcher = TvShowMatcher {
                    conn: &conn,
                    event_tx: &event_tx,
                };

                patch_tv_metadata(&mut orphan, &mut tx).await?;
                matcher
                    .inner_match(result.clone(), &orphan, &mut tx, Some(id))
                    .await?;
            }

            _ => {}
        }
    }

    tx.commit().await?;

    Ok(StatusCode::OK)
}
