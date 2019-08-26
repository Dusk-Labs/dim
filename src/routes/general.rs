use crate::core::DbConnection;
use std::path::PathBuf;
use walkdir::WalkDir;
use diesel::prelude::*;
use dim_database::media::Media;
use dim_database::media::MEDIA_ALL_COLUMNS;
use dim_database::mediafile::MediaFile;
use dim_database::genre::*;
use rocket::http::Status;
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use rand::Rng;

fn construct_standard(conn: &DbConnection, data: &Media) -> JsonValue {
    let duration = match MediaFile::get_of_media(&conn, &data) {
        Ok(x) => x.duration.unwrap(),
        Err(_) => 0,
    };

    let genres = Genre::get_by_media(&conn, data.id)
        .unwrap()
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();

    json!({
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
    })
}

#[get("/dashboard")]
pub fn dashboard(conn: DbConnection) -> Result<JsonValue, Status> {
    use dim_database::schema::media::dsl::*;

    let top_rated = media
        .select(MEDIA_ALL_COLUMNS)
        .order(rating.desc())
        .load::<Media>(&*conn)
        .unwrap()
        .iter()
        .map(|x| construct_standard(&conn, x))
        .collect::<Vec<JsonValue>>();

    let recently_added = media
        .select(MEDIA_ALL_COLUMNS)
        .order(added.desc())
        .load::<Media>(&*conn)
        .unwrap()
        .iter()
        .map(|x| construct_standard(&conn, x))
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
pub fn banners(conn: DbConnection) -> Result<Json<Vec<JsonValue>>, Status> {
    use dim_database::schema::media::dsl::*;
    no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");
    let results = media
        .select(MEDIA_ALL_COLUMNS)
        .order(RANDOM)
        .limit(3)
        .load::<Media>(&*conn)
        .expect("unable to load banners")
        .iter()
        .filter(|x| x.backdrop_path.is_some())
        .map(|x| {
            let duration = match MediaFile::get_of_media(&*conn, &x) {
                Ok(x) => x.duration.unwrap(),
                Err(_) => 0,
            };
            
            let genres: Vec<String> = Genre::get_by_media(&*conn, x.id)
                .unwrap()
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<_>>();

            json!({
                "id": x.id,
                "title": x.name,
                "year": x.year,
                "synopsis": x.description,
                "backdrop": x.backdrop_path,
                "duration": duration,
                "genres": genres,
                "delta": rand::thread_rng().gen_range(240, duration),
                "banner_caption": "WATCH SOMETHING FRESH"
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(results))
}

#[get("/filebrowser/<path..>")]
pub fn get_directory_structure(path: PathBuf) -> Result<Json<Vec<String>>, Status> {
    let dirs: Vec<String> = WalkDir::new(format!("/{}", path.to_str().unwrap()))
        .max_depth(1usize)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| !x.file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false))
        .filter(|x| !x.path().is_file())
        .map(|x| x.path().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    Ok(Json(dirs))
}

#[get("/search?<query>&<year>&<library_id>&<genre>")]
pub fn search(
    conn: DbConnection,
    query: Option<&RawStr>,
    year: Option<i32>,
    library_id: Option<i32>,
    genre: Option<String>
) -> Result<Json<Vec<JsonValue>>, Status> {
    use dim_database::schema::media;
    use crate::dim_database::schema::genre_media;
    use diesel::sql_types::Text;
	use diesel::dsl::sql;
    use diesel_full_text_search::*;
    
    let mut result = media::table
        .select(MEDIA_ALL_COLUMNS)
        .into_boxed();

    if let Some(query_string) = query {
        let query_string = query_string
            .url_decode_lossy()
            .split(" ")
            .collect::<Vec<&str>>()
            .as_slice()
            .join(" & ");

        let q = sql::<TsQuery>("plainto_tsquery(")
            .bind::<Text, _>(query_string)
            .sql(")");
        result = result.filter(q.clone().matches(media::name_search_index));
    }

    if let Some(x) = year {
        result = result.filter(media::year.eq(x));
    }

    if let Some(x) = library_id {
        result = result.filter(media::library_id.eq(x));
    }

    if let Some(x) = genre {
        let genre_row = Genre::get_by_name(&*conn, x).unwrap().id;
        let new_result = result.inner_join(genre_media::table)
            .filter(genre_media::genre_id.eq(genre_row));

        match new_result.load::<Media>(&*conn) {
            Ok(x) => return Ok(Json(x
                                    .iter()
                                    .map(|x| construct_standard(&conn, x))
                                    .collect::<Vec<JsonValue>>())),
            Err(_) => return Err(Status::NotFound),
        }
    }
    
    match result.load::<Media>(&*conn) {
        Ok(x) => Ok(Json(x
                         .iter()
                         .map(|x| construct_standard(&conn, x))
                         .collect::<Vec<JsonValue>>())),
        Err(_) => Err(Status::NotFound),
    }
}
