use crate::media::MediaTrait;
use crate::schema::movie;
use crate::streamable_media::{StreamableMedia, StreamableTrait};
use diesel::prelude::*;

/// Struct represents a Movie entry in the database
#[derive(Clone, Identifiable, Queryable, Associations)]
#[belongs_to(StreamableMedia, foreign_key = "id")]
#[table_name = "movie"]
pub struct Movie {
    /// id of the movie that is also a foreign key to a media entry.
    id: i32,
}

/// Struct reperesents a insertable movie entry
#[derive(Clone, Insertable)]
#[table_name = "movie"]
pub struct InsertableMovie {
    /// id of a media entry that should be used as a foreign key.
    id: i32,
}

impl StreamableTrait for InsertableMovie {
    /// Method returns a new instance of InsertableMovie, this is a trait method because it is used
    /// to indicate that this specific media entry can be streamed.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of the movie we are inserting, this id should already exist in the media table.
    fn new(id: i32) -> Self {
        Self { id }
    }

    /// Method inserts the object into the table movie returning its id which should be equivalent
    /// to the field id.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    fn insert(&self, conn: &crate::DbConnection) -> Result<i32, diesel::result::Error> {
        diesel::insert_into(movie::table)
            .values(self)
            .execute(conn)?;

        Ok(self.id)
    }
}

impl MediaTrait for InsertableMovie {}
