use crate::core::DbConnection;
use crate::errors;
use cfg_if::cfg_if;

use database::episode::Episode;
use database::genre::*;
use database::library::MediaType;
use database::media::Media;
use database::mediafile::MediaFile;
use database::progress::Progress;
use database::schema::season;
use database::season::Season;

use diesel::prelude::*;
use diesel::sql_types::Text;
use tokio_diesel::*;

use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::json;
use serde_json::Value as JsonValue;

use warp::reply::json;
use warp::reply::Json;

pub mod auth;
// pub mod catchers;
pub mod dashboard;
// pub mod general;
pub mod library;
// pub mod media;
// pub mod mediafile;
pub mod statik;
// pub mod stream;
// pub mod tv;

pub mod global_filters {
    use database::DbConnection;

    use std::convert::Infallible;
    use warp::Filter;

    pub fn with_db(
        conn: DbConnection,
    ) -> impl Filter<Extract = (DbConnection,), Error = Infallible> + Clone {
        warp::any().map(move || conn.clone())
    }

    pub fn with_state<T: Send + Clone>(
        state: T,
    ) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
        warp::any().map(move || state.clone())
    }

    pub async fn handle_rejection(
        err: warp::reject::Rejection,
    ) -> Result<impl warp::Reply, warp::reject::Rejection> {
        if let Some(e) = err.find::<crate::errors::AuthError>() {
            return Ok(e.clone());
        }

        Err(err)
    }
}

pub async fn get_top_duration(conn: &DbConnection, data: &Media) -> Result<i32, errors::DimError> {
    match MediaFile::get_of_media(conn, data).await {
        Ok(files) => {
            let last = files
                .iter()
                .filter(|file| !matches!(file.corrupt, Some(true)))
                .last();

            match last {
                None => Ok(0),
                Some(file) => file.duration.ok_or(errors::DimError::NoneError),
            }
        }
        Err(_) => Ok(0),
    }
}

pub async fn get_season(conn: &DbConnection, data: &Media) -> Result<Season, errors::DimError> {
    let season = season::table
        .filter(season::tvshowid.eq(data.id))
        .order(season::season_number.asc())
        .first_async::<Season>(&conn)
        .await?;

    Ok(season)
}

pub async fn get_episode(conn: &DbConnection, data: &Season) -> Result<Episode, errors::DimError> {
    let mut episodes = Episode::get_all_of_season(conn, data).await?;
    episodes.sort_by(|b, a| a.episode.cmp(&b.episode));

    episodes.pop().ok_or(errors::DimError::NoneError)
}

pub async fn construct_standard_quick(data: &Media) -> Result<JsonValue, errors::DimError> {
    Ok(json!({
        "id": data.id,
        "name": data.name,
        "library_id": data.library_id
    }))
}

pub async fn construct_standard(
    conn: &DbConnection,
    data: &Media,
    user: &::auth::Wrapper,
) -> Result<JsonValue, errors::DimError> {
    // TODO: convert to enums
    let duration = get_top_duration(conn, data).await?;
    let season = get_season(conn, data).await;

    let genres = Genre::get_by_media(&conn, data.id)
        .await?
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    if let Ok(season) = season {
        if let Ok(episode) = get_episode(conn, &season).await {
            let progress = Progress::get_for_media_user(conn, user.0.claims.get_user(), episode.id)
                .await
                .map(|x| x.delta)
                .unwrap_or(0);

            let duration = get_top_duration(conn, &episode.media).await?;

            return Ok(json!({
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
                "episode": episode.episode,
                "season": season.season_number,
                "progress": progress,
            }));
        }
    }

    let progress = Progress::get_for_media_user(conn, user.0.claims.get_user(), data.id)
        .await
        .map(|x| x.delta)
        .unwrap_or(0);
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
        "progress": progress,
    }))
}
