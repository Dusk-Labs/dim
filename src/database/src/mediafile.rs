use crate::library::Library;
use crate::media::Media;
use crate::schema::mediafile;
use crate::streamablemedia::StreamableMedia;
use diesel::prelude::*;

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

/// Same as [`MediaFile`](MediaFile) except its missing the id field.
#[derive(Insertable, Serialize, Debug, Default)]
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

/// Same as [`MediaFile`](MediaFile) except its missing the id and library_id fields. Everything is
/// optional too.
#[derive(Default, AsChangeset, Deserialize, PartialEq, Debug)]
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

impl MediaFile {
    /// Method returns all mediafiles associated with a library.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `lib` - reference to a Library object that we will match against
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn get_by_lib(
        conn: &diesel::PgConnection,
        lib: &Library,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        Self::belonging_to(lib).load::<Self>(conn)
    }

    /// Method returns all mediafiles associated with a Media object.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `lib` - reference to a Library object that we will match against
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn get_of_media(
        conn: &diesel::PgConnection,
        media: &Media,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let streamable_media =
            StreamableMedia::belonging_to(media).first::<StreamableMedia>(conn)?;

        // TODO: Figure out why the fuck .filter against mediafile::corrupted doesnt fucking work.
        // Fuck you.
        let result = Self::belonging_to(&streamable_media).load::<Self>(conn)?;

        Ok(result)
    }

    /// Method returns all metadata of a mediafile based on the id supplied.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `_id` - id of the mediafile object we are targetting
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn get_one(conn: &diesel::PgConnection, _id: i32) -> Result<Self, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;

        let result = mediafile.filter(id.eq(_id)).first::<Self>(conn)?;

        Ok(result)
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
    /// use database::get_conn;
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
    pub fn exists_by_file(conn: &diesel::PgConnection, file: &str) -> bool {
        use crate::schema::mediafile::dsl::*;
        use diesel::dsl::exists;
        use diesel::dsl::select;
        select(exists(mediafile.filter(target_file.eq(file))))
            .get_result(conn)
            .unwrap()
    }

    /// Method deletes mediafile matching the id supplied
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `_id` - id of the mediafile entry we want to delete
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn delete(conn: &diesel::PgConnection, _id: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;

        let result = diesel::delete(mediafile.filter(id.eq(_id))).execute(conn)?;
        Ok(result)
    }
}

impl InsertableMediaFile {
    /// Method inserts a new mediafile into the database.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;
        let result: i32 = diesel::insert_into(mediafile)
            .values(self)
            .returning(id)
            .get_result(conn)?;

        Ok(result)
    }
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
    /// use database::get_conn;
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
    pub fn update(
        &self,
        conn: &diesel::PgConnection,
        _id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;
        let entry = mediafile.filter(id.eq(_id));

        diesel::update(entry).set(self).execute(conn)
    }
}
