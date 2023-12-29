use crate::DatabaseError;

use serde::Deserialize;
use serde::Serialize;

/// Struct shows a single genre entry
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Genre {
    pub id: i64,
    /// Genre name, ie "Action"
    pub name: String,
}

/// Intermediary table showing the relationship between a media and a genre
#[derive(Clone, Debug, PartialEq)]
pub struct GenreMedia {
    pub id: i64,
    pub genre_id: i64,
    pub media_id: i64,
}

impl Genre {
    /// Method returns the entry of a genre if exists based on its name.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `query` - genre name
    pub async fn get_by_name(
        conn: &mut crate::Transaction<'_>,
        query: String,
    ) -> Result<Self, DatabaseError> {
        let query = query.to_uppercase();
        Ok(sqlx::query_as!(
            Genre,
            "SELECT * FROM genre WHERE UPPER(genre.name) LIKE ?",
            query
        )
        .fetch_one(&mut **conn)
        .await?)
    }

    /// Method returns all of the episodes belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `media` - reference to a media object which should be a tv show.
    pub async fn get_by_media(
        conn: &mut crate::Transaction<'_>,
        media_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            Genre,
            r#"SELECT genre.id as "id!", genre.name FROM genre
                INNER JOIN genre_media ON genre_media.genre_id = genre.id
                WHERE genre_media.media_id = ?"#,
            media_id
        )
        .fetch_all(&mut **conn)
        .await?)
    }

    /// Method returns a genre based on genre_id and media_id
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `genre_id` - id of a genre
    /// * `media_id` - id of a media object
    pub async fn get_by_id(
        conn: &mut crate::Transaction<'_>,
        genre_id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
            Genre,
            "SELECT * FROM genre
            WHERE id = ?",
            genre_id
        )
        .fetch_one(&mut **conn)
        .await?)
    }

    /// Method removes a genre from the genre table based on its id
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `id` - genre id
    pub async fn delete(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!("DELETE FROM genre WHERE id = ?", id)
            .execute(&mut **conn)
            .await?
            .rows_affected() as usize)
    }

    /// Decouple media from all genres passed in
    pub async fn decouple_all(
        conn: &mut crate::Transaction<'_>,
        media_id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!(
            "DELETE FROM genre_media WHERE genre_media.media_id = ?",
            media_id
        )
        .execute(&mut **conn)
        .await?
        .rows_affected() as usize)
    }
}

/// Genre entry that can be inserted into the db.
#[derive(Clone)]
pub struct InsertableGenre {
    /// Genre name
    pub name: String,
}

impl InsertableGenre {
    /// Method inserts a new genre into the table otherwise returns the id of a existing entry
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    pub async fn insert(&self, conn: &mut crate::Transaction<'_>) -> Result<i64, DatabaseError> {
        let name = self.name.clone().to_uppercase();

        if let Some(record) = sqlx::query!(
            "SELECT id FROM genre
            WHERE UPPER(genre.name) LIKE ?",
            name
        )
        .fetch_optional(&mut **conn)
        .await?
        {
            return Ok(record.id);
        }

        let id = sqlx::query!(r#"INSERT INTO genre (name) VALUES ($1)"#, self.name)
            .execute(&mut **conn)
            .await?
            .last_insert_rowid();

        Ok(id)
    }
}

/// Struct which is used to pair a genre to a media
#[derive(Clone)]
pub struct InsertableGenreMedia {
    pub genre_id: i64,
    pub media_id: i64,
}

impl InsertableGenreMedia {
    /// Method inserts a new entry into the intermediary genre table linking a genre to a media
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    pub async fn insert(&self, conn: &mut crate::Transaction<'_>) {
        let _ = sqlx::query!(
            "INSERT INTO genre_media (genre_id, media_id) VALUES ($1, $2)",
            self.genre_id,
            self.media_id
        )
        .execute(&mut **conn)
        .await;
    }

    /// Method inserts a pair into the genre media table based on a genre_id and media_id.
    ///
    /// # Arguments
    /// * `genre_id` - id of the genre we are trying to link to a media object.
    /// * `media_id` - id of the media object we are trying to link to a media.
    /// * `conn` - mutable reference to a sqlx transaction.
    pub async fn insert_pair(
        genre_id: i64,
        media_id: i64,
        conn: &mut crate::Transaction<'_>,
    ) -> Result<i64, DatabaseError> {
        if let Some(r) = sqlx::query!(
            "SELECT genre.id FROM genre
            JOIN genre_media
            WHERE genre_media.media_id = ?
            AND genre_media.genre_id = ?",
            media_id,
            genre_id
        )
        .fetch_optional(&mut **conn)
        .await?
        {
            return Ok(r.id);
        }

        let id = sqlx::query!(
            "INSERT INTO genre_media (genre_id, media_id)
            VALUES ($1, $2)",
            genre_id,
            media_id
        )
        .execute(&mut **conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
