#[cfg(test)]
mod tests {
    use crate::tests::{drop_all_data, put_garbage, CLIENT};
    use database::{library::MediaType, media::Media};
    use rocket::{
        http::{ContentType, Status},
        local::Client,
    };
    use std::collections::HashMap;

    #[test]
    fn get_media_by_id() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();
        put_garbage();

        let mut resp = client.get("/api/v1/media/1").dispatch();
        assert_eq!(resp.status(), Status::Ok);

        let resp: Media = serde_json::from_str(&resp.body_string().unwrap()).unwrap();

        assert_eq!(resp.id, 1);
        assert_eq!(resp.library_id, 1);
        assert_eq!(resp.name, "unittest".to_string());
        assert_eq!(resp.added, Some("unittest".to_string()));

        assert!(resp.description.is_none());
        assert!(resp.rating.is_none());
        assert!(resp.year.is_none());
        assert!(resp.poster_path.is_none());
        assert!(resp.media_type.is_none());
    }

    #[test]
    fn get_extra_info() {
        let client = CLIENT.lock().unwrap();
        drop_all_data();
        put_garbage();

        let resp = client.get("/api/v1/media/1").dispatch();

        assert_eq!(resp.status(), Status::Ok);
    }
}
