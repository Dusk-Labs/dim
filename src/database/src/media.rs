use crate::library::{Library, MediaType};
use crate::mediafile::MediaFile;
use crate::schema::media;
use crate::streamablemedia::InsertableStreamableMedia;
use crate::streamablemedia::StreamableTrait;
use crate::tv::StaticTrait;
use cfg_if::cfg_if;
use diesel::prelude::*;

/// Marker trait used to mark media types that inherit from Media.
/// Used internally by InsertableTVShow.
pub trait MediaTrait {}

/// Media struct that represents a media object, usually a movie, tv show or a episode of a tv
/// show. This struct is returned by several methods and can be serialized to json.
#[derive(Clone, Identifiable, Queryable, Serialize, Deserialize, Debug, Associations, Default)]
#[belongs_to(Library, foreign_key = "library_id")]
#[table_name = "media"]
pub struct Media {
    /// unique id automatically assigned by postgres.
    pub id: i32,
    /// id of the library that this media objects belongs to.
    pub library_id: i32,
    /// name of this media object. Usually the title of a movie, episode or tv show.
    pub name: String,
    /// description of this media object. Usually overview of a movie etc.
    pub description: Option<String>,
    /// rating provided by any API that is encoded as a signed integer. Usually TMDB rating.
    pub rating: Option<i32>,
    /// Year in which this movie/tv show/episode was released/aired.
    pub year: Option<i32>,
    /// Date when this media object was created and inserted into the database. Used by several
    /// routes to return sorted lists of medias, based on when they were scanned and inserted into
    /// the db.
    pub added: Option<String>,
    /// Path to the media poster.
    pub poster_path: Option<String>,
    /// Path to the backdrop for this media object.
    pub backdrop_path: Option<String>,
    /// Media type encoded as a string. Either movie/tv/episode or none.
    // TODO: Use a enum instead of a string
    #[serde(flatten)]
    pub media_type: Option<MediaType>,
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
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media};
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie,
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let library_id = new_library.insert(&conn).unwrap();
    /// let library = Library::get_one(&conn, library_id).unwrap();
    ///
    /// let new_media = InsertableMedia {
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.insert(&conn).unwrap();
    /// let media = Media::get_all(&conn, library).unwrap().pop().unwrap();
    ///
    /// assert_eq!(media.library_id, library_id);
    ///
    /// let media_from_library = Library::get(&conn, library_id).unwrap().pop().unwrap();
    ///
    /// assert_eq!(media, media_from_library);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub fn get_all(
        conn: &crate::DbConnection,
        library: Library,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let result = Self::belonging_to(&library)
            .filter(media::media_type.ne(MediaType::Episode))
            .load::<Self>(conn)?;
        Ok(result)
    }

    /// Method returns a media object based on its id
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `req_id` - id of a media that we'd like to match against.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media};
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
    ///
    /// let media = Media::get(&conn, new_media_id).unwrap();
    ///
    /// assert_eq!(media.id, new_media_id);
    /// assert_eq!(media.library_id, library_id);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub fn get(conn: &crate::DbConnection, req_id: i32) -> Result<Self, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = media.filter(id.eq(req_id)).first::<Self>(conn)?;

        Ok(result)
    }

    /// Method to get a entry in a library based on name and library
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `library` - reference to a library object
    /// * `name` - string slice reference containing the name we would like to filter by.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media};
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie,
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let library_id = new_library.insert(&conn).unwrap();
    /// let library = Library::get_one(&conn, library_id).unwrap();
    ///
    /// let new_media = InsertableMedia {
    ///     name: "test".to_string(),
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.insert(&conn).unwrap();
    ///
    /// let media = Media::get_by_name_and_lib(&conn, &library, "test").unwrap();
    ///
    /// assert_eq!(media.id, new_media_id);
    /// assert_eq!(media.library_id, library_id);
    /// assert_eq!(media.name, new_media.name);
    ///
    /// let not_media = Media::get_by_name_and_lib(&conn, &library, "doesntexist");
    /// assert!(not_media.is_err());
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub fn get_by_name_and_lib(
        conn: &crate::DbConnection,
        library: &Library,
        name: &str,
    ) -> Result<Self, diesel::result::Error> {
        let result = Self::belonging_to(library).load::<Self>(conn)?;

        // Manual filter because of a bug with combining filter with belonging_to
        for i in result {
            if i.name == *name {
                return Ok(i);
            }
        }

        Err(diesel::result::Error::NotFound)
    }

    pub fn get_of_mediafile(
        conn: &crate::DbConnection,
        mediafile: &MediaFile,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::mediafile;

        mediafile::table
            .inner_join(media::table)
            .filter(mediafile::id.eq(mediafile.id))
            .select(media::all_columns)
            .first::<Self>(conn)
    }

    /// Method deletes a media object based on its id.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `id_to_del` - id of a media object we want to delete
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media};
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
    ///     name: "test".to_string(),
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.insert(&conn).unwrap();
    /// let rows = Media::delete(&conn, new_media_id).unwrap();
    ///
    /// assert!(rows == 1);
    ///
    /// let not_media = Media::get(&conn, new_media_id);
    /// assert!(not_media.is_err());
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub fn delete(
        conn: &crate::DbConnection,
        id_to_del: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = diesel::delete(media.filter(id.eq(id_to_del))).execute(conn)?;
        Ok(result)
    }

    /// This function exists because for some reason `CASCADE DELETE` doesnt work with a sqlite
    /// backend. Thus we must manually delete entries when deleting a library.
    pub fn delete_by_lib_id(
        conn: &crate::DbConnection,
        lib_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        diesel::delete(media.filter(library_id.eq(lib_id))).execute(conn)
    }
}

/// Struct which represents a insertable media object. It is usually used only by the scanners to
/// insert new media objects. It is the same as [`Media`](Media) except it doesnt have the
/// [`id`](Media::id) field.
#[derive(Default, Insertable, Debug)]
#[table_name = "media"]
pub struct InsertableMedia {
    pub library_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub year: Option<i32>,
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
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media};
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
    ///     name: "test".to_string(),
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.insert(&conn).unwrap();
    ///
    /// let invalid_lib = InsertableMedia {
    ///     name: "test".to_string(),
    ///     library_id: 123123123,
    ///     ..Default::default()
    /// };
    /// let fail = invalid_lib.insert(&conn);
    /// assert!(fail.is_err());
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub fn insert(&self, conn: &crate::DbConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        library
            .filter(id.eq(self.library_id))
            .first::<Library>(conn)?;

        let query = diesel::insert_into(media::table).values(self);

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                query.returning(media::id)
                    .get_result(conn)
            } else {
                query.execute(conn)?;
                diesel::select(crate::last_insert_rowid).get_result(conn)
            }
        }
    }

    /// Method used as a intermediary to insert media objects into a middle table used as a marker
    /// for anything that can be streamed. For example movies and episodes would be using this
    /// method on insertion, while tv shows dont as they cant be streamed.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `manual_insert` - flag to denote whether we want to insert the object into its table
    /// automatically
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::movie::{InsertableMovie, Movie};
    /// use database::streamablemedia::StreamableTrait;
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
    ///     name: "test".to_string(),
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.into_streamable::<InsertableMovie>(&conn, None).unwrap();
    /// let new_media_id = new_media.into_streamable::<InsertableMovie>(&conn, Some(())).unwrap();
    /// let _ = InsertableMovie::new(new_media_id).insert(&conn).unwrap();
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    pub fn into_streamable<T: StreamableTrait>(
        &self,
        conn: &crate::DbConnection,
        manual_insert: Option<()>,
    ) -> Result<i32, diesel::result::Error> {
        let id = self.insert(conn).unwrap();
        let _ = InsertableStreamableMedia::insert(id, conn)?;

        match manual_insert {
            Some(_) => Ok(id),
            None => T::new(id).insert(conn),
        }
    }

    /// Method used as a intermediary to insert media objects into a middle table used as a marker
    /// for anything that cannot be streamed. For example tv shows would be using this
    /// method on insertion, while movies and episodes dont as they cant be streamed.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// automatically
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::{InsertableTVShow, TVShow};
    /// use database::tv::StaticTrait;
    ///
    /// let new_library = InsertableLibrary {
    ///     name: "test".to_string(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let conn = get_conn().unwrap();
    /// let library_id = new_library.insert(&conn).unwrap();
    ///
    /// let new_media = InsertableMedia {
    ///     name: "test".to_string(),
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.into_static::<InsertableTVShow>(&conn).unwrap();
    /// let show = TVShow::get(&conn, new_media_id).unwrap();
    ///
    /// assert_eq!(show.id, new_media_id);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    pub fn into_static<T: StaticTrait>(
        &self,
        conn: &crate::DbConnection,
    ) -> Result<i32, diesel::result::Error> {
        let id = self.insert(conn).unwrap();
        T::new(id).insert(conn)
    }
}

/// Struct which is used when we need to update information about a media object. Same as
/// [`InsertableMedia`](InsertableMedia) except `library_id` cannot be changed and everything field
/// is a `Option<T>`.
#[derive(Default, AsChangeset, Deserialize, Debug)]
#[table_name = "media"]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub year: Option<i32>,
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
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::media::{InsertableMedia, Media, UpdateMedia};
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
    ///     name: "test".to_string(),
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.insert(&conn).unwrap();
    ///
    /// let media = Media::get(&conn, new_media_id).unwrap();
    /// assert_eq!(media.name, new_media.name);
    ///
    /// let update_media = UpdateMedia {
    ///     name: Some("new_test".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let rows = update_media.update(&conn, new_media_id).unwrap();
    /// assert!(rows == 1);
    ///
    /// let media = Media::get(&conn, new_media_id).unwrap();
    /// assert_eq!(media.name, update_media.name.unwrap());
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub fn update(
        &self,
        conn: &crate::DbConnection,
        _id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let entry = media.filter(id.eq(_id));

        diesel::update(entry).set(self).execute(conn)
    }
}
