use crate::core::DbConnection;
use diesel::prelude::*;
use dim_database::media::Media;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[get("/dashboard")]
pub fn dashboard(conn: DbConnection) -> Result<JsonValue, Status> {
    use dim_database::schema::media::dsl::*;
    let top_rated = media.order(rating.desc()).load::<Media>(&*conn).unwrap();

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
