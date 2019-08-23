use crate::core::DbConnection;
use std::path::PathBuf;
use walkdir::WalkDir;
use diesel::prelude::*;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[get("/dashboard")]
pub fn dashboard(conn: DbConnection) -> Result<JsonValue, Status> {
    use dim_database::schema::media::dsl::*;
    let top_rated = media
        .order(rating.desc())
        .load::<Media>(&*conn)
        .unwrap();

    let recently_added = media.order(added.desc()).load::<Media>(&*conn).unwrap();

    if top_rated.len() >= 10 && recently_added.len() >= 10 {
        Ok(json!({
            "TOP RATED": top_rated[0..10].to_vec(),
            "FRESHLY ADDED": recently_added[0..10].to_vec()
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

            json!({
                "id": x.id,
                "title": x.name,
                "year": x.year,
                "synopsis": x.description,
                "backdrop": x.backdrop_path,
                "duration": duration,
                "genres": x.genres,
                "delta": 0,
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
