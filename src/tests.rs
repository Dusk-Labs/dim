extern crate parking_lot;
extern crate diesel;

mod library_tests {
    use crate::core::rocket;
    use crate::schema;
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;
    use super::parking_lot::Mutex;
    use serde_json::Result;

    static DB_LOCK: Mutex<()> = Mutex::new(());

    macro_rules! run_test {
    (|$client:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();
        let rocket = rocket();
        let $client = Client::new(rocket).expect("Rocket client");

        $block
    })
    }
    
    fn drop_all_data(){
        use crate::schema::library::dsl::*;
        use crate::schema::media::dsl::*;
        use diesel::prelude::*;
        use crate::core::DbConnection;

        let conn = DbConnection::get_one(&rocket()).expect("Failed to get db");
        diesel::delete(library).execute(&*conn).unwrap();
        diesel::delete(media).execute(&*conn).unwrap();
    }

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
            }).to_string();
            let resp = client.post("/api/v1/library/")
                .body(body)
                .header(ContentType::JSON)
                .dispatch();
            assert_eq!(resp.status(), Status::Created);

            let mut new_resp = client.get("/api/v1/library/").dispatch();
            assert_eq!(new_resp.body_string().unwrap(), "[{\"id\":1,\"name\":\"unittest\",\"location\":\"/tmp/unittest\",\"media_type\":\"unittest\"}]");
        });
    }

    #[test]
    fn post_new_library_invalid_inputs() {
        run_test!(|client| {
            drop_all_data();
            let body = json!({
                "name": "unittest",
                "location": "/tmp/unittest"
            }).to_string();
            let resp = client.post("/api/v1/library/")
                .body(body)
                .header(ContentType::JSON)
                .dispatch();
            assert_ne!(resp.status(), Status::Ok);

            let body = json!({
                "name": "unittest",
                "media_type": "unittest"
            }).to_string();
            let resp = client.post("/api/v1/library/")
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
            }).to_string();
            let resp = client.post("/api/v1/library")
                .body(body)
                .header(ContentType::JSON)
                .dispatch();
            assert_eq!(resp.status(), Status::Created);

            // Delete library assuming id 1 because dbs were dropped
            let resp = client.delete("/api/v1/library/1")
                .dispatch();

            assert_eq!(resp.status(), Status::NoContent);
        });
    }

    #[test]
    fn delete_invalid_library() {
        run_test!(|client| {
            drop_all_data();
            // Delete library assuming id 1000 for invalid library
            let resp = client.delete("/api/v1/library/1000")
                .dispatch();
            assert_eq!(resp.status(), Status::NoContent);
        });
    }

    fn post_media_test_template(client: &Client) {
        drop_all_data();
        let body = json!({
            "name": "unittest",
            "location": "/tmp/unittest",
            "media_type": "unittest"
        }).to_string();
        let resp = client.post("/api/v1/library")
            .body(body)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(resp.status(), Status::Created);

        let body = json!({
            "library_id": 1, // Assume libid to be 1
            "name": "unittest",
            "added": "unittest",
            "media_type": "unittest"
        }).to_string();
        let resp = client.post("/api/v1/media")
            .body(body)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(resp.status(), Status::Created);

        // Assume media id is 1
        let mut resp = client.get("/api/v1/media/1")
            .dispatch();

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
        }).to_string();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.body_string().unwrap(), resp_content);
    }

    #[test]
    fn post_media() {
        run_test!(|client| {
            post_media_test_template(&client);
        });
    }

    #[test]
    fn get_library_content() {
        run_test!(|client| {
            // Create a temporary library with test data
            post_media_test_template(&client);
            let mut resp = client.get("/api/v1/library/1") // assume id of 1 for library
                .dispatch();
            
            let resp_content = json!([{
                "id": 1,
                "library_id": 1,
                "name": "unittest",
                "description": null,
                "rating": null,
                "year": null,
                "added": "unittest",
                "poster_path": null,
                "media_type": "unittest"
            }]).to_string();
            assert_eq!(resp.status(), Status::Ok);
            assert_eq!(resp.body_string().unwrap(), resp_content);
        });
    }
}
