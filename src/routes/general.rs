use crate::core::DbConnection;
use crate::errors;
use auth::Wrapper as Auth;
use cfg_if::cfg_if;
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

// Necessary to emulate ilike.
sql_function!(fn upper(x: Text) -> Text);

pub fn enumerate_directory<T: AsRef<std::path::Path>>(path: T) -> io::Result<Vec<String>> {
    let mut dirs: Vec<String> = fs::read_dir(path)?
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| {
            !x.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
                && !x.path().is_file()
        })
        .map(|x| x.path().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    dirs.sort();
    Ok(dirs)
}

/// TODO: Refactor this function into something that is less fucked than this jesus
pub fn get_top_duration(conn: &DbConnection, data: &Media) -> Result<i32, errors::DimError> {
    match MediaFile::get_of_media(conn, data) {
        Ok(x) => {
            let mut x = x
                .into_iter()
                .filter(|x| x.corrupt != Some(true))
                .collect::<Vec<MediaFile>>();
            if !x.is_empty() {
                Ok(x.pop()?.duration?)
            } else {
                Ok(0)
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

/// Function takes
pub fn construct_standard(
    conn: &DbConnection,
    data: &Media,
    user: &Auth,
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
                "progress": progress
            }));
        }
        let progress =
            Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), data.id)
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
        .filter(|x| x.backdrop_path.is_some())
        .filter_map(|x| get_top_duration(&conn, &x).and_then(|y| Ok((x, y))).ok()) // filter for banners with a duration
        .map(|(media, media_duration)| {
            let season_episode_pair =
                get_season(&conn, &media).and_then(|x| Ok((x.clone(), get_episode(&conn, &x))));

            let genres = Genre::get_by_media(conn.as_ref(), media.id).map_or_else(
                |_| vec![],
                |y| y.into_iter().map(|x| x.name).collect::<Vec<_>>(),
            );

            if let Ok(pair) = season_episode_pair {
                let episode = pair.1.unwrap();
                let progress = Progress::get_for_media_user(
                    conn.as_ref(),
                    user.0.claims.get_user(),
                    episode.id,
                )
                .unwrap_or(0);
                let duration = get_top_duration(&conn, &episode.media).unwrap();
                return json!({
                    "id": media.id,
                    "title": media.name,
                    "year": media.year,
                    "synopsis": media.description,
                    "backdrop": media.backdrop_path,
                    "duration": duration,
                    "genres": genres,
                    "delta": progress,
                    "banner_caption": "WATCH SOMETHING FRESH",
                    "episode": episode.episode,
                    "season": pair.0.season_number
                });
            }
            let progress =
                Progress::get_for_media_user(conn.as_ref(), user.0.claims.get_user(), media.id)
                    .unwrap_or(0);
            return json!({
                "id": media.id,
                "title": media.name,
                "year": media.year,
                "synopsis": media.description,
                "backdrop": media.backdrop_path,
                "duration": media_duration,
                "genres": genres,
                "delta": progress,
                "banner_caption": "WATCH SOMETHING FRESH"
            });
        })
        .take(3)
        .collect::<Vec<_>>();

    Ok(Json(results))
}

#[get("/filebrowser")]
pub fn get_root_directory_structure(_user: Auth) -> Result<Json<Vec<String>>, errors::DimError> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            Ok(Json(enumerate_directory(r"C:\")?))
        } else {
            Ok(Json(enumerate_directory("/")?))
        }
    }
}

#[get("/filebrowser/<path..>")]
pub fn get_directory_structure(
    path: PathBuf,
    _user: Auth,
) -> Result<Json<Vec<String>>, errors::DimError> {
    Ok(Json(enumerate_directory(path)?))
}

#[get("/search?<query>&<year>&<library_id>&<genre>&<quick>")]
pub fn search(
    conn: DbConnection,
    query: Option<&RawStr>,
    year: Option<i32>,
    library_id: Option<i32>,
    genre: Option<String>,
    quick: Option<bool>,
    user: Auth,
) -> Result<Json<Vec<JsonValue>>, errors::DimError> {
    let quick = quick.unwrap_or(false);
    let mut result = media::table.into_boxed();

    result = result.filter(media::media_type.ne(MediaType::Episode));

    if let Some(query_string) = query {
        let query_string = query_string
            .url_decode_lossy()
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice()
            .join("% %");

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                result = result.filter(media::name.ilike(format!("%{}%", query_string)));
            } else {
                result = result.filter(upper(media::name).like(format!("%{}%", query_string)));
            }
        }
    }

    if let Some(x) = year {
        result = result.filter(media::year.eq(x));
    }

    if let Some(x) = library_id {
        result = result.filter(media::library_id.eq(x));
    }

    if let Some(x) = genre {
        let genre_row = Genre::get_by_name(conn.as_ref(), x)?.id;

        let new_result = result
            .inner_join(genre_media::table)
            .filter(genre_media::genre_id.eq(genre_row));

        let new_result = new_result.load::<Media>(conn.as_ref())?;
        return Ok(Json(
            new_result
                .iter()
                .filter_map(|x| construct_standard(&conn, x, &user, quick).ok())
                .collect::<Vec<JsonValue>>(),
        ));
    }

    let result = result.load::<Media>(conn.as_ref())?;
    Ok(Json(
        result
            .iter()
            .filter_map(|x| construct_standard(&conn, x, &user, quick).ok())
            .collect::<Vec<JsonValue>>(),
    ))
}
