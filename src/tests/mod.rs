#[allow(unused_imports)]
use rocket::local::Client;

#[cfg(test)]
pub fn drop_all_data() {
    use crate::core::rocket;
    use crate::core::DbConnection;
    use crate::schema::episode::dsl::*;
    use crate::schema::library::dsl::*;
    use crate::schema::media::dsl::*;
    use crate::schema::season::dsl::*;
    use crate::schema::tv_show::dsl::*;
    use diesel::prelude::*;

    let conn = DbConnection::get_one(&rocket()).expect("Failed to get db");

    diesel::delete(library).execute(&*conn).unwrap();
    diesel::delete(media).execute(&*conn).unwrap();
    diesel::delete(tv_show).execute(&*conn).unwrap();
    diesel::delete(season).execute(&*conn).unwrap();
    diesel::delete(episode).execute(&*conn).unwrap();
}

#[cfg(test)]
pub fn post_media_test_template(client: &Client, media_type: &str) {
    use rocket::http::ContentType;
    use rocket::http::Status;

    drop_all_data();
    let body = json!({
        "name": "unittest",
        "location": "/tmp/unittest",
        "media_type": media_type
    })
    .to_string();
    let resp = client
        .post("/api/v1/library")
        .body(body)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(resp.status(), Status::Created);

    let body = json!({
        "library_id": 1, // Assume libid to be 1
        "name": "unittest",
        "added": "unittest",
        "media_type": media_type
    })
    .to_string();
    let resp = client
        .post("/api/v1/media")
        .body(body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    // Assume media id is 1
    let mut resp = client.get("/api/v1/media/1").dispatch();

    let resp_content = json!({
        "id": 1,
        "library_id": 1,
        "name": "unittest",
        "description": null,
        "rating": null,
        "year": null,
        "added": "unittest",
        "poster_path": null,
        "media_type": media_type
    })
    .to_string();

    assert_eq!(resp.status(), Status::Ok);
    assert_eq!(resp.body_string().unwrap(), resp_content);
}

#[cfg(test)]
pub fn post_season_test_template(client: &Client) {
    use rocket::http::ContentType;
    use rocket::http::Status;

    post_media_test_template(client, "tv");

    let body = json!({
        "season_number": 2,
        "tvshowid": 1,
        "added": "12341212",
        "poster": "/tmp/path.jpg"
    })
    .to_string();

    let resp = client
        .post("/api/v1/tv/1/season")
        .body(body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);
}

#[cfg(test)]
pub fn post_episode_test_template(client: &Client) {
    use rocket::http::ContentType;
    use rocket::http::Status;

    post_season_test_template(client);

    let body = json!({
        "seasonid": 1,
        "episode": 1,
        "library_id": 1,
        "name": "episode1",
        "description": null,
        "rating": null,
        "year": null,
        "added": "unittest",
        "poster_path": null,
        "media_type": "episode"
    })
    .to_string();

    let resp = client
        .post("/api/v1/tv/1/season/2/episode")
        .body(body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);
}

pub mod route_library_tests;
pub mod route_media_tests;
pub mod route_tv_tests;
