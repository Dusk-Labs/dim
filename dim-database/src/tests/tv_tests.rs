use crate::library;
use crate::media;

pub async fn insert_tv(conn: &mut crate::Transaction<'_>) -> i64 {
    let media = media::InsertableMedia {
        library_id: 1,
        name: "TestMedia".into(),
        description: None,
        rating: Some(10.0),
        year: Some(2020),
        added: "Test".into(),
        poster: None,
        backdrop: None,
        media_type: library::MediaType::Tv,
    };

    let id = media.insert(&mut *conn).await.unwrap();
    id
}
