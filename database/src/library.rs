use crate::DatabaseError;
use cfg_if::cfg_if;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

/// Enum represents a media type and can be used on a library or on a media.
/// When returned in a http response, the fields are lowercase.
#[derive(Copy, Serialize, Debug, Clone, Eq, PartialEq, Deserialize, Hash, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
    Episode,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Movie => "movie",
                Self::Tv => "tv",
                Self::Episode => "episode",
            }
        )
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Movie
    }
}

/// Library struct which we can use to deserialize database queries into.
#[derive(Serialize, Deserialize, Clone)]
pub struct Library {
    /// unique id provided by postgres
    pub id: i64,
    /// unique name of the library
    pub name: String,

    /// a path on the filesystem that holds media. ie /home/user/media/movies
    // TODO: convert location from `String` to `Vec<String>` to allow for one library to hold more
    // than one path to media.
    pub location: String,

    /// Enum used to identify the media type that this library contains. At the
    /// moment only `movie` and `tv` are supported
    // TODO: support mixed content, music
    pub media_type: MediaType,
}

impl Library {
    /// Method returns all libraries that exist in the database in the form of a Vec.
    /// If no libraries are found the the Vec will just be empty.
    pub async fn get_all(conn: &crate::DbConnection) -> Vec<Self> {
        sqlx::query_as!(
            Library,
            r#"SELECT id as "id!", name, location, media_type as "media_type: _" FROM library"#
        )
        .fetch_all(conn)
        .await
        .unwrap_or_default()
    }

    /// Method filters the database for a library with the id supplied and returns it.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](crate::DbConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    pub async fn get_one(
        conn: &crate::DbConnection,
        lib_id: i64,
    ) -> Result<Library, DatabaseError> {
        Ok(sqlx::query_as!(
            Library,
            r#"SELECT id as "id!", name, location, media_type as "media_type: _" FROM library WHERE id = ?"#,
            lib_id
        )
        .fetch_one(conn)
        .await?)
    }

    /// Method filters the database for a library with the id supplied and deletes it.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](crate::DbConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    pub async fn delete(
        conn: &crate::DbConnection,
        id_to_del: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!("DELETE FROM library WHERE id = ?", id_to_del)
            .execute(conn)
            .await?
            .rows_affected() as usize)
    }
}

/// InsertableLibrary struct, same as [`Library`](Library) but without the id field.
#[derive(Clone, Serialize, Deserialize)]
pub struct InsertableLibrary {
    pub name: String,
    pub location: String,
    pub media_type: MediaType,
}

impl InsertableLibrary {
    /// Method inserts a InsertableLibrary object into the database (makes a new library).
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](crate::DbConnection)
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i64, DatabaseError> {
        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(sqlx::query!(
                        r#"INSERT INTO library (name, location, media_type)
                           VALUES ($1, $2, $3)
                           RETURNING id"#,
                        self.name,
                        self.location,
                        self.media_type)
                    .fetch_one(conn)
                    .await?)

            } else {
                Ok(sqlx::query!(
                        r#"INSERT INTO library (name, location, media_type) VALUES ($1, $2, $3)"#,
                        self.name,
                        self.location,
                        self.media_type)
                    .execute(conn)
                    .await?
                    .last_insert_rowid())
            }
        }
    }
}
