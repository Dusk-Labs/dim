use crate::DatabaseError;

#[derive(Clone, Copy)]
pub struct Movie;

impl Movie {
    /// Method will return the number of mediafiles linked against this media object.
    pub async fn count_children(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            "SELECT COUNT(*) AS count FROM mediafile WHERE media_id = ?",
            id
        )
        .fetch_one(&mut **conn)
        .await?
        .count as _)
    }
}
