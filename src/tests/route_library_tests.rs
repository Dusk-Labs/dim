#[cfg(test)]
mod tests {
    use crate::core::rocket_pad;
    use crate::macros::DB_LOCK;
    use crate::tests::drop_all_data;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn get_all_libraries() {
        run_test!(|client| {
            drop_all_data();
            let mut resp = client.get("/api/v1/library/").dispatch();
            let body = resp.body_string().unwrap();
            assert_eq!(body, "[]");
            assert_eq!(resp.status(), Status::Ok);
        });
    }

    #[test]
    fn post_new_library() {
        run_test!(|client| {
            drop_all_data();
            let body = json!({
                "name": "unittest",
                "location": "/tmp/unittest",
                "media_type": "unittest"
            })
            .to_string();
            let resp = client
                .post("/api/v1/library/")
                .body(body)
                .header(ContentType::JSON)
                .dispatch();
            assert_eq!(resp.status(), Status::Created);

            let mut new_resp = client.get("/api/v1/library/").dispatch();
            let body = json!([{
                "id": 1,
                "name": "unittest",
                "location": "/tmp/unittest",
                "media_type": "unittest"
            }])
            .to_string();
            assert_eq!(new_resp.body_string().unwrap(), body);
        });
    }

    #[test]
    fn post_new_library_invalid_inputs() {
        run_test!(|client| {
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
        });
    }

    #[test]
    fn delete_library() {
        run_test!(|client| {
            drop_all_data();
            // Post some test data
            let body = json!({
                "name": "unittest",
                "location": "/tmp/unittest",
                "media_type": "unittest"
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
        });
    }

    #[test]
    fn delete_invalid_library() {
        run_test!(|client| {
            drop_all_data();
            // Delete library assuming id 1000 for invalid library
            let resp = client.delete("/api/v1/library/1000").dispatch();
            assert_eq!(resp.status(), Status::NoContent);
        });
    }

    #[test]
    fn get_library_content() {
        run_test!(|client| {
            drop_all_data();
            // Post some test data
            let body = json!({
                "name": "unittest",
                "location": "/tmp/unittest",
                "media_type": "unittest"
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

            let resp_content = json!({
                "unittest": []
            })
            .to_string();
            assert_eq!(resp.status(), Status::Ok);
            assert_eq!(resp.body_string().unwrap(), resp_content);
        });
    }
}
