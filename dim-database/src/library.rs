use crate::DatabaseError;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;
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

impl TryFrom<&str> for MediaType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "movie" | "movies" => Ok(Self::Movie),
            "tv" | "tv_show" | "tv show" | "tv shows" => Ok(Self::Tv),
            "episode" | "episodes" | "ep" => Ok(Self::Episode),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for MediaType {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
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
    pub id: i64,
    /// unique name of the library
    pub name: String,

    /// a path on the filesystem that holds media. ie /home/user/media/movies
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,

    /// Enum used to identify the media type that this library contains. At the
    /// moment only `movie` and `tv` are supported
    // TODO: support mixed content, music
    pub media_type: MediaType,
    /// Is library hidden?
    pub hidden: bool,
}

impl Library {
    /// Method returns all libraries that exist in the database in the form of a Vec.
    /// If no libraries are found the the Vec will just be empty.
    ///
    /// This method will not return the locations indexed for this library, if you need those you
    /// must query for them separately.
    pub async fn get_all(conn: &mut crate::Transaction<'_>) -> Vec<Self> {
        sqlx::query!(r#"SELECT id, name, media_type as "media_type: MediaType", hidden as "hidden: bool" FROM library WHERE NOT hidden"#)
            .fetch_all(&mut **conn)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|x| Self {
                id: x.id,
                name: x.name,
                media_type: x.media_type,
                hidden: x.hidden,
                locations: vec![],
            })
            .collect()
    }

    pub async fn get_locations(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<Vec<String>, DatabaseError> {
        Ok(sqlx::query_scalar!(
            "SELECT location FROM indexed_paths
            WHERE library_id = ?",
            id
        )
        .fetch_all(&mut **conn)
        .await?)
    }

    /// Method filters the database for a library with the id supplied and returns it.
    /// This method will also fetch the indexed locations for this library.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    pub async fn get_one(
        conn: &mut crate::Transaction<'_>,
        lib_id: i64,
    ) -> Result<Self, DatabaseError> {
        let library = sqlx::query!(
            r#"SELECT id, name, media_type as "media_type: MediaType", hidden as "hidden: bool" FROM library
            WHERE id = ?"#,
            lib_id
        )
        .fetch_one(&mut **conn)
        .await?;

        let locations = sqlx::query_scalar!(
            r#"SELECT location FROM indexed_paths
            WHERE library_id = ?"#,
            lib_id
        )
        .fetch_all(&mut **conn)
        .await?;

        Ok(Self {
            id: library.id,
            name: library.name,
            media_type: library.media_type,
            hidden: library.hidden,
            locations,
        })
    }

    /// Method filters the database for a library with the id supplied and deletes it.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    pub async fn delete(
        conn: &mut crate::Transaction<'_>,
        id_to_del: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!("DELETE FROM library WHERE id = ?", id_to_del)
            .execute(&mut **conn)
            .await?
            .rows_affected() as usize)
    }

    pub async fn mark_hidden(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(
            sqlx::query!("UPDATE library SET hidden = 1 WHERE id = ?", id)
                .execute(&mut **conn)
                .await?
                .rows_affected() as usize,
        )
    }

    pub async fn get_size(
        tx: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!(
            "SELECT COUNT(id) as size FROM _tblmedia WHERE library_id = ?",
            id
        )
        .fetch_one(&mut **tx)
        .await?
        .size as usize)
    }
}

/// InsertableLibrary struct, same as [`Library`](Library) but without the id field.
#[derive(Clone, Serialize, Deserialize)]
pub struct InsertableLibrary {
    pub name: String,
    pub locations: Vec<String>,
    pub media_type: MediaType,
}

impl InsertableLibrary {
    /// Method inserts a InsertableLibrary object into the database (makes a new library).
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    pub async fn insert(&self, conn: &mut crate::Transaction<'_>) -> Result<i64, DatabaseError> {
        let lib_id = sqlx::query!(
            r#"INSERT INTO library (name, media_type) VALUES ($1, $2)"#,
            self.name,
            self.media_type
        )
        .execute(&mut **conn)
        .await?
        .last_insert_rowid();

        for location in &self.locations {
            sqlx::query!(
                r#"INSERT into indexed_paths(location, library_id)
                VALUES ($1, $2)"#,
                location,
                lib_id
            )
            .execute(&mut **conn)
            .await?;
        }

        Ok(lib_id)
    }
}
