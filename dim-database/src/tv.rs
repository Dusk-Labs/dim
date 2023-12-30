use crate::DatabaseError;
use serde::{Deserialize, Serialize};

/// Struct represents a tv show entry in the database.
/// This is mostly used as a marker to mark shows from movies, and episodes.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TVShow {
    /// id of a media object we marked as a tv show.
    pub id: i64,
}

impl TVShow {
    pub async fn count_children(
        tx: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<i64, DatabaseError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            count: i64,
        }

        Ok(sqlx::query_as::<_, Row>(
            r#"SELECT COUNT(_tblseason.id) as count FROM _tblmedia
            INNER JOIN _tblseason on _tblseason.tvshowid = _tblmedia.id
            WHERE _tblmedia.id = ?"#,
        )
        .bind(id)
        .fetch_one(&mut **tx)
        .await?
        .count)
    }
}
