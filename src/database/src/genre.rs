use crate::schema::{genre, genre_media};
use crate::DatabaseError;
use cfg_if::cfg_if;

use diesel::prelude::*;
use tokio_diesel::*;

/// Struct shows a single genre entry
#[derive(Clone, Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "genre"]
pub struct Genre {
    pub id: i32,
    /// Genre name, ie "Action"
    pub name: String,
}

/// Intermediary table showing the relationship between a media and a genre
#[derive(Clone, Identifiable, Queryable, Debug, PartialEq)]
#[table_name = "genre_media"]
pub struct GenreMedia {
    pub id: i32,
    pub genre_id: i32,
    pub media_id: i32,
}

impl Genre {
    /// Method returns the entry of a genre if exists based on its name.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `query` - genre name
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::genre::{InsertableGenre, Genre};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// assert!(Genre::get_by_name(&conn, "test2".into()).is_err());
    ///
    /// let new_genre = InsertableGenre {
    ///     name: "test2".into(),
    /// };
    ///
    /// let id = new_genre.insert(&conn).unwrap();
    /// let genre = Genre::get_by_name(&conn, "test2".into()).unwrap();
    ///
    /// assert_eq!(genre.name, "test2".to_string());
    ///
    /// Genre::delete(&conn, id);
    pub async fn get_by_name(
        conn: &crate::DbConnection,
        query: String,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::genre::dsl::*;

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(genre.filter(name.ilike(query)).first_async::<Self>(conn).await?)
            } else {
                Ok(genre.filter(crate::upper(name).like(query.to_uppercase())).first_async::<Self>(conn).await?)
            }
        }
    }

    /// Method returns all of the episodes belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `media` - reference to a media object which should be a tv show.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::genre::{InsertableGenre, Genre, InsertableGenreMedia};
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
    /// let show_id = new_show.insert(&conn).unwrap();
    ///
    /// let new_genre = InsertableGenre {
    ///     name: "test".into(),
    /// };
    ///
    /// let genre_id = new_genre.insert(&conn).unwrap();
    ///
    /// let pair = InsertableGenreMedia {
    ///     genre_id: genre_id,
    ///     media_id: show_id,
    /// };
    ///
    /// pair.insert(&conn);
    ///
    /// let genres = Genre::get_by_media(&conn, show_id).unwrap();
    ///
    /// assert!(genres.len() == 1);
    /// assert_eq!(genres[0].name, "test".to_string());
    ///
    /// Library::delete(&conn, library_id).unwrap();
    /// Genre::delete(&conn, genre_id);
    pub async fn get_by_media(
        conn: &crate::DbConnection,
        query: i32,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(genre::table
            .inner_join(genre_media::table)
            .filter(genre_media::media_id.eq(query))
            .select((genre::dsl::id, genre::dsl::name))
            .load_async::<Self>(conn)
            .await?)
    }

    /// Method returns a genre based on genre_id and media_id
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `genre_id` - id of a genre
    /// * `media_id` - id of a media object
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::genre::{InsertableGenre, Genre, InsertableGenreMedia};
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
    /// let show_id = new_show.insert(&conn).unwrap();
    ///
    /// let new_genre = InsertableGenre {
    ///     name: "test_genre_media".into(),
    /// };
    ///
    /// let genre_id = new_genre.insert(&conn).unwrap();
    ///
    /// let pair = InsertableGenreMedia {
    ///     genre_id: genre_id,
    ///     media_id: show_id,
    /// };
    ///
    /// pair.insert(&conn);
    ///
    /// let genres = Genre::get_by_media_and_genre(&conn, genre_id, show_id).unwrap();
    ///
    /// assert_eq!(genres.name, "test_genre_media".to_string());
    ///
    /// Library::delete(&conn, library_id).unwrap();
    /// Genre::delete(&conn, genre_id);
    pub async fn get_by_media_and_genre(
        conn: &crate::DbConnection,
        genre_id: i32,
        media_id: i32,
    ) -> Result<Self, DatabaseError> {
        Ok(genre::table
            .inner_join(genre_media::table)
            .filter(genre_media::media_id.eq(media_id))
            .filter(genre_media::genre_id.eq(genre_id))
            .select((genre::dsl::id, genre::dsl::name))
            .first_async::<Self>(conn)
            .await?)
    }

    /// Method removes a genre from the genre table based on its id
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - genre id
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::genre::{InsertableGenre, Genre};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// assert!(Genre::get_by_name(&conn, "test".into()).is_err());
    ///
    /// let new_genre = InsertableGenre {
    ///     name: "test".into(),
    /// };
    ///
    /// let id = new_genre.insert(&conn).unwrap();
    /// let genre = Genre::get_by_name(&conn, "test".into()).unwrap();
    ///
    /// assert_eq!(genre.name, "test".to_string());
    ///
    /// Genre::delete(&conn, id);
    ///
    /// assert!(Genre::get_by_name(&conn, "test".into()).is_err());
    pub async fn delete(conn: &crate::DbConnection, genre_id: i32) -> Result<usize, DatabaseError> {
        use crate::schema::genre::dsl::*;

        Ok(diesel::delete(genre.filter(id.eq(genre_id)))
            .execute_async(conn)
            .await?)
    }
}

/// Genre entry that can be inserted into the db.
#[derive(Clone, Insertable)]
#[table_name = "genre"]
pub struct InsertableGenre {
    /// Genre name
    pub name: String,
}

impl InsertableGenre {
    /// Method inserts a new genre into the table otherwise returns the id of a existing entry
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::genre::{InsertableGenre, Genre};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// assert!(Genre::get_by_name(&conn, "test_genre1".into()).is_err());
    ///
    /// let new_genre = InsertableGenre {
    ///     name: "test_genre1".into(),
    /// };
    ///
    /// let id = new_genre.insert(&conn).unwrap();
    /// let genre = Genre::get_by_name(&conn, "test_genre1".into()).unwrap();
    ///
    /// assert_eq!(genre.name, "test_genre1".to_string());
    /// let id2 = new_genre.insert(&conn).unwrap();
    ///
    /// assert_eq!(id, id2);
    ///
    /// Genre::delete(&conn, id);
    /// ```
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        use crate::schema::genre::dsl::*;

        // first check if exists
        if let Ok(x) = Genre::get_by_name(&conn, self.name.clone()).await {
            return Ok(x.id);
        }

        let query = diesel::insert_into(genre).values(self.clone());

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(query.returning(id)
                    .get_result_async(conn).await?)
            } else {
                query.execute_async(conn).await?;

                Ok(diesel::select(crate::last_insert_rowid).get_result_async(conn).await?)
            }
        }
    }
}
/// Struct which is used to pair a genre to a media
#[derive(Clone, Insertable)]
#[table_name = "genre_media"]
pub struct InsertableGenreMedia {
    pub genre_id: i32,
    pub media_id: i32,
}

impl InsertableGenreMedia {
    /// Method inserts a new entry into the intermediary genre table linking a genre to a media
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::genre::{InsertableGenre, Genre, InsertableGenreMedia};
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
    /// let show_id = new_show.insert(&conn).unwrap();
    ///
    /// let new_genre = InsertableGenre {
    ///     name: "test_genre".into(),
    /// };
    ///
    /// let genre_id = new_genre.insert(&conn).unwrap();
    ///
    /// let pair = InsertableGenreMedia {
    ///     genre_id: genre_id,
    ///     media_id: show_id,
    /// };
    ///
    /// pair.insert(&conn);
    ///
    /// let genres = Genre::get_by_media_and_genre(&conn, genre_id, show_id).unwrap();
    ///
    /// assert_eq!(genres.name, "test_genre".to_string());
    ///
    /// Library::delete(&conn, library_id).unwrap();
    /// Genre::delete(&conn, genre_id);
    pub async fn insert(&self, conn: &crate::DbConnection) {
        use crate::schema::genre_media::dsl::*;
        let _ = diesel::insert_into(genre_media)
            .values(self.clone())
            .execute_async(conn)
            .await;
    }

    /// Method inserts a pair into the genre media table based on a genre_id and media_id.
    ///
    /// # Arguments
    /// * `genre_id` - id of the genre we are trying to link to a media object.
    /// * `media_id` - id of the media object we are trying to link to a media.
    /// * `conn` - diesel connection reference to postgres
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::genre::{InsertableGenre, Genre, InsertableGenreMedia};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Movie
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Movie,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.insert(&conn).unwrap();
    ///
    /// let new_genre = InsertableGenre {
    ///     name: "test".into(),
    /// };
    ///
    /// let genre_id = new_genre.insert(&conn).unwrap();
    ///
    /// InsertableGenreMedia::insert_pair(genre_id, show_id, &conn);
    ///
    /// let genres = Genre::get_by_media(&conn, show_id).unwrap();
    ///
    /// assert_eq!(genres.len(), 1);
    /// assert_eq!(genres[0].name, "test".to_string());
    ///
    /// Library::delete(&conn, library_id).unwrap();
    /// Genre::delete(&conn, genre_id);
    pub async fn insert_pair(genre_id: i32, media_id: i32, conn: &crate::DbConnection) {
        if Genre::get_by_media_and_genre(&conn, genre_id, media_id)
            .await
            .is_ok()
        {
            return;
        }

        let pair = Self { genre_id, media_id };

        pair.insert(conn).await;
    }
}
