use crate::library::MediaType;
use crate::DatabaseError;

use serde::Deserialize;
use serde::Serialize;

/// Marker trait used to mark media types that inherit from Media.
/// Used internally by InsertableTVShow.
pub trait MediaTrait {}

/// Media struct that represents a media object, usually a movie, tv show or a episode of a tv
/// show. This struct is returned by several methods and can be serialized to json.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Media {
    /// unique id automatically assigned by postgres.
    pub id: i64,
    /// id of the library that this media objects belongs to.
    pub library_id: i64,
    /// name of this media object. Usually the title of a movie, episode or tv show.
    pub name: String,
    /// description of this media object. Usually overview of a movie etc.
    pub description: Option<String>,
    /// rating provided by any API that is encoded as a signed integer. Usually TMDB rating.
    pub rating: Option<i64>,
    /// Year in which this movie/tv show/episode was released/aired.
    pub year: Option<i64>,
    /// Date when this media object was created and inserted into the database. Used by several
    /// routes to return sorted lists of medias, based on when they were scanned and inserted into
    /// the db.
    pub added: Option<String>,
    /// Path to the media poster.
    pub poster_path: Option<String>,
    /// Path to the backdrop for this media object.
    pub backdrop_path: Option<String>,
    /// Media type encoded as a string. Either movie/tv/episode or none.
    #[serde(flatten)]
    pub media_type: MediaType,
}

impl PartialEq for Media {
    fn eq(&self, other: &Media) -> bool {
        self.id == other.id
    }
}

impl Media {
    /// Method returns all Media objects associated with a Library. Its exactly the same as
    /// [`Library::get`](Library::get) except it takes in a Library object instead of a id.
    /// [`Library::get`](Library::get) is a intermediary to this function, as it calls this
    /// function.
    ///
    /// # Arguments
    /// * `conn` - postgres connection instance
    /// * `library` - a [`Library`](Library) instance
    pub async fn get_all(
        conn: &crate::DbConnection,
        library_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT id as "id!", library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _" FROM media WHERE library_id = ? AND NOT media_type = "episode""#,
                library_id
            )
            .fetch_all(conn)
            .await?)
    }

    /// Method returns a media object based on its id
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `req_id` - id of a media that we'd like to match against.
    pub async fn get(conn: &crate::DbConnection, id: i64) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT id as "id!", library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _" FROM media WHERE id = ?"#,
                id
            )
            .fetch_one(conn)
            .await?)
    }

    /// Method to get a entry in a library based on name and library
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `library` - reference to a library object
    /// * `name` - string slice reference containing the name we would like to filter by.
    pub async fn get_by_name_and_lib(
        conn: &crate::DbConnection,
        library_id: i64,
        name: &str,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT id as "id!", library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _" FROM media WHERE library_id = ? AND name = ? AND NOT media_type = "episode""#,
                library_id,
                name,
            )
            .fetch_one(conn)
            .await?)
    }

    pub async fn get_of_mediafile(
        conn: &crate::DbConnection,
        mediafile_id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT media.id, media.library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _"
                FROM media
                INNER JOIN mediafile ON mediafile.media_id = media.id
                WHERE mediafile.id = ?"#,
                mediafile_id
            ).fetch_one(conn).await?)
    }

    /// Method returns the top rated medias
    pub async fn get_top_rated(
        conn: &crate::DbConnection,
        limit: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT media.id, media.library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _"
                FROM media
                WHERE NOT media_type = "episode"
                GROUP BY id, name
                ORDER BY rating DESC
                LIMIT ?"#,
                limit
            ).fetch_all(conn).await?)
    }

    /// Method returns the recently added medias
    pub async fn get_recently_added(
        conn: &crate::DbConnection,
        limit: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT media.id, media.library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _"
                FROM media
                WHERE NOT media_type = "episode"
                GROUP BY id, name
                ORDER BY added DESC
                LIMIT ?"#,
                limit
            ).fetch_all(conn).await?)
    }

    pub async fn get_random_with(
        conn: &crate::DbConnection,
        limit: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT media.id, media.library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _"
                FROM media
                WHERE NOT media_type = "episode"
                GROUP BY id
                ORDER BY RANDOM()
                LIMIT ?
                "#,
                limit
        ).fetch_all(conn).await?)
    }

    pub async fn get_search(
        conn: &crate::DbConnection,
        query: &str,
        limit: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        let query = format!("%{}%", query);
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT media.id, media.library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _"
                FROM media
                WHERE NOT media_type = "episode"
                AND UPPER(name) LIKE ?
                LIMIT ?
                "#,
                query,
                limit
        ).fetch_all(conn).await?)
    }

    pub async fn get_of_genre(
        conn: &crate::DbConnection,
        genre_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT media.id, media.library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _"
                FROM media
                INNER JOIN genre_media ON genre_media.media_id = media.id
                WHERE NOT media_type = "episode"
                AND genre_media.genre_id = ?
                "#,
                genre_id,
        ).fetch_all(conn).await?)
    }

    pub async fn get_of_year(
        conn: &crate::DbConnection,
        year: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
                Media,
                r#"SELECT media.id, media.library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type as "media_type: _"
                FROM media
                WHERE NOT media_type = "episode"
                AND year = ?
                "#,
                year,
        ).fetch_all(conn).await?)
    }

    /// Method deletes a media object based on its id.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `id` - id of a media object we want to delete
    pub async fn delete(conn: &crate::DbConnection, id: i64) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!("DELETE FROM media WHERE id = ?", id)
            .execute(conn)
            .await?
            .rows_affected() as usize)
    }

    /// This function exists because for some reason `CASCADE DELETE` doesnt work with a sqlite
    /// backend. Thus we must manually delete entries when deleting a library.
    pub async fn delete_by_lib_id(
        conn: &crate::DbConnection,
        library_id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(
            sqlx::query!("DELETE FROM media WHERE library_id = ?", library_id)
                .execute(conn)
                .await?
                .rows_affected() as usize,
        )
    }
}

impl Into<super::tv::TVShow> for Media {
    fn into(self) -> super::tv::TVShow {
        super::tv::TVShow { id: self.id }
    }
}

/// Struct which represents a insertable media object. It is usually used only by the scanners to
/// insert new media objects. It is the same as [`Media`](Media) except it doesnt have the
/// [`id`](Media::id) field.
#[derive(Clone, Default, Debug)]
pub struct InsertableMedia {
    pub library_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub rating: Option<i64>,
    pub year: Option<i64>,
    pub added: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub media_type: MediaType,
}

impl InsertableMedia {
    /// Method used to insert a new media object.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i64, DatabaseError> {
        let tx = conn.begin().await?;

        if let Some(record) = sqlx::query!(r#"SELECT id FROM media where name = ?"#, self.name)
            .fetch_optional(conn)
            .await?
        {
            return Ok(record.id);
        }

        let id = sqlx::query!(
            r#"INSERT INTO media (library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type)
            VALUES ($1, $2, $3, $4, $5, $6,$7, $8, $9)
            ON CONFLICT DO UPDATE
            SET name = $2
            RETURNING media.id as "id!: i64"
            "#,
            self.library_id,
            self.name,
            self.description,
            self.rating,
            self.year,
            self.added,
            self.poster_path,
            self.backdrop_path,
            self.media_type
        ).fetch_one(conn).await?.id;

        tx.commit().await?;
        Ok(id)
    }

    /// Method blindly inserts `self` into the database without checking whether a similar entry exists.
    /// This is especially useful for tv shows as they usually have similar metadata with key differences
    /// which are not indexed in the database.
    pub async fn insert_blind(&self, conn: &crate::DbConnection) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            r#"INSERT INTO media (library_id, name, description, rating, year, added, poster_path, backdrop_path, media_type)
            VALUES ($1, $2, $3, $4, $5, $6,$7, $8, $9)"#,
            self.library_id,
            self.name,
            self.description,
            self.rating,
            self.year,
            self.added,
            self.poster_path,
            self.backdrop_path,
            self.media_type
        ).execute(conn).await?.last_insert_rowid())
    }
}

/// Struct which is used when we need to update information about a media object. Same as
/// [`InsertableMedia`](InsertableMedia) except `library_id` cannot be changed and everything field
/// is a `Option<T>`.
#[derive(Clone, Default, Deserialize, Debug)]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rating: Option<i64>,
    pub year: Option<i64>,
    pub added: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub media_type: Option<MediaType>,
}

impl UpdateMedia {
    /// Method used to update the fields of a media object that is in the database using the id of
    /// this object as a discriminator.
    ///
    /// # Arguments
    /// * `conn` - diesel connection
    /// * `_id` - id of the media object we want to update
    pub async fn update(
        &self,
        conn: &crate::DbConnection,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        let tx = conn.begin().await?;

        crate::opt_update!(conn, tx,
            "UPDATE media SET name = ? WHERE id = ?" => (self.name, id),
            "UPDATE media SET description = ? WHERE id = ?" => (self.description, id),
            "UPDATE media SET rating = ? WHERE id = ?" => (self.rating, id),
            "UPDATE media SET year = ? WHERE id = ?" => (self.year, id),
            "UPDATE media SET added = ? WHERE id = ?" => (self.added, id),
            "UPDATE media SET poster_path = ? WHERE id = ?" => (self.poster_path, id),
            "UPDATE media SET backdrop_path = ? WHERE id = ?" => (self.backdrop_path, id),
            "UPDATE media SET media_type = ? WHERE id = ?" => (self.media_type, id)
        );

        tx.commit().await?;
        Ok(1)
    }
}
