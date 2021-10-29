use crate::media::MediaTrait;
use crate::DatabaseError;

/// Struct reperesents a insertable movie entry
#[derive(Clone, Copy)]
pub struct InsertableMovie;

impl InsertableMovie {
    /// Method inserts the object into the table movie returning its id which should be equivalent
    /// to the field id.
    ///
    /// # Arguments
    /// * `&` - diesel &ection reference to postgres
    /// * `id` - id of the media that should be a movie
    pub async fn insert(conn: &mut crate::Transaction<'_>, id: i64) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!("INSERT INTO movie (id) VALUES ($1)", id)
            .execute(&mut *conn)
            .await?
            .last_insert_rowid())
    }
}

impl MediaTrait for InsertableMovie {}
