use crate::core::DbConnection;
use crate::errors;
use cfg_if::cfg_if;
use database::{
    episode::Episode, genre::*, library::MediaType, media::Media, mediafile::MediaFile,
    progress::Progress, schema::season, season::Season,
};
use diesel::prelude::*;
use diesel::sql_types::Text;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

use std::fs;
use std::io;
use std::path::PathBuf;

pub mod auth;
pub mod dashboard;
pub mod general;
pub mod library;
pub mod media;
pub mod mediafile;
pub mod statik;
pub mod stream;
pub mod tv;

pub fn get_top_duration(conn: &DbConnection, data: &Media) -> Result<i32, errors::DimError> {
    match MediaFile::get_of_media(conn, data) {
        Ok(files) => {
            let last = files
                .iter()
                .filter(|file| !matches!(file.corrupt, Some(true)))
                .last();

            match last {
                None => Ok(0),
                Some(file) => Ok(file.duration?),
            }
        }
        Err(_) => Ok(0),
    }
}

pub fn get_season(conn: &DbConnection, data: &Media) -> Result<Season, errors::DimError> {
    let season = season::table
        .filter(season::tvshowid.eq(data.id))
        .order(season::season_number.asc())
        .first::<Season>(conn.as_ref())?;

    Ok(season)
}

pub fn get_episode(conn: &DbConnection, data: &Season) -> Result<Episode, errors::DimError> {
    let mut episodes = Episode::get_all_of_season(conn, data)?;
    episodes.sort_by(|b, a| a.episode.cmp(&b.episode));

    Ok(episodes.pop()?)
}

pub fn construct_standard(
    conn: &DbConnection,
    data: &Media,
    user: &::auth::Wrapper,
    quick: bool,
) -> Result<JsonValue, errors::DimError> {
    // TODO: convert to enums
    let duration = get_top_duration(conn, data)?;
    let season_episode_pair =
        get_season(conn, data).and_then(|x| Ok((x.clone(), get_episode(&conn, &x))));
    let genres = Genre::get_by_media(&conn, data.id)?
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    if quick {
        Ok(json!({
            "id": data.id,
            "name": data.name,
            "library_id": data.library_id
        }))
    } else {
        if let Ok(pair) = season_episode_pair {
            let episode = pair.1?;
            let progress =
                Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), episode.id)
                    .map(|x| x.delta)
                    .unwrap_or(0);
            let duration = get_top_duration(&conn, &episode.media)?;
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
                "season": pair.0.season_number,
                "progress": progress,
            }));
        }
        let progress =
            Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), data.id)
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
}
