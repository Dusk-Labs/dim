use crate::media::*;
use crate::schema::tv_show;
use crate::DatabaseError;

use diesel::prelude::*;
use tokio_diesel::*;

use async_trait::async_trait;
use cfg_if::cfg_if;

/// Trait used as a marker to mark media entries that cannot be streamed, as in not being directly
/// linked to a file on the filesystem. For example tv shows.
#[async_trait]
pub trait StaticTrait {
    /// Required method returning a instance of a object we'd like to mark as static.
    ///
    /// # Arguments
    /// * `id` - id of a media object.
    fn new(id: i32) -> Self;
    /// Required method that inserts Self into the database returning its id.
    async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError>;
}

/// Struct represents a tv show entry in the database.
/// This is mostly used as a marker to mark shows from movies, and episodes.
#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(Media, foreign_key = "id")]
#[table_name = "tv_show"]
pub struct TVShow {
    /// id of a media object we marked as a tv show.
    pub id: i32,
}

/// Struct represents a insertable tv show entry in the database.
/// This is mostly used as a marker to mark shows from movies, and episodes.
#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "tv_show"]
pub struct InsertableTVShow {
    /// id of a media object we'd like to mark as a tv show.
    pub id: i32,
}

impl TVShow {
    /// Method returns a media object and has the same behaviour as [`Media::get`](Media::get)
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of the tv show we are requesting.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::{InsertableTVShow, TVShow};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let show = TVShow::get(&conn, show_id).unwrap();
    ///
    /// assert_eq!(show.id, show_id);
    /// assert_eq!(show.library_id, library_id);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn get(conn: &crate::DbConnection, req_id: i32) -> Result<Media, DatabaseError> {
        use crate::schema::media::dsl::*;
        Ok(media.filter(id.eq(req_id)).first_async(conn).await?)
    }

    /// Method returns all the tv shows in the database.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::{InsertableTVShow, TVShow};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let show = TVShow::get_all(&conn).unwrap();
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn get_all(conn: &crate::DbConnection) -> Result<Vec<Media>, DatabaseError> {
        use crate::schema::media;

        Ok(media::dsl::media
            .inner_join(tv_show::dsl::tv_show)
            .select(media::all_columns)
            .load_async(conn)
            .await?)
    }

    /// Upgrades a TV Show object into a Media object
    pub async fn upgrade(self, conn: &crate::DbConnection) -> Result<Media, DatabaseError> {
        use crate::schema::media;

        Ok(media::dsl::media
            .filter(media::dsl::id.eq(self.id))
            .first_async(conn)
            .await?)
    }
}

#[async_trait]
impl StaticTrait for InsertableTVShow {
    fn new(id: i32) -> Self {
        Self { id }
    }

    /// Method inserts a new tv show in the database.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::tv::StaticTrait;
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let media_id = new_show.insert(&conn).unwrap();
    ///
    /// let show_id = InsertableTVShow::new(media_id).insert(&conn).unwrap();
    /// assert_eq!(media_id, show_id);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        let query = diesel::insert_into(tv_show::table).values(self.clone());

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(query.returning(tv_show::id)
                    .get_result_async(conn).await?)
            } else {
                query.execute_async(conn).await?;
                Ok(diesel::select(crate::last_insert_rowid).get_result_async::<i32>(conn).await?)
            }
        }
    }
}

impl MediaTrait for InsertableTVShow {}
