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
    pub async fn get_all(conn: &mut crate::Transaction<'_>) -> Result<Vec<Media>, DatabaseError> {
        Ok(sqlx::query_as!(
            Media,
            r#"SELECT 
                media.id, media.library_id, media.name, media.description,
                media.rating, media.year, media.added, media.poster_path, 
                media.backdrop_path, media.media_type as "media_type: _" 
                FROM media INNER JOIN tv_show ON media.id = tv_show.id"#
        )
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .collect())
    }

    /// Upgrades a TV Show object into a Media object
    pub async fn upgrade(self, conn: &mut crate::Transaction<'_>) -> Result<Media, DatabaseError> {
        let media = sqlx::query_as!(
            Media,
            r#"SELECT 
                media.id, media.library_id, media.name, media.description,
                media.rating, media.year, media.added, media.poster_path, 
                media.backdrop_path, media.media_type as "media_type: _"
                FROM media 
                INNER JOIN tv_show ON tv_show.id = media.id
                WHERE tv_show.id = ?"#,
            self.id
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(media)
    }

    /// Returns total duration of the files on disk for a tv show.
    pub async fn get_total_duration(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<i64, DatabaseError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            total: i64,
        }

        // FIXME: The sqlx proc macro crashes on this query with the message: "no entry found for
        // key"
        Ok(sqlx::query_as::<_, Row>(
            r#"SELECT COALESCE(SUM(mediafile.duration), 0) as "total: i64"
            FROM tv_show
            INNER JOIN season on season.tvshowid = tv_show.id
            INNER JOIN episode on episode.seasonid = season.id
            INNER JOIN mediafile on mediafile.media_id = episode.id
            WHERE tv_show.id = ?
            GROUP BY episode.id
            "#,
        )
        .bind(id)
        .fetch_one(&mut *conn)
        .await?
        .total)
    }

    /// Returns total number of episodes for a tv show.
    pub async fn get_total_episodes(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<i64, DatabaseError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            total: i64,
        }

        // FIXME: See `get_total_duration`
        Ok(sqlx::query_as::<_, Row>(
            r#"SELECT COALESCE(COUNT(episode.id), 0) as "total: i64" FROM tv_show
            INNER JOIN season on season.tvshowid = tv_show.id
            INNER JOIN episode on episode.seasonid = season.id
            WHERE tv_show.id = ?"#,
        )
        .bind(id)
        .fetch_one(&mut *conn)
        .await?
        .total)
    }

    /// Method inserts a new tv show in the database.
    ///
    /// # Arguments
    /// * `id` - id of a media object that should be a tv show.
    pub async fn insert(conn: &mut crate::Transaction<'_>, id: i64) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!("INSERT INTO tv_show (id) VALUES ($1)", id)
            .execute(&mut *conn)
            .await?
            .last_insert_rowid())
    }
}
