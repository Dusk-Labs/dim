#[cfg(test)]
mod tests {
    use
        crate::tests::{drop_all_data, CLIENT};
        use database::library::Library;
        use rocket::{
            http::{ContentType, Status},
            local::Client,
        };
        use std::collections::HashMap;

    #[test]
    fn get_all_libraries() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();
        let mut resp = client.get("/api/v1/library/").dispatch();
        let body = resp.body_string().unwrap();
        assert_eq!(body, "[]");
        assert_eq!(resp.status(), Status::Ok);
    }

    #[test]
    fn post_new_library() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();
        let body = json!({
            "name": "unittest",
            "location": "/tmp/unittest",
            "media_type": "movie"
        })
        .to_string();
        let resp = client
            .post("/api/v1/library/")
            .body(body)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(resp.status(), Status::Created);

        let mut new_resp = client.get("/api/v1/library/").dispatch();
        let response_body: Vec<Library> =
            serde_json::from_str(&new_resp.body_string().unwrap()).unwrap();

        assert_eq!(response_body[0].id, 1);
        assert_eq!(response_body[0].name, "unittest".to_string());
    }

    #[test]
    fn post_new_library_invalid_inputs() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();

        let body = json!({
            "name": "unittest",
            "location": "/tmp/unittest"
        })
        .to_string();

        let resp = client
            .post("/api/v1/library/")
            .body(body)
            .header(ContentType::JSON)
            .dispatch();
        assert_ne!(resp.status(), Status::Ok);

        let body = json!({
            "name": "unittest",
            "media_type": "unittest"
        })
        .to_string();

        let resp = client
            .post("/api/v1/library/")
            .body(body)
            .header(ContentType::JSON)
            .dispatch();
        assert_ne!(resp.status(), Status::Ok);
    }

    #[test]
    fn delete_library() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();
        // Post some test data
        let body = json!({
            "name": "unittest",
            "location": "/tmp/unittest",
            "media_type": "movie"
        })
        .to_string();
        let resp = client
            .post("/api/v1/library")
            .body(body)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(resp.status(), Status::Created);

        // Delete library assuming id 1 because dbs were dropped
        let resp = client.delete("/api/v1/library/1").dispatch();

        assert_eq!(resp.status(), Status::NoContent);
    }

    #[test]
    fn delete_invalid_library() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();
        // Delete library assuming id 1000 for invalid library
        let resp = client.delete("/api/v1/library/1000").dispatch();
        assert_eq!(resp.status(), Status::NoContent);
    }

    #[test]
    fn get_library_content() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();
        // Post some test data
        let body = json!({
            "name": "unittest",
            "location": "/tmp/unittest",
            "media_type": "movie"
        })
        .to_string();
        let resp = client
            .post("/api/v1/library")
            .body(body)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(resp.status(), Status::Created);

        let mut resp = client
            .get("/api/v1/library/1/media") // assume id of 1 for library
            .dispatch();

        let body_string = resp.body_string().unwrap();

        assert_eq!(resp.status(), Status::Ok);

        let response: Result<HashMap<&str, Vec<&str>>, _> =
            serde_json::from_str(body_string.as_str());

        let expected = {
            let mut m = HashMap::new();
            m.insert("Unmatched Media", Vec::new());
            m.insert("unittest", Vec::new());
            m
        };
        assert_eq!(response.unwrap(), expected);
    }
}
