use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors::*;
use crate::external::ExternalQueryIntoShow;
use crate::scanner::movie;
use crate::scanner::parse_filenames;
use crate::scanner::tv_show;
use crate::scanner::MediaMatcher;
use crate::scanner::WorkUnit;

use super::media::MOVIES_PROVIDER;
use super::media::TV_PROVIDER;

use std::sync::Arc;

use dim_database::library::MediaType;
use dim_database::mediafile::MediaFile;

use tracing::error;
use tracing::info;

use http::status::StatusCode;

pub mod filters {
    use crate::core::EventTx;
    use crate::routes::global_filters::with_auth;
    use crate::routes::global_filters::with_state;
    use dim_database::user::User;
    use dim_database::DbConnection;
    use serde::Deserialize;

    use warp::reject;
    use warp::Filter;

    pub fn rematch_media_by_id(
        conn: DbConnection,
        event_tx: EventTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct RouteArgs {
            external_id: String,
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

/// FIXME: Merge this function into rematch_mediafile as theyre functionally the same fucking thing
/// except here we are matching whole media objects rather than mediafiles. This was a different
/// api in the past because the scanner wasnt intelligent enough to decouple and clean up stale
/// media objects but now that it can do that we can just rematch a matched mediafile and it will
/// work as it should.
///
/// TODO: Add ability to specify overrides like episode and season ranges.
pub async fn rematch_media(
    conn: DbConnection,
    _event_tx: EventTx,
    id: i64,
    external_id: String,
    media_type: String,
) -> Result<impl warp::Reply, DimError> {
    let Ok(media_type) = media_type.to_lowercase().try_into() else {
        return Err(DimError::InvalidMediaType);
    };

    let provider: Arc<dyn ExternalQueryIntoShow> = match media_type {
        MediaType::Movie => (*MOVIES_PROVIDER).clone(),
        MediaType::Tv => (*TV_PROVIDER).clone(),
        _ => return Err(DimError::InvalidMediaType),
    };

    let matcher = match media_type {
        MediaType::Movie => Arc::new(movie::MovieMatcher) as Arc<dyn MediaMatcher>,
        MediaType::Tv => Arc::new(tv_show::TvMatcher) as Arc<dyn MediaMatcher>,
        _ => unreachable!(),
    };

    let mut tx = conn.read().begin().await?;

    let mediafiles = match media_type {
        MediaType::Movie => MediaFile::get_of_media(&mut tx, id).await?,
        MediaType::Tv => MediaFile::get_of_show(&mut tx, id).await?,
        _ => unreachable!(),
    };

    let mediafile_ids = mediafiles.iter().map(|x| x.id).collect::<Vec<_>>();

    info!(?media_type, mediafiles = ?&mediafile_ids, "Rematching media");

    provider.search_by_id(&external_id).await.map_err(|e| {
        error!(?e, "Failed to search for tmdb_id when rematching.");
        DimError::ExternalSearchError(e)
    })?;

    drop(tx);

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;

    for mediafile in mediafiles {
        let Some((_, metadata)) = parse_filenames(IntoIterator::into_iter([&mediafile.target_file])).pop() else {
            continue;
        };

        matcher
            .match_to_id(
                &mut tx,
                provider.clone(),
                WorkUnit(mediafile.clone(), metadata),
                &external_id,
            )
            .await?;
    }

    tx.commit().await?;

    Ok(StatusCode::OK)
}
