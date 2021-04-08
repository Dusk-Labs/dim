use crate::core::DbConnection;
use crate::errors;
use crate::routes::construct_standard;
use crate::routes::get_episode;
use crate::routes::get_season;
use crate::routes::get_top_duration;

use auth::Wrapper as Auth;
use cfg_if::cfg_if;
use database::tv::TVShow;
use database::{
    episode::Episode,
    genre::*,
    library::MediaType,
    media::Media,
    mediafile::MediaFile,
    progress::Progress,
    schema::{genre_media, media, season},
    season::Season,
};

use diesel::prelude::*;
use diesel::sql_types::Text;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

use std::fs;
use std::io;
use std::path::PathBuf;

no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

#[get("/dashboard")]
pub fn dashboard(conn: DbConnection, user: Auth) -> Result<JsonValue, errors::DimError> {
    let mut top_rated = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by((media::id, media::name))
        .order(media::rating.desc())
        .load::<Media>(conn.as_ref())?;

    top_rated.dedup_by(|a, b| a.name.eq(&b.name));

    let top_rated = top_rated
        .into_iter()
        .filter_map(|ref x| construct_standard(&conn, x, &user, false).ok())
        .take(10)
        .collect::<Vec<JsonValue>>();

    let recently_added = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by((media::id, media::name))
        .order(media::added.desc())
        .load::<Media>(conn.as_ref())?
        .into_iter()
        .filter_map(|ref x| construct_standard(&conn, x, &user, false).ok())
        .take(10)
        .collect::<Vec<JsonValue>>();

    Ok(json!({
        "TOP RATED": top_rated,
        "FRESHLY ADDED": recently_added,
    }))
}

#[get("/dashboard/banner")]
pub fn banners(conn: DbConnection, user: Auth) -> Result<Json<Vec<JsonValue>>, errors::DimError> {
    let results = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by(media::id)
        .order(RANDOM)
        .limit(10)
        .load::<Media>(conn.as_ref())?
        .iter()
        // we want to display stuff with a backdrop
        .filter(|x| x.backdrop_path.is_some())
        // make sure the backdrops have a duration
        .filter(|x| get_top_duration(&conn, &x).is_ok())
        // make sure we show medias for which the total amount watched is nil
        /*
        .filter(|x| {
            matches!(
                Progress::get_total_for_media(&conn, x, user.0.claims.get_user()),
                Err(_) | Ok(0)
            )
        })
        */
        .filter_map(|x| match x.media_type {
            Some(MediaType::Tv) => banner_for_show(&conn, &user, &x).ok(),
            Some(MediaType::Movie) => banner_for_movie(&conn, &user, &x).ok(),
            _ => unreachable!(),
        })
        .take(3)
        .collect::<Vec<_>>();

    Ok(Json(results))
}

fn banner_for_movie(
    conn: &DbConnection,
    user: &Auth,
    media: &Media,
) -> Result<JsonValue, errors::DimError> {
    let progress = Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), media.id)
        .map(|x| x.delta)
        .unwrap_or(0);
    let mediafiles = MediaFile::get_of_media(&conn, media)?;
    let media_duration = get_top_duration(&conn, media)?;

    let genres = Genre::get_by_media(conn.as_ref(), media.id)
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

fn banner_for_show(
    conn: &DbConnection,
    user: &Auth,
    media: &Media,
) -> Result<JsonValue, errors::DimError> {
    let show: TVShow = media.clone().into();
    let first_season = Season::get_first(conn, &show)?;

    let episodes = Episode::get_all_of_tv(conn, media)?;

    let mut delta_sorted = episodes
        .iter()
        .filter_map(|x| {
            Progress::get_for_media_user(conn, user.0.claims.get_user(), x.id)
                .ok()
                .and_then(|y| Some((x.clone(), y)))
        })
        .filter(|(_, x)| x.delta > 0)
        .collect::<Vec<_>>();

    delta_sorted.sort_unstable_by(|a, b| a.1.populated.partial_cmp(&b.1.populated).unwrap());

    let episode = if let Some(x) = delta_sorted.first() {
        x.0.clone()
    } else {
        Episode::get_first_for_season(conn, &first_season)?
    };

    let season = Season::get_by_id(conn, episode.seasonid)?;

    let genres = Genre::get_by_media(conn.as_ref(), media.id).map_or_else(
        |_| vec![],
        |y| y.into_iter().map(|x| x.name).collect::<Vec<_>>(),
    );

    let progress =
        Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), episode.id)
            .map(|x| x.delta)
            .unwrap_or(0);
    let duration = get_top_duration(&conn, &episode.media)?;
    let mediafiles = MediaFile::get_of_media(&conn, &episode.media)?;

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
