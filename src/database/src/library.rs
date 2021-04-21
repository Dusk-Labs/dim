use crate::media::*;
use crate::schema::library;
use crate::DatabaseError;
use cfg_if::cfg_if;

use diesel::prelude::*;
use tokio_diesel::*;

/// Enum represents a media type and can be used on a library or on a media.
/// When returned in a http response, the fields are lowercase.
#[derive(Serialize, Debug, Clone, DbEnum, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
    Episode,
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Movie
    }
}

/// Library struct which we can use to deserialize database queries into.
#[derive(Queryable, Serialize, Deserialize, Identifiable, Clone)]
#[table_name = "library"]
pub struct Library {
    /// unique id provided by postgres
    pub id: i32,
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
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let new_id = new_library.insert(&conn).unwrap();
    /// let mut libraries = Library::get_all(&conn);
    ///
    /// assert!(libraries.len() > 0);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, new_id).unwrap();
    /// ```
    pub async fn get_all(conn: &crate::DbConnection) -> Vec<Self> {
        use crate::schema::library::dsl::*;

        // TODO: Dont panic on error event tho this technically never panics and just returns a
        // null vec
        library
            .load_async::<Self>(conn)
            .await
            .expect("Error querying all libraries")
    }

    /// Method filters the database for a library with the id supplied and returns in as a Result.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](crate::DbConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie,
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let new_id = new_library.insert(&conn).unwrap();
    /// let library = Library::get_one(&conn, new_id).unwrap();
    ///
    /// assert_eq!(library.id, new_id);
    /// assert_eq!(library.name, new_library.name);
    /// assert_eq!(library.location, new_library.location);
    /// assert_eq!(library.media_type, new_library.media_type);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, new_id).unwrap();
    /// ```
    pub async fn get_one(
        conn: &crate::DbConnection,
        lib_id: i32,
    ) -> Result<Library, DatabaseError> {
        use crate::schema::library::dsl::*;

        Ok(library
            .filter(id.eq(lib_id))
            .first_async::<Self>(conn)
            .await?)
    }

    /// Method filters the database for a library with the id supplied and all Media objects
    /// associated with the library then returns all those as a Vec.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](crate::DbConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia};
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie,
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let library_id = new_library.insert(&conn).unwrap();
    ///
    /// let new_media = InsertableMedia {
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.insert(&conn).unwrap();
    /// let media = Library::get(&conn, library_id).unwrap().pop().unwrap();
    ///
    /// assert_eq!(media.library_id, library_id);
    /// assert_eq!(media.id, new_media_id);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub async fn get(conn: &crate::DbConnection, lib_id: i32) -> Result<Vec<Media>, DatabaseError> {
        use crate::schema::library::dsl::*;
        let result = library
            .filter(id.eq(lib_id))
            .first_async::<Self>(conn)
            .await?;

        Media::get_all(conn, result).await
    }

    /// Method filters the database for a library with the id supplied and deletes it.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](crate::DbConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie,
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let library_id = new_library.insert(&conn).unwrap();
    /// let rows = Library::delete(&conn, library_id).unwrap();
    /// assert_eq!(rows, 1usize);
    /// ```
    pub async fn delete(
        conn: &crate::DbConnection,
        id_to_del: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::library::dsl::*;

        let result = diesel::delete(library.filter(id.eq(id_to_del)))
            .execute_async(conn)
            .await?;

        Ok(result)
    }
}

/// InsertableLibrary struct, same as [`Library`](Library) but without the id field.
#[derive(Clone, Insertable, Serialize, Deserialize)]
#[table_name = "library"]
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
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie,
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let new_id = new_library.insert(&conn).unwrap();
    /// let library = Library::get_one(&conn, new_id).unwrap();
    ///
    /// assert_eq!(library.id, new_id);
    /// assert_eq!(library.name, new_library.name);
    /// assert_eq!(library.location, new_library.location);
    /// assert_eq!(library.media_type, new_library.media_type);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, new_id);
    /// ```
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        let query = diesel::insert_into(library::table).values(self.clone());

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(query.returning(library::id)
                    .get_result_async(conn).await?)
            } else {
                query.execute_async(conn).await?;
                Ok(diesel::select(crate::last_insert_rowid).get_result_async(conn).await?)
            }
        }
    }
}
