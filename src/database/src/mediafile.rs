use crate::library::Library;
use crate::media::Media;
use crate::schema::mediafile;
use crate::streamable_media::StreamableMedia;
use crate::DatabaseError;
use crate::retry_while;

use cfg_if::cfg_if;

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use tokio_diesel::*;

/// MediaFile struct which represents a media file on the filesystem. This struct holds some basic
/// information which the video player on the front end might require.
#[derive(Identifiable, Queryable, Serialize, PartialEq, Debug, Associations, Clone)]
#[belongs_to(Library, foreign_key = "library_id")]
#[belongs_to(StreamableMedia, foreign_key = "media_id")]
#[table_name = "mediafile"]
pub struct MediaFile {
    /// Unique identifier provided by postgres
    pub id: i32,
    /// Foreign key linking this entry to the media table or [`Media`](Media) struct
    pub media_id: Option<i32>,
    /// Library foreign key linking this entry to the library table or [`Library`](Library) struct
    pub library_id: i32,
    /// String representing the file path of the file we target. This should be a real path on the
    /// filesystem.
    pub target_file: String,

    /// Raw name that we extract from the filename using regex and the parse-torrent-name library
    pub raw_name: String,
    /// Raw year we might be able to extract from the filename using regex and the
    /// parse-torrent-name library
    pub raw_year: Option<i32>,

    /// Quality string that we might get from ffprobe when running it against our file
    pub quality: Option<String>,
    /// Codec that we might get from ffprobe when running it against our file
    pub codec: Option<String>,
    /// Container descriptor that we might get from ffprobe
    pub container: Option<String>,
    /// Audio codec specifier that we might get from ffprobe
    pub audio: Option<String>,
    /// Video resolution that we can obtain from ffprobe
    pub original_resolution: Option<String>,
    /// Duration of the video file that we obtain from ffprobe
    pub duration: Option<i32>,

    /// Episode number that we might get from using regex and the parse-torrent-name crate. This is
    /// specific to tv shows only.
    pub episode: Option<i32>,
    /// Season number that we might get from using regexa and the parse-torrent-name crate. This is
    /// specific to tv shows only.
    pub season: Option<i32>,

    /// Flag which tells us if the file is corrupted or not. ie if ffprobe cant open the file and
    /// reports no metadata this flag will be set.
    pub corrupt: Option<bool>,
}

impl MediaFile {
    /// Method returns all mediafiles associated with a library.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `lib` - reference to a Library object that we will match against
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile};
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
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     target_file: format!("/dev/null/{}", library_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let media = MediaFile::get_by_lib(&conn, &library).unwrap().pop().unwrap();
    ///
    /// assert_eq!(media.library_id, library_id);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// let _ = MediaFile::delete(&conn, media.id);
    /// ```
    pub async fn get_by_lib(
        conn: &crate::DbConnection,
        lib: &Library,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(mediafile::dsl::mediafile
            .filter(mediafile::library_id.eq(lib.id))
            .load_async::<Self>(conn)
            .await?)
    }

    /// Method returns all mediafiles associated with a library and filters for those not
    /// associated with a media
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `lib` - reference to a Library object that we will match against
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile};
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
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     target_file: format!("/dev/null/{}", library_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let media = MediaFile::get_by_lib_null_media(&conn, &library).unwrap().pop().unwrap();
    ///
    /// assert_eq!(media.library_id, library_id);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// let _ = MediaFile::delete(&conn, media.id);
    /// ```
    pub async fn get_by_lib_null_media(
        conn: &crate::DbConnection,
        lib: &Library,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(mediafile::dsl::mediafile
            .filter(mediafile::library_id.eq(lib.id))
            .filter(mediafile::media_id.is_null())
            .load_async::<Self>(conn)
            .await?)
    }
    /// Method returns all mediafiles associated with a Media object.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `lib` - reference to a Library object that we will match against
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile};
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
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.into_streamable::<InsertableMovie>(&conn, None).unwrap();
    /// let media = Media::get(&conn, new_media_id).unwrap();
    ///
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     media_id: Some(new_media_id),
    ///     target_file: format!("/dev/null/{}", new_media_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let mediafile = MediaFile::get_of_media(&conn, &media).unwrap().pop().unwrap();
    ///
    /// assert_eq!(mediafile.library_id, library_id);
    /// assert_eq!(mediafile.media_id.unwrap(), new_media_id);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// let _ = MediaFile::delete(&conn, media.id);
    /// ```
    pub async fn get_of_media(
        conn: &crate::DbConnection,
        media: &Media,
    ) -> Result<Vec<Self>, DatabaseError> {
        use crate::schema::streamable_media;

        let streamable_media = streamable_media::dsl::streamable_media
            .filter(streamable_media::id.eq(media.id))
            .first_async::<StreamableMedia>(conn)
            .await?;

        // TODO: Figure out why the fuck .filter against mediafile::corrupted doesnt fucking work.
        // Fuck you.
        Ok(mediafile::dsl::mediafile
            .filter(mediafile::media_id.eq(streamable_media.id))
            .load_async::<Self>(conn)
            .await?)
    }

    /// Method returns all metadata of a mediafile based on the id supplied.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `_id` - id of the mediafile object we are targetting
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile};
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
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     target_file: format!("/dev/null/{}", library_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let mediafile = MediaFile::get_one(&conn, mediafile_id).unwrap();
    ///
    /// assert_eq!(mediafile.library_id, library_id);
    /// assert_eq!(mediafile.id, mediafile_id);
    /// assert_eq!(mediafile.raw_name, "nullfile".to_string());
    ///
    /// let non_existent_mediafile = MediaFile::get_one(&conn, 1123123123);
    ///
    /// assert!(non_existent_mediafile.is_err());
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// let _ = MediaFile::delete(&conn, mediafile.id);
    /// ```
    pub async fn get_one(conn: &crate::DbConnection, _id: i32) -> Result<Self, DatabaseError> {
        use crate::schema::mediafile::dsl::*;

        Ok(mediafile
            .filter(id.eq(_id))
            .first_async::<Self>(conn)
            .await?)
    }

    /// Method checks whether a mediafile entry with the filepath supplied exists or not, returning
    /// a bool.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `file` - string slice containing our filepath
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile};
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
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     target_file: format!("/dev/null/{}", library_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let exists = MediaFile::exists_by_file(&conn, format!("/dev/null/{}", library_id).as_str());
    ///
    /// assert!(exists);
    ///
    /// let doesnt_exist = MediaFile::exists_by_file(&conn, "/dev/null/doesntexist");
    ///
    /// assert!(!doesnt_exist);
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// let _ = MediaFile::delete(&conn, mediafile_id);
    /// ```
    pub async fn exists_by_file(conn: &crate::DbConnection, file: &str) -> bool {
        use crate::schema::mediafile::dsl::*;
        use diesel::dsl::exists;
        use diesel::dsl::select;

        let file = file.to_string();

        select(exists(mediafile.filter(target_file.eq(file))))
            .get_result_async(conn)
            .await
            .unwrap()
    }

    pub async fn get_by_file(
        conn: &crate::DbConnection,
        file: &str,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::mediafile::dsl::*;

        Ok(mediafile
            .filter(target_file.eq(file.to_string()))
            .first_async::<Self>(conn)
            .await?)
    }

    /// Method deletes mediafile matching the id supplied
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `_id` - id of the mediafile entry we want to delete
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile};
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
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     target_file: format!("/dev/null/{}", library_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let mediafile = MediaFile::get_one(&conn, mediafile_id).unwrap();
    ///
    /// assert_eq!(mediafile.library_id, library_id);
    /// assert_eq!(mediafile.id, mediafile_id);
    ///
    /// let rows = MediaFile::delete(&conn, mediafile_id).unwrap();
    /// assert!(rows == 1);
    ///
    /// let mediafile = MediaFile::get_one(&conn, mediafile_id);
    /// assert!(mediafile.is_err());
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// ```
    pub async fn delete(conn: &crate::DbConnection, _id: i32) -> Result<usize, DatabaseError> {
        use crate::schema::mediafile::dsl::*;

        Ok(diesel::delete(mediafile.filter(id.eq(_id)))
            .execute_async(conn)
            .await?)
    }

    /// Function deletes all mediafiles with `library_id` of lib_id. This function is used when
    /// deleting a library with a sqlite backend.
    pub async fn delete_by_lib_id(
        conn: &crate::DbConnection,
        lib_id: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::mediafile::dsl::*;

        Ok(diesel::delete(mediafile.filter(library_id.eq(lib_id)))
            .execute_async(conn)
            .await?)
    }
}

/// Same as [`MediaFile`](MediaFile) except its missing the id field.
#[derive(Clone, Insertable, Serialize, Debug, Default)]
#[table_name = "mediafile"]
pub struct InsertableMediaFile {
    pub media_id: Option<i32>,
    pub library_id: i32,
    pub target_file: String,

    pub raw_name: String,
    pub raw_year: Option<i32>,

    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i32>,

    /***
     * Options specific to tv show scanner hence Option<T>
     ***/
    pub episode: Option<i32>,
    pub season: Option<i32>,
    /*** ***/
    pub corrupt: Option<bool>,
}

impl InsertableMediaFile {
    /// Method inserts a new mediafile into the database.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile};
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
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     target_file: format!("/dev/null/{}", library_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let mediafile = MediaFile::get_one(&conn, mediafile_id).unwrap();
    ///
    /// assert_eq!(mediafile.library_id, library_id);
    /// assert_eq!(mediafile.id, mediafile_id);
    ///
    /// let mediafile = MediaFile::get_one(&conn, 123123123);
    /// assert!(mediafile.is_err());
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// let _ = MediaFile::delete(&conn, mediafile_id);
    /// ```
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        use crate::schema::mediafile::dsl::*;

        let query = diesel::insert_into(mediafile).values(self.clone());

        Ok(retry_while!(DatabaseErrorKind::SerializationFailure, {
            conn.transaction::<_, _>(|conn| {
                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        let _ = diesel::sql_query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
                            .execute(conn);

                        query.returning(id)
                            .get_result(conn)
                    } else {
                        query.execute(conn)?;
                        diesel::select(crate::last_insert_rowid).get_result(conn)
                    }
                }
            })
            .await
        })?)
    }
}

/// Same as [`MediaFile`](MediaFile) except its missing the id and library_id fields. Everything is
/// optional too.
#[derive(Clone, Default, AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name = "mediafile"]
pub struct UpdateMediaFile {
    pub media_id: Option<i32>,
    pub target_file: Option<String>,
    pub raw_name: Option<String>,
    pub raw_year: Option<i32>,
    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i32>,

    /***
     * Options specific to tv show scanner hence Option<T>
     ***/
    pub episode: Option<i32>,
    pub season: Option<i32>,
    /*** ***/
    pub corrupt: Option<bool>,
}

impl UpdateMediaFile {
    /// Method updates the columns of a mediafile entry with what is supplied. The row is selected
    /// based on its id.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `_id` - id of the mediafile row we are targetting
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{InsertableLibrary, Library, MediaType};
    /// use database::mediafile::{InsertableMediaFile, MediaFile, UpdateMediaFile};
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
    ///     library_id,
    ///     ..Default::default()
    /// };
    ///
    /// let new_media_id = new_media.into_streamable::<InsertableMovie>(&conn, None).unwrap();
    /// let media = Media::get(&conn, new_media_id).unwrap();
    ///
    /// let new_mediafile = InsertableMediaFile {
    ///     library_id,
    ///     target_file: format!("/dev/null/{}", library_id).to_string(),
    ///     raw_name: "nullfile".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mediafile_id = new_mediafile.insert(&conn).unwrap();
    /// let mediafile = MediaFile::get_one(&conn, mediafile_id).unwrap();
    ///
    /// assert_eq!(mediafile.library_id, library_id);
    /// assert_eq!(mediafile.media_id, None);
    ///
    /// let update_mediafile = UpdateMediaFile {
    ///     media_id: Some(new_media_id),
    ///     ..Default::default()
    /// };
    ///
    /// let rows = update_mediafile.update(&conn, mediafile_id).unwrap();
    /// assert!(rows == 1);
    ///
    /// let mediafile = MediaFile::get_one(&conn, mediafile_id).unwrap();
    /// assert_eq!(mediafile.library_id, library_id);
    /// assert_eq!(mediafile.media_id, Some(new_media_id));
    ///
    /// // clean up the test
    /// let _ = Library::delete(&conn, library_id);
    /// let _ = MediaFile::delete(&conn, mediafile_id);
    /// ```
    pub async fn update(
        &self,
        conn: &crate::DbConnection,
        _id: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::mediafile::dsl::*;

        let entry = mediafile.filter(id.eq(_id));
        Ok(diesel::update(entry)
            .set(self.clone())
            .execute_async(conn)
            .await?)
    }
}

impl Into<Media> for MediaFile {
    fn into(self) -> Media {
        Media {
            id: self.id,
            library_id: self.library_id,
            name: self.raw_name,
            ..Default::default()
        }
    }
}
