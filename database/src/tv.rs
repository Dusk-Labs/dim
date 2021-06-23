use crate::media::*;
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
    /// Method returns all the tv shows in the database.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    pub async fn get_all(conn: &crate::DbConnection) -> Result<Vec<Media>, DatabaseError> {
        Ok(sqlx::query_as!(
            Media,
            r#"SELECT 
                media.id, media.library_id, media.name, media.description,
                media.rating, media.year, media.added, media.poster_path, 
                media.backdrop_path, media.media_type as "media_type: _" 
                FROM media INNER JOIN tv_show ON media.id = tv_show.id"#
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .collect())
    }

    /// Upgrades a TV Show object into a Media object
    pub async fn upgrade(self, conn: &crate::DbConnection) -> Result<Media, DatabaseError> {
        let media = sqlx::query_as!(
            Media,
            r#"SELECT 
                media.id, media.library_id, media.name, media.description,
                media.rating, media.year, media.added, media.poster_path, 
                media.backdrop_path, media.media_type as "media_type: _"
                FROM media WHERE media.id = ?"#,
            self.id
        )
        .fetch_one(conn)
        .await?;

        Ok(media)
    }

    /// Method inserts a new tv show in the database.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of a media object that should be a tv show.
    pub async fn insert(conn: &crate::DbConnection, id: i64) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!("INSERT INTO tv_show (id) VALUES ($1)", id)
            .execute(conn)
            .await?
            .last_insert_rowid())
    }
}
