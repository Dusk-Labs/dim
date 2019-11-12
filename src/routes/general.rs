use crate::core::DbConnection;
use auth::Wrapper as Auth;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dim_database::episode::Episode;
use dim_database::genre::*;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use dim_database::season::Season;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use std::path::PathBuf;
use walkdir::WalkDir;

/// TODO: Refactor this function into something that is less fucked than this jesus
pub fn get_top_duration(conn: &DbConnection, data: &Media) -> i32 {
    match MediaFile::get_of_media(conn, data) {
        Ok(x) => {
            let mut x = x
                .iter()
                .filter(|x| x.corrupt != Some(true))
                .collect::<Vec<&MediaFile>>();
            if x.len() > 0 {
                x.pop().unwrap().duration.unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

pub fn get_season(conn: &DbConnection, data: &Media) -> Result<Season, DieselError> {
    use dim_database::schema::season;

    let season = season::table
        .filter(season::tvshowid.eq(data.id))
        .order(season::season_number.asc())
        .first::<Season>(&**conn)?;

    Ok(season)
}

pub fn get_episode(conn: &DbConnection, data: &Season) -> Result<Episode, DieselError> {
    let mut episodes = Episode::get_all_of_season(conn, data)?;

    episodes.sort_by(|a, b| a.episode.cmp(&b.episode));

    Ok(episodes.pop().unwrap())
}

pub fn construct_standard(conn: &DbConnection, data: &Media, quick: Option<bool>) -> JsonValue {
    // TODO: convert to enums

    let duration = get_top_duration(conn, data);
    let season_episode_pair =
        get_season(conn, data).and_then(|x| Ok((x.clone(), get_episode(&conn, &x))));
    let genres = Genre::get_by_media(&conn, data.id)
        .unwrap()
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();

    if quick.unwrap_or(false) {
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
pub fn dashboard(conn: DbConnection, _user: Auth) -> Result<JsonValue, Status> {
    use dim_database::schema::media;

    let mut top_rated = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by((media::id, media::name))
        .order(media::rating.desc())
        .load::<Media>(&*conn)
        .unwrap();

    top_rated.dedup_by(|a, b| a.name.eq(&b.name));

    let top_rated = top_rated
        .iter()
        .map(|x| construct_standard(&conn, x, None))
        .collect::<Vec<JsonValue>>();

    let recently_added = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by((media::id, media::name))
        .order(media::added.desc())
        .load::<Media>(&*conn)
        .unwrap()
        .iter()
        .map(|x| construct_standard(&conn, x, None))
        .collect::<Vec<JsonValue>>();

    if top_rated.len() >= 10 && recently_added.len() >= 10 {
        Ok(json!({
            "TOP RATED": top_rated[0..10].to_vec(),
            "FRESHLY ADDED": recently_added[0..10].to_vec(),
        }))
    } else {
        Ok(json!({}))
    }
}

#[get("/dashboard/banner")]
pub fn banners(conn: DbConnection, _user: Auth) -> Result<Json<Vec<JsonValue>>, Status> {
    use dim_database::schema::media;
    use rand::distributions::{Distribution, Uniform};

    no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

    let sampler = Uniform::new(0, 240);
    let mut rng = rand::thread_rng();
    let results = media::table
        .filter(media::media_type.ne(MediaType::Episode))
        .group_by(media::id)
        .order(RANDOM)
        .limit(3)
        .load::<Media>(&*conn)
        .expect("unable to load banners")
        .iter()
        .filter(|x| x.backdrop_path.is_some())
        .map(|x| {
            let duration = get_top_duration(&conn, &x);
            // let (is_show, (season, episode)) = get_episode(&conn, &x);
            let season_episode_pair =
                get_season(&conn, &x).and_then(|x| Ok((x.clone(), get_episode(&conn, &x))));
            let genres: Vec<String> = Genre::get_by_media(&*conn, x.id)
                .unwrap()
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<_>>();

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

#[get("/filebrowser/<path..>")]
pub fn get_directory_structure(path: PathBuf, _user: Auth) -> Result<Json<Vec<String>>, Status> {
    let mut dirs: Vec<String> = WalkDir::new(format!("/{}", path.to_str().unwrap()))
        .max_depth(1usize)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| {
            !x.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter(|x| !x.path().is_file())
        .map(|x| x.path().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    dirs.sort();

    Ok(Json(dirs))
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
) -> Result<Json<Vec<JsonValue>>, Status> {
    use dim_database::schema::genre_media;
    use dim_database::schema::media;

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
        let genre_row = match Genre::get_by_name(&*conn, x) {
            Ok(x) => x.id,
            Err(_) => return Err(Status::NotFound),
        };

        let new_result = result
            .inner_join(genre_media::table)
            .filter(genre_media::genre_id.eq(genre_row));

        match new_result.load::<Media>(&*conn) {
            Ok(x) => {
                return Ok(Json(
                    x.iter()
                        .map(|x| construct_standard(&conn, x, quick))
                        .collect::<Vec<JsonValue>>(),
                ))
            }
            Err(_) => return Err(Status::NotFound),
        }
    }

    if let Ok(x) = result.load::<Media>(&*conn) {
        return Ok(Json(
            x.iter()
                .map(|x| construct_standard(&conn, x, quick))
                .collect::<Vec<JsonValue>>(),
        ));
    }
    Err(Status::NotFound)
}
