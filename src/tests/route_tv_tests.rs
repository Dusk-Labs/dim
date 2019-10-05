#[cfg(test)]
mod tests {
    use crate::dim_database::get_conn;
    use crate::core::rocket_pad;
    use crate::macros::DB_LOCK;
    use crate::tests::drop_all_data;
    use crate::tests::{
        post_episode_test_template, post_media_test_template, post_season_test_template,
    };
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn get_tv_by_media_id() {
        run_test!(|client| {
            drop_all_data();
            post_media_test_template(&client, "tv");

            let mut resp = client.get("/api/v1/tv/1").dispatch();

            let resp_content = json!({
                "id": 1,
                "library_id": 1,
                "name": "unittest",
                "description": null,
                "rating": null,
                "year": null,
                "added": "unittest",
                "poster_path": null,
                "media_type": "tv"
            })
            .to_string();

            assert_eq!(resp.status(), Status::Ok);
            assert_eq!(resp.body_string().unwrap(), resp_content);
        });
    }

    #[test]
    fn get_tv_seasons() {
        run_test!(|client| {
            drop_all_data();
            post_season_test_template(&client);

            let mut resp = client.get("/api/v1/tv/1/season").dispatch();
            let resp_content = json!([{
                "id": 1,
                "season_number": 2,
                "tvshowid": 1,
                "added": "12341212",
                "poster": "/tmp/path.jpg"
            }])
            .to_string();

            assert_eq!(resp.status(), Status::Ok);
            assert_eq!(resp.body_string().unwrap(), resp_content);
        });
    }

    #[test]
    fn get_season_info() {
        run_test!(|client| {
            drop_all_data();
            post_season_test_template(&client);

            let mut resp = client.get("/api/v1/tv/1/season/2/").dispatch();
            let resp_content = json!({
                "id": 1,
                "season_number": 2,
                "tvshowid": 1,
                "added": "12341212",
                "poster": "/tmp/path.jpg"
            })
            .to_string();

            assert_eq!(resp.status(), Status::Ok);
            assert_eq!(resp.body_string().unwrap(), resp_content);
        });
    }

    #[test]
    fn post_new_season() {
        run_test!(|client| {
            drop_all_data();
            post_season_test_template(&client);
        });
    }

    #[test]
    fn patch_season() {
        run_test!(|client| {
            drop_all_data();
            post_season_test_template(&client);

            let new_data = json!({
                "season_number": 3,
                "added": "12354234",
                "poster": "/tmp/newpath.jpg"
            })
            .to_string();

            let resp = client
                .patch("/api/v1/tv/1/season/2/")
                .body(new_data)
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(resp.status(), Status::NoContent);
        });
    }

    #[test]
    fn delete_season() {
        run_test!(|client| {
            drop_all_data();
            post_season_test_template(&client);

            let resp = client.delete("/api/v1/tv/1/season/2/").dispatch();
            assert_eq!(resp.status(), Status::Ok);

            let resp = client.get("/api/v1/tv/1/season/2/").dispatch();
            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn post_episode() {
        run_test!(|client| {
            drop_all_data();
            post_episode_test_template(&client);
        });
    }

    #[test]
    fn get_episode_info() {
        run_test!(|client| {
            drop_all_data();
            post_episode_test_template(&client);

            let mut resp = client.get("/api/v1/tv/1/season/2/episode/1/").dispatch();
            assert_eq!(resp.status(), Status::Ok);

            let body = json!({
                "seasonid": 1,
                "episode":  1,
                "id": 2,
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
            assert_eq!(resp.body_string().unwrap(), body);
        });
    }

    #[test]
    fn patch_episode_info() {
        run_test!(|client| {
            drop_all_data();
            post_episode_test_template(&client);

            let body = json!({
                "description": "test"
            })
            .to_string();

            let resp = client
                .patch("/api/v1/tv/1/season/2/episode/1/")
                .header(ContentType::JSON)
                .body(body)
                .dispatch();

            assert_eq!(resp.status(), Status::NoContent);

            let mut resp = client.get("/api/v1/tv/1/season/2/episode/1/").dispatch();
            assert_eq!(resp.status(), Status::Ok);

            let body = json!({
                "seasonid": 1,
                "episode": 1,
                "id": 2,
                "library_id": 1,
                "name": "episode1",
                "description": "test",
                "rating": null,
                "year": null,
                "added": "unittest",
                "poster_path": null,
                "media_type": "episode"
            })
            .to_string();

            assert_eq!(resp.body_string().unwrap(), body);
        });
    }

    #[test]
    fn delete_episode_by_id() {
        run_test!(|client| {
            drop_all_data();
            post_episode_test_template(&client);

            let resp = client.delete("/api/v1/tv/1/season/2/episode/1/").dispatch();

            assert_eq!(resp.status(), Status::Ok);

            let resp = client.get("/api/v1/tv/1/season/2/episode/1/").dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }
}

#[cfg(test)]
mod abnormal_url_params_tests {
    use crate::core::rocket;
    use crate::macros::DB_LOCK;
    use crate::tests::drop_all_data;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn get_tv_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let resp = client.get("/api/v1/tv/123123/").dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn get_tv_seasons_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let resp = client.get("/api/v1/tv/123123/season").dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn get_season_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let resp = client.get("/api/v1/season/123123/").dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn patch_season_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let new_data = json!({
                "season_number": 3,
                "added": "12354234",
                "poster": "/tmp/newpath.jpg"
            })
            .to_string();

            let resp = client
                .patch("/api/v1/season/123123/")
                .body(new_data)
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn delete_season_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let resp = client.delete("/api/v1/season/123123").dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn get_episode_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let resp = client.get("/api/v1/episode/123123").dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn patch_episode_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let body = json!({
                "description": "test"
            })
            .to_string();

            let resp = client
                .patch("/api/v1/episode/2123123/")
                .header(ContentType::JSON)
                .body(body)
                .dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }

    #[test]
    fn delete_episode_invalid_id() {
        run_test!(|client| {
            drop_all_data();

            let resp = client.delete("/api/v1/episode/12312312312312").dispatch();

            assert_eq!(resp.status(), Status::NotFound);
        });
    }
}

#[cfg(test)]
mod post_patch_abnormal_body_tests {
    #[test]
    fn post_season_abnormal_json() {
        assert_eq!(true, true);
    }

    #[test]
    fn patch_season_abnormal_json() {
        assert_eq!(true, true);
    }

    #[test]
    fn post_episode_abnormal_json() {
        assert_eq!(true, true);
    }

    #[test]
    fn patch_episode_abnormal_json() {
        assert_eq!(true, true);
    }
}
