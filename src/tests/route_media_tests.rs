#[cfg(test)]
mod tests {
    
    
    
    
    

/*
    #[test]
    fn get_media_by_id() {
        run_test!(|client| {
            drop_all_data();
            post_media_test_template(&client, "unittest");

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
                "media_type": "unittest"
            })
            .to_string();

            assert_eq!(resp.status(), Status::Ok);
            assert_eq!(resp.body_string().unwrap(), resp_content);
        });
    }

    #[test]
    fn update_media_by_id() {
        run_test!(|client| {
            post_media_test_template(&client, "unittest");
            let body = json!({
                "name": "test",
                "description": "test",
                "rating": 10,
                "year": 2019,
                "added": "test",
                "poster_path": "test",
                "media_type": "test"
            })
            .to_string();

            // Assume media id is 1
            let resp = client
                .patch("/api/v1/media/1")
                .body(body)
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(resp.status(), Status::NoContent);

            let body = json!({
                "id": 1,
                "library_id": 1,
                "name": "test",
                "description": "test",
                "rating": 10,
                "year": 2019,
                "added": "test",
                "poster_path": "test",
                "media_type": "test"
            })
            .to_string();

            let mut resp = client.get("/api/v1/media/1").dispatch();

            assert_eq!(resp.body_string().unwrap(), body);
        });
    }

    #[test]
    fn create_media_invalid_data() {
        run_test!(|client| {
            drop_all_data();

            let body = json!({
                "library_id": 1000, // Invalid library id
                "added": null,
                "media_type": 120
            })
            .to_string();
            let resp = client
                .post("/api/v1/media")
                .body(body)
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(resp.status(), Status::UnprocessableEntity);
        });
    } */
}
