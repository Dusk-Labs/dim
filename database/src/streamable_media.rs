use crate::media::Media;
use crate::schema::streamable_media;
use crate::DatabaseError;

use async_trait::async_trait;
use tokio_diesel::*;

/// Trait used to tell between a static media, ie. tv show and a streamable media such as a movie
/// or episode.
#[async_trait]
pub trait StreamableTrait {
    /// Required method that inserts Self into the database returning the id of it or a error.
    async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError>;
    /// Method should return a instance of Self.
    fn new(id: i32) -> Self;
}

/// Struct represents a streamable media located in the database.
/// It is more of a marker struct rather than something functionally required.
#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug, Associations)]
#[belongs_to(Media, foreign_key = "id")]
#[table_name = "streamable_media"]
pub struct StreamableMedia {
    /// id which should be the id a media entry we'd like to mark as streamable.
    pub id: i32,
}

/// Struct used to create and insert a new streamable_media.
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "streamable_media"]
pub struct InsertableStreamableMedia {
    /// id which should be the id a media entry we'd like to mark as streamable.
    pub id: i32,
}

impl InsertableStreamableMedia {
    /// Method inserts and marks a media object as streamable.
    /// This method is only accessible within this crate and will only ever be called internally.
    ///
    /// # Arguments
    /// * `id` - id of a media entry we'd like to mark as streamable.
    /// * `conn` - diesel connection reference to postgres
    pub(crate) async fn insert(id: i32, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        diesel::insert_into(streamable_media::table)
            .values(InsertableStreamableMedia { id })
            .execute_async(conn)
            .await?;

        Ok(id)
    }
}
