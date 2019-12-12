use crate::core::DbConnection;
use crate::errors;
use auth::Wrapper as Auth;
use database::{
    episode::Episode,
    genre::*,
    library::MediaType,
    media::Media,
    mediafile::MediaFile,
    schema::{genre_media, media, season},
    season::Season,
};
use diesel::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};
use std::path::PathBuf;
use walkdir::WalkDir;

no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

pub fn enumerate_directory<T: AsRef<std::path::Path>>(path: T) -> Vec<String> {
    let mut dirs: Vec<String> = WalkDir::new(path)
        .max_depth(1usize)
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
    dirs
}

/// TODO: Refactor this function into something that is less fucked than this jesus
pub fn get_top_duration(conn: &DbConnection, data: &Media) -> i32 {
    match MediaFile::get_of_media(conn, data) {
        Ok(x) => {
            let mut x = x
                .into_iter()
                .filter(|x| x.corrupt != Some(true))
                .collect::<Vec<MediaFile>>();
            if !x.is_empty() {
                x.pop().unwrap().duration.unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

pub fn get_season(conn: &DbConnection, data: &Media) -> Result<Season, errors::DimError> {
    let season = season::table
        .filter(season::tvshowid.eq(data.id))
        .order(season::season_number.asc())
        .first::<Season>(&**conn)?;

    Ok(season)
}

pub fn get_episode(conn: &DbConnection, data: &Season) -> Result<Episode, errors::DimError> {
    let mut episodes = Episode::get_all_of_season(conn, data)?;

    episodes.sort_by(|a, b| a.episode.cmp(&b.episode));
    if episodes.len() < 1 {
        println!("{:?}", data);
    }

    Ok(episodes.pop().unwrap())
}

/// Function takes
pub fn construct_standard(conn: &DbConnection, data: &Media, quick: bool) -> JsonValue {
    // TODO: convert to enums
    let duration = get_top_duration(conn, data);
    let season_episode_pair =
        get_season(conn, data).and_then(|x| Ok((x.clone(), get_episode(&conn, &x))));
    let genres = Genre::get_by_media(&conn, data.id)
        .unwrap()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    if quick {
        return json!({
            "id": data.id,
            "name": data.name,
            "library_id": data.library_id
        });
    } else {
        if let Ok(pair) = season_episode_pair {
            let episode = pair.1.unwrap();
            let duration = get_top_duration(&conn, &episode.media);
            return json!({
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
                "season": pair.0.season_number
            });
        }
        return json!({
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
            "duration": duration
        });
    }
}

#[get("/dashboard")]
pub fn dashboard(conn: DbConnection, _user: Auth) -> Result<JsonValue, errors::DimError> {
    let mut top_rated = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by((media::id, media::name))
        .order(media::rating.desc())
        .load::<Media>(conn.as_ref())?;

    top_rated.dedup_by(|a, b| a.name.eq(&b.name));

    let top_rated = top_rated
        .into_iter()
        .map(|ref x| construct_standard(&conn, x, false))
        .take(10)
        .collect::<Vec<JsonValue>>();

    let recently_added = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by((media::id, media::name))
        .order(media::added.desc())
        .load::<Media>(conn.as_ref())?
        .into_iter()
        .map(|ref x| construct_standard(&conn, x, false))
        .take(10)
        .collect::<Vec<JsonValue>>();

    Ok(json!({
        "TOP RATED": top_rated,
        "FRESHLY ADDED": recently_added,
    }))
}

#[get("/dashboard/banner")]
pub fn banners(conn: DbConnection, _user: Auth) -> Result<Json<Vec<JsonValue>>, errors::DimError> {
    let sampler = Uniform::new(0, 240);
    let mut rng = rand::thread_rng();
    let results = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by(media::id)
        .order(RANDOM)
        .limit(3)
        .load::<Media>(conn.as_ref())?
        .iter()
        .filter(|x| x.backdrop_path.is_some())
        .map(|x| {
            let duration = get_top_duration(&conn, &x);
            let season_episode_pair =
                get_season(&conn, &x).and_then(|x| Ok((x.clone(), get_episode(&conn, &x))));

            let genres = Genre::get_by_media(conn.as_ref(), x.id).map_or_else(
                |_| vec![],
                |y| y.into_iter().map(|x| x.name).collect::<Vec<_>>(),
            );

            if let Ok(pair) = season_episode_pair {
                let episode = pair.1.unwrap();
                let duration = get_top_duration(&conn, &episode.media);
                return json!({
                    "id": x.id,
                    "title": x.name,
                    "year": x.year,
                    "synopsis": x.description,
                    "backdrop": x.backdrop_path,
                    "duration": duration,
                    "genres": genres,
                    "delta": sampler.sample(&mut rng),
                    "banner_caption": "WATCH SOMETHING FRESH",
                    "episode": episode.episode,
                    "season": pair.0.season_number
                });
            }
            return json!({
                "id": x.id,
                "title": x.name,
                "year": x.year,
                "synopsis": x.description,
                "backdrop": x.backdrop_path,
                "duration": duration,
                "genres": genres,
                "delta": sampler.sample(&mut rng),
                "banner_caption": "WATCH SOMETHING FRESH"
            });
        })
        .collect::<Vec<_>>();

    Ok(Json(results))
}

// TODO: Audit the security of this.
#[get("/filebrowser")]
pub fn get_root_directory_structure(_user: Auth) -> Result<Json<Vec<String>>, errors::DimError> {
    Ok(Json(enumerate_directory("/")))
}

// TODO: Audit the security of this.
#[get("/filebrowser/<path..>")]
pub fn get_directory_structure(
    path: Option<PathBuf>,
    _user: Auth,
) -> Result<Json<Vec<String>>, errors::DimError> {
    let path = path.map_or_else(
        || "/".into(),
        |x| format!("/{}", x.to_string_lossy().to_owned()),
    );

    Ok(Json(enumerate_directory(path)))
}

#[get("/search?<query>&<year>&<library_id>&<genre>&<quick>")]
pub fn search(
    conn: DbConnection,
    query: Option<&RawStr>,
    year: Option<i32>,
    library_id: Option<i32>,
    genre: Option<String>,
    quick: Option<bool>,
    _user: Auth,
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

        result = result.filter(media::name.ilike(format!("%{}%", query_string)));
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
                .map(|x| construct_standard(&conn, x, quick))
                .collect::<Vec<JsonValue>>(),
        ));
    }

    let result = result.load::<Media>(conn.as_ref())?;
    Ok(Json(
        result
            .iter()
            .map(|x| construct_standard(&conn, x, quick))
            .collect::<Vec<JsonValue>>(),
    ))
}
