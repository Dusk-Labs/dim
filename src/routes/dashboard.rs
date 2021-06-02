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
use database::schema::genre_media;
use database::schema::media;
use database::schema::season;
use database::season::Season;
use database::tv::TVShow;

use diesel::prelude::*;
use diesel::sql_types::Text;
use tokio_diesel::*;

use futures::stream;
use futures::StreamExt;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::json;
use serde_json::Value;

use warp::reply;
use warp::Filter;

no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

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
    let mut top_rated = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by((media::id, media::name))
        .order(media::rating.desc())
        .load_async::<Media>(&conn)
        .await?;

    top_rated.dedup_by(|a, b| a.name.eq(&b.name));

    let top_rated = stream::iter(top_rated)
        .filter_map(|x| {
            let x = x.clone();
            let conn = &conn;
            let user = &user;
            async move { construct_standard(&conn, &x, &user).await.ok() }
        })
        .take(10)
        .collect::<Vec<Value>>()
        .await;

    let recently_added = stream::iter(
        media::table
            .filter(media::media_type.ne(MediaType::Episode))
            .group_by((media::id, media::name))
            .order(media::added.desc())
            .load_async::<Media>(&conn)
            .await?,
    )
    .filter_map(|x| {
        let conn = &conn;
        let x = x.clone();
        let user = &user;
        async move { construct_standard(&conn, &x, &user).await.ok() }
    })
    .take(10)
    .collect::<Vec<Value>>()
    .await;

    Ok(reply::json(&json!({
        "TOP RATED": top_rated,
        "FRESHLY ADDED": recently_added,
    })))
}

// FIXME: Basically this function purely async just kinda fails to compile because of various
// lifetime and opaque type issues. The bigger problem is that the `rocket::get` macro hides the
// error and makes it unreadable and removing the macro gets rid of the issue completely.
pub async fn banners(conn: DbConnection, user: Auth) -> Result<impl warp::Reply, errors::DimError> {
    // make sure we show medias for which the total amount watched is nil
    /*
       .filter(|x| {
           matches!(
               Progress::get_total_for_media(&conn, x, user.0.claims.get_user()),
               Err(_) | Ok(0)
           )
       })
    */

    let results = stream::iter(
        media::table
            .filter(media::media_type.ne(MediaType::Episode))
            .group_by(media::id)
            .order(RANDOM)
            .limit(10)
            .load_async::<Media>(&conn)
            .await?,
    )
    .filter(|x| {
        let x = x.clone();
        async {
            let x = x;
            get_top_duration(&conn, &x).await.is_ok()
        }
    })
    .collect::<Vec<Media>>()
    .await;

    let banners = stream::iter(results)
        .filter_map(|x| {
            let x = x.clone();
            async {
                let x = x;
                match x.media_type {
                    Some(MediaType::Tv) => banner_for_show(&conn, &user, &x).await.ok(),
                    Some(MediaType::Movie) => banner_for_movie(&conn, &user, &x).await.ok(),
                    _ => unreachable!(),
                }
            }
        })
        .take(3)
        .collect::<Vec<Value>>()
        .await;

    Ok(reply::json(&banners))
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

    let mediafiles = MediaFile::get_of_media(conn, media).await?;
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
    let first_season = Season::get_first(conn, &show).await?;

    let episodes = Episode::get_all_of_tv(conn, media).await?;

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
        Episode::get_first_for_season(conn, &first_season).await?
    };

    let season = Season::get_by_id(conn, episode.seasonid).await?;

    let genres = Genre::get_by_media(conn, media.id).await.map_or_else(
        |_| vec![],
        |y| y.into_iter().map(|x| x.name).collect::<Vec<_>>(),
    );

    let progress = Progress::get_for_media_user(conn, user.0.claims.get_user(), episode.id)
        .await
        .map(|x| x.delta)
        .unwrap_or(0);
    let duration = get_top_duration(conn, &episode.media).await?;
    let mediafiles = MediaFile::get_of_media(conn, &episode.media).await?;

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
