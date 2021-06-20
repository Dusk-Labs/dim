use crate::library::{Library, MediaType};
use crate::mediafile::MediaFile;
use crate::retry_while;
use crate::schema::media;
use crate::streamable_media::InsertableStreamableMedia;
use crate::streamable_media::StreamableTrait;
use crate::tv::StaticTrait;
use crate::DatabaseError;
use cfg_if::cfg_if;

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use tokio_diesel::*;

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
    pub async fn get_all(
        conn: &crate::DbConnection,
        library: Library,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(media::dsl::media
            .filter(media::library_id.eq(library.id))
            .filter(media::media_type.ne(MediaType::Episode))
            .load_async::<Self>(conn)
            .await?)
    }

    /// Method returns a media object based on its id
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `req_id` - id of a media that we'd like to match against.
    pub async fn get(conn: &crate::DbConnection, req_id: i32) -> Result<Self, DatabaseError> {
        use crate::schema::media::dsl::*;

        let result = media
            .filter(id.eq(req_id))
            .first_async::<Self>(conn)
            .await?;

        Ok(result)
    }

    /// Method to get a entry in a library based on name and library
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `library` - reference to a library object
    /// * `name` - string slice reference containing the name we would like to filter by.
    pub async fn get_by_name_and_lib(
        conn: &crate::DbConnection,
        library: &Library,
        name: &str,
    ) -> Result<Self, DatabaseError> {
        Ok(media::dsl::media
            .filter(media::library_id.eq(library.id))
            .filter(media::name.eq(name.to_string()))
            .first_async::<Self>(conn)
            .await?)
    }

    pub async fn get_by_name_and_lib_id(
        conn: &crate::DbConnection,
        library: i32,
        name: &str,
    ) -> Result<Self, DatabaseError> {
        Ok(media::dsl::media
            .filter(media::library_id.eq(library))
            .filter(media::name.eq(name.to_string()))
            .first_async::<Self>(conn)
            .await?)
    }

    pub async fn get_of_mediafile(
        conn: &crate::DbConnection,
        mediafile: &MediaFile,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::mediafile;

        Ok(mediafile::table
            .inner_join(media::table)
            .filter(mediafile::id.eq(mediafile.id))
            .select(media::all_columns)
            .first_async::<Self>(conn)
            .await?)
    }

    /// Method deletes a media object based on its id.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `id_to_del` - id of a media object we want to delete
    pub async fn delete(
        conn: &crate::DbConnection,
        id_to_del: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::media::dsl::*;

        let result = diesel::delete(media.filter(id.eq(id_to_del)))
            .execute_async(conn)
            .await?;
        Ok(result)
    }

    /// This function exists because for some reason `CASCADE DELETE` doesnt work with a sqlite
    /// backend. Thus we must manually delete entries when deleting a library.
    pub async fn delete_by_lib_id(
        conn: &crate::DbConnection,
        lib_id: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::media::dsl::*;

        Ok(diesel::delete(media.filter(library_id.eq(lib_id)))
            .execute_async(conn)
            .await?)
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
#[derive(Clone, Default, Insertable, Debug)]
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
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        use crate::schema::library::dsl::*;

        library
            .filter(id.eq(self.library_id))
            .first_async::<Library>(conn)
            .await?;

        // we need to atomically select or insert.
        Ok(retry_while!(DatabaseErrorKind::SerializationFailure, {
            conn.transaction::<_, _>(|conn| {
                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        let _ = diesel::sql_query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
                            .execute(conn)?;
                    }
                }

                let result = media::table
                    .filter(media::name.eq(self.name.clone()))
                    .select(media::id)
                    .get_result::<i32>(conn);

                if let Ok(x) = result {
                    return Ok(x);
                }

                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        Ok(diesel::insert_into(media::table).values(self.clone())
                            .returning(media::id)
                            .get_result(conn)?)
                    } else {
                        diesel::insert_into(media::table).values(self.clone())
                            .execute(conn)?;
                        Ok(diesel::select(crate::last_insert_rowid).get_result(conn)?)
                    }
                }
            })
            .await
        })?)
    }

    /// Method blindly inserts `self` into the database without checking whether a similar entry exists.
    /// This is especially useful for tv shows as they usually have similar metadata with key differences
    /// which are not indexed in the database.
    pub async fn insert_blind(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        use crate::schema::library::dsl::*;

        library
            .filter(id.eq(self.library_id))
            .first_async::<Library>(conn)
            .await?;

        // we need to atomically select or insert.
        Ok(retry_while!(DatabaseErrorKind::SerializationFailure, {
            conn.transaction::<_, _>(|conn| {
                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        let _ = diesel::sql_query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
                            .execute(conn);
                    }
                }

                let query = diesel::insert_into(media::table).values(self.clone());

                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        Ok(query.returning(media::id)
                           .get_result(conn)?)
                    } else {
                        query.execute(conn)?;
                        Ok(diesel::select(crate::last_insert_rowid).get_result(conn)?)
                    }
                }
            })
            .await
        })?)
    }

    /// Method used as a intermediary to insert media objects into a middle table used as a marker
    /// for anything that can be streamed. For example movies and episodes would be using this
    /// method on insertion, while tv shows dont as they cant be streamed.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `manual_insert` - flag to denote whether we want to insert the object into its table
    /// automatically
    pub async fn into_streamable<T: StreamableTrait>(
        &self,
        conn: &crate::DbConnection,
        id: i32,
        manual_insert: Option<()>,
    ) -> Result<i32, DatabaseError> {
        let _ = InsertableStreamableMedia::insert(id, conn).await?;

        match manual_insert {
            Some(_) => Ok(id),
            None => T::new(id).insert(conn).await,
        }
    }

    /// Method used as a intermediary to insert media objects into a middle table used as a marker
    /// for anything that cannot be streamed. For example tv shows would be using this
    /// method on insertion, while movies and episodes dont as they cant be streamed.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// automatically
    pub async fn into_static<T: StaticTrait>(
        &self,
        conn: &crate::DbConnection,
        id: i32,
    ) -> Result<i32, DatabaseError> {
        T::new(id).insert(conn).await
    }
}

/// Struct which is used when we need to update information about a media object. Same as
/// [`InsertableMedia`](InsertableMedia) except `library_id` cannot be changed and everything field
/// is a `Option<T>`.
#[derive(Clone, Default, AsChangeset, Deserialize, Debug)]
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
    pub async fn update(
        &self,
        conn: &crate::DbConnection,
        _id: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::media::dsl::*;

        let entry = media.filter(id.eq(_id));

        Ok(diesel::update(entry)
            .set(self.clone())
            .execute_async(conn)
            .await?)
    }
}
