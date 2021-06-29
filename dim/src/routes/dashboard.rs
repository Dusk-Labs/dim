use crate::core::DbConnection;
use crate::errors;
use crate::routes::construct_standard;
use crate::routes::get_episode;
use crate::routes::get_season;
use crate::routes::get_top_duration;

use auth::Wrapper as Auth;
use cfg_if::cfg_if;

use database::episode::Episode;
use database::genre::*;
use database::library::MediaType;
use database::media::Media;
use database::mediafile::MediaFile;
use database::progress::Progress;
use database::season::Season;
use database::tv::TVShow;

use futures::stream;
use futures::StreamExt;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::json;
use serde_json::Value;

use warp::reply;
use warp::Filter;

pub fn dashboard_router(
    conn: DbConnection,
    rt: tokio::runtime::Handle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::dashboard(conn.clone(), rt.clone())
        .or(filters::banners(conn.clone()))
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use database::DbConnection;

    use warp::reject;
    use warp::Filter;

    use super::super::global_filters::with_state;

    use tokio::runtime::Handle as TokioHandle;

    use auth::Wrapper as Auth;

    pub fn dashboard(
        conn: DbConnection,
        rt: tokio::runtime::Handle,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "dashboard")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and(with_state::<TokioHandle>(rt))
            .and_then(
                |user: Auth, conn: DbConnection, rt: TokioHandle| async move {
                    super::dashboard(conn, user, rt)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn banners(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "dashboard" / "banner")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|user: Auth, conn: DbConnection| async move {
                super::banners(conn, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

pub async fn dashboard(
    conn: DbConnection,
    user: Auth,
    rt: tokio::runtime::Handle,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut top_rated = Vec::new();
    for media in Media::get_top_rated(&conn, 10).await? {
        top_rated.push(construct_standard(&conn, &media, &user).await?);
    }

    let mut recently_added = Vec::new();
    for media in Media::get_recently_added(&conn, 10).await? {
        recently_added.push(construct_standard(&conn, &media, &user).await?);
    }

    let mut continue_watching = Vec::new();
    for media in Progress::get_continue_watching(&conn, user.0.claims.get_user(), 10).await? {
        continue_watching.push(construct_standard(&conn, &media, &user).await?);
    }

    Ok(reply::json(&json!({
        "CONTINUE WATCHING": continue_watching,
        "TOP RATED": top_rated,
        "FRESHLY ADDED": recently_added,
    })))
}

pub async fn banners(conn: DbConnection, user: Auth) -> Result<impl warp::Reply, errors::DimError> {
    // NOTE (val): previous diesel implementation also checked whether `get_top_duration` return `Ok(_)`
    // and filtered out entries that didnt. Im not sure why i did that
    let mut banners = Vec::new();
    for media in Media::get_random_with(&conn, 10).await? {
        if let Ok(x) = match media.media_type {
            MediaType::Tv => banner_for_show(&conn, &user, &media).await,
            MediaType::Movie => banner_for_movie(&conn, &user, &media).await,
            _ => unreachable!(),
        } {
            banners.push(x);
        }
    }

    Ok(reply::json(&banners.iter().take(3).collect::<Vec<_>>()))
}

async fn banner_for_movie(
    conn: &DbConnection,
    user: &Auth,
    media: &Media,
) -> Result<Value, errors::DimError> {
    let progress = Progress::get_for_media_user(conn, user.0.claims.get_user(), media.id)
        .await
        .map(|x| x.delta)
        .unwrap_or(0);

    let mediafiles = MediaFile::get_of_media(conn, media.id).await?;
    let media_duration = get_top_duration(conn, media).await?;

    let genres = Genre::get_by_media(conn, media.id)
        .await
        .map(|x| x.into_iter().map(|x| x.name).collect::<Vec<_>>())
        .unwrap_or_default();

    let caption = if progress > 0 {
        "CONTINUE WATCHING"
    } else {
        "WATCH SOMETHING FRESH"
    };

    Ok(json!({
        "id": media.id,
        "title": media.name,
        "year": media.year,
        "synopsis": media.description,
        "backdrop": media.backdrop_path,
        "duration": media_duration,
        "genres": genres,
        "delta": progress,
        "banner_caption": caption,
        "versions": mediafiles.iter().map(|x| json!({
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

async fn banner_for_show(
    conn: &DbConnection,
    user: &Auth,
    media: &Media,
) -> Result<Value, errors::DimError> {
    let show: TVShow = media.clone().into();
    let first_season = Season::get_first(conn, show.id).await?;

    let episodes = Episode::get_all_of_tv(conn, media.id).await?;

    let delta_sorted = stream::iter(episodes)
        .filter_map(|x| {
            // FIXME: ugly workaround.
            let x = x.clone();
            async {
                let x = x;
                Progress::get_for_media_user(conn, user.0.claims.get_user(), x.id)
                    .await
                    .ok()
                    .and_then(|y| Some((x.clone(), y)))
            }
        })
        .collect::<Vec<_>>()
        .await;

    let mut delta_sorted = delta_sorted
        .iter()
        .filter(|(_, x)| x.delta > 0)
        .collect::<Vec<_>>();

    delta_sorted.sort_unstable_by(|a, b| a.1.populated.partial_cmp(&b.1.populated).unwrap());

    let episode = if let Some(x) = delta_sorted.first() {
        x.0.clone()
    } else {
        Episode::get_first_for_season(conn, first_season.id).await?
    };

    let season = Season::get_by_id(conn, episode.seasonid).await?;

    let genres = Genre::get_by_media(conn, media.id)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>();

    let progress = Progress::get_for_media_user(conn, user.0.claims.get_user(), episode.id)
        .await
        .map(|x| x.delta)
        .unwrap_or(0);
    let duration = get_top_duration(conn, &episode.media).await?;
    let mediafiles = MediaFile::get_of_media(conn, episode.id).await?;

    let caption = if progress > 0 {
        "CONTINUE WATCHING"
    } else {
        "WATCH SOMETHING FRESH"
    };

    Ok(json!({
        "id": media.id,
        "title": media.name,
        "year": media.year,
        "synopsis": media.description,
        "backdrop": media.backdrop_path,
        "duration": duration,
        "genres": genres,
        "delta": progress,
        "banner_caption": caption,
        "episode": episode.episode,
        "season": season.season_number,
        "versions": mediafiles.iter().map(|x| json!({
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
