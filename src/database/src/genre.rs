use crate::schema::{genre, genre_media};
use diesel::prelude::*;

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

/// Genre entry that can be inserted into the db.
#[derive(Insertable)]
#[table_name = "genre"]
pub struct InsertableGenre {
    /// Genre name
    pub name: String,
}

/// Struct which is used to pair a genre to a media
#[derive(Insertable)]
#[table_name = "genre_media"]
pub struct InsertableGenreMedia {
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
    /// use database::get_conn;
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
    pub fn get_by_name(
        conn: &diesel::PgConnection,
        query: String,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::genre::dsl::*;
        genre.filter(name.ilike(query)).first::<Self>(conn)
    }

    /// Method returns all of the episodes belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `media` - reference to a media object which should be a tv show.
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn get_by_media(
        conn: &diesel::PgConnection,
        query: i32,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        genre::table
            .inner_join(genre_media::table)
            .filter(genre_media::media_id.eq(query))
            .select((genre::dsl::id, genre::dsl::name))
            .load::<Self>(&*conn)
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
    /// use database::get_conn;
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
    pub fn get_by_media_and_genre(
        conn: &diesel::PgConnection,
        genre_id: i32,
        media_id: i32,
    ) -> Result<Self, diesel::result::Error> {
        genre::table
            .inner_join(genre_media::table)
            .filter(genre_media::media_id.eq(media_id))
            .filter(genre_media::genre_id.eq(genre_id))
            .select((genre::dsl::id, genre::dsl::name))
            .first::<Self>(&*conn)
    }

    /// Method removes a genre from the genre table based on its id
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - genre id
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn delete(
        conn: &diesel::PgConnection,
        genre_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::genre::dsl::*;

        diesel::delete(genre.filter(id.eq(genre_id))).execute(conn)
    }
}

impl InsertableGenre {
    /// Method inserts a new genre into the table otherwise returns the id of a existing entry
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::genre::dsl::*;

        // first check if exists
        if let Ok(x) = Genre::get_by_name(&conn, self.name.clone()) {
            return Ok(x.id);
        }

        let result = diesel::insert_into(genre)
            .values(self)
            .returning(id)
            .get_result(conn)?;

        Ok(result)
    }
}

impl InsertableGenreMedia {
    /// Method inserts a new entry into the intermediary genre table linking a genre to a media
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    pub fn insert(&self, conn: &diesel::PgConnection) {
        use crate::schema::genre_media::dsl::*;
        let _ = diesel::insert_into(genre_media).values(self).execute(conn);
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
    /// use database::get_conn;
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
    pub fn insert_pair(genre_id: i32, media_id: i32, conn: &diesel::PgConnection) {
        if Genre::get_by_media_and_genre(&conn, genre_id, media_id).is_ok() {
            return;
        }

        let pair = Self { genre_id, media_id };

        pair.insert(conn);
    }
}
