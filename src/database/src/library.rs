use crate::media::*;
use crate::schema::library;
use diesel::prelude::*;

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

/// InsertableLibrary struct, same as [`Library`](Library) but without the id field.
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "library"]
pub struct InsertableLibrary {
    pub name: String,
    pub location: String,
    pub media_type: MediaType,
}

impl Library {
    /// Method returns all libraries that exist in the database in the form of a Vec.
    /// If no libraries are found the the Vec will just be empty.
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn get_all(conn: &diesel::PgConnection) -> Vec<Self> {
        use crate::schema::library::dsl::*;

        // TODO: Dont panic on error event tho this technically never panics and just returns a
        // null vec
        library
            .load::<Self>(conn)
            .expect("Error querying all libraries")
    }

    /// Method filters the database for a library with the id supplied and returns in as a Result.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](diesel::PgConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn get_one(
        conn: &diesel::PgConnection,
        lib_id: i32,
    ) -> Result<Library, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        library.filter(id.eq(lib_id)).first::<Self>(conn)
    }

    /// Method filters the database for a library with the id supplied and all Media objects
    /// associated with the library then returns all those as a Vec.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](diesel::PgConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn get(
        conn: &diesel::PgConnection,
        lib_id: i32,
    ) -> Result<Vec<Media>, diesel::result::Error> {
        use crate::schema::library::dsl::*;
        let result = library.filter(id.eq(lib_id)).first::<Self>(conn)?;

        Media::get_all(conn, result)
    }

    /// Method filters the database for a library with the id supplied and deletes it.
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](diesel::PgConnection)
    /// * `lib_id` - a integer that is the id of the library we are trying to query
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn delete(
        conn: &diesel::PgConnection,
        id_to_del: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        let result = diesel::delete(library.filter(id.eq(id_to_del))).execute(conn)?;

        Ok(result)
    }
}

impl InsertableLibrary {
    /// Method inserts a InsertableLibrary object into the database (makes a new library).
    ///
    /// # Arguments
    /// * `conn` - [diesel connection](diesel::PgConnection)
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        diesel::insert_into(library::table)
            .values(self)
            .returning(library::id)
            .get_result(conn)
    }
}
