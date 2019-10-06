use crate::media::MediaTrait;
use crate::schema::movie;
use crate::streamablemedia::{StreamableMedia, StreamableTrait};
use diesel::prelude::*;

#[derive(Clone, Identifiable, Queryable, Associations)]
#[belongs_to(StreamableMedia, foreign_key = "id")]
#[table_name = "movie"]
pub struct Movie {
    id: i32,
}

#[derive(Clone, Insertable)]
#[table_name = "movie"]
pub struct InsertableMovie {
    id: i32,
}

impl StreamableTrait for InsertableMovie {
    fn new(id: i32) -> Self {
        Self { id }
    }

    fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        diesel::insert_into(movie::table)
            .values(self)
            .returning(movie::id)
            .get_result(conn)
    }
}

impl MediaTrait for InsertableMovie {}
