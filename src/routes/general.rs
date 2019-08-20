use crate::core::DbConnection;
use dim_database::library::{InsertableLibrary, Library};
use dim_database::media::Media;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use diesel::prelude::*;

#[get("/dashboard")]
pub fn dashboard(
    conn: DbConnection,
) -> Result<JsonValue, Status> {
    use dim_database::schema::media::dsl::*;
    let top_rated = media
        .order(rating.desc())
        .load::<Media>(&*conn).unwrap();
    
    let recently_added = media
        .order(added.desc())
        .load::<Media>(&*conn).unwrap();

    let body = json!({
        "TOP RATED": top_rated[0..10].to_vec(),
        "FRESHLY ADDED": recently_added[0..10].to_vec()
    });
    Ok(body)
}

#[get("/dashboard/banner")]
pub fn banners(
    conn: DbConnection,
) -> Result<Json<Vec<JsonValue>>, Status> {
    use dim_database::schema::media::dsl::*;
    no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");
    let results = media
        .order(RANDOM)
        .limit(3)
        .load::<Media>(&*conn)
        .expect("unable to load banners")
        .iter()
        .map(|x| {
            json!({
                "id": x.id,
                "title": x.name,
                "synopsis": x.description,
                "backdrop": x.backdrop_path,
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(results))
}
