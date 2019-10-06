use crate::media::Media;
use crate::schema::streamable_media;
use diesel::prelude::*;

pub trait StreamableTrait {
    fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error>;
    fn new(id: i32) -> Self;
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug, Associations)]
#[belongs_to(Media, foreign_key = "id")]
#[table_name = "streamable_media"]
pub struct StreamableMedia {
    pub id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "streamable_media"]
pub struct InsertableStreamableMedia {
    pub id: i32,
}

impl InsertableStreamableMedia {
    pub(crate) fn insert(id: i32, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        diesel::insert_into(streamable_media::table)
            .values(InsertableStreamableMedia {id})
            .returning(streamable_media::id)
            .get_result(conn)
    }
}
