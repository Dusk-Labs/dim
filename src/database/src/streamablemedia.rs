use crate::media::Media;
use crate::schema::streamable_media;
use diesel::prelude::*;

/// Trait used to tell between a static media, ie. tv show and a streamable media such as a movie
/// or episode.
pub trait StreamableTrait {
    /// Required method that inserts Self into the database returning the id of it or a error.
    fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error>;
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
    pub(crate) fn insert(
        id: i32,
        conn: &diesel::PgConnection,
    ) -> Result<i32, diesel::result::Error> {
        diesel::insert_into(streamable_media::table)
            .values(InsertableStreamableMedia { id })
            .returning(streamable_media::id)
            .get_result(conn)
    }
}
