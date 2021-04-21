use crate::schema::season;
use crate::tv::TVShow;
use crate::DatabaseError;

use cfg_if::cfg_if;
use diesel::prelude::*;
use tokio_diesel::*;

/// Struct represents a season entry in the database.
#[derive(
    Identifiable, Associations, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone,
)]
#[belongs_to(TVShow, foreign_key = "tvshowid")]
#[table_name = "season"]
pub struct Season {
    pub id: i32,
    /// Season number
    pub season_number: i32,
    /// Foreign key to the tv show we'd like to link against
    pub tvshowid: i32,
    /// String holding the date when the season was added to the database.
    pub added: Option<String>,
    /// URL to the location of the poster for this season.
    pub poster: Option<String>,
}

impl Season {
    /// Method returns all of the seasons that are linked to a tv show based on a tvshow id
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::{Season, InsertableSeason};
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
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let all_seasons = Season::get_all(&conn, show_id).unwrap();
    ///
    /// assert!(all_seasons.len() == 1);
    ///
    /// let season = &all_seasons[0];
    /// assert_eq!(season.id, season_id);
    /// assert_eq!(season.season_number, 1);
    /// assert_eq!(season.added, Some("test".to_string()));
    /// assert_eq!(season.poster, Some("test".to_string()));
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn get_all(
        conn: &crate::DbConnection,
        tv_id: i32,
    ) -> Result<Vec<Self>, DatabaseError> {
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show
            .find(tv_id)
            .get_result_async::<TVShow>(conn)
            .await?;

        let result = season::dsl::season
            .filter(season::tvshowid.eq(tv_show.id))
            .load_async::<Self>(conn)
            .await?;

        Ok(result)
    }

    /// Method returns the season based on the season number belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - season number we'd like to fetch.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::{Season, InsertableSeason};
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
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let season = Season::get(&conn, show_id, new_season.season_number).unwrap();
    ///
    /// assert_eq!(season.id, season_id);
    /// assert_eq!(season.season_number, 1);
    /// assert_eq!(season.added, Some("test".to_string()));
    /// assert_eq!(season.poster, Some("test".to_string()));
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn get(
        conn: &crate::DbConnection,
        tv_id: i32,
        season_num: i32,
    ) -> Result<Season, DatabaseError> {
        use crate::schema::season::dsl::*;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show
            .find(tv_id)
            .get_result_async::<TVShow>(conn)
            .await?;

        Ok(season
            .filter(tvshowid.eq(tv_show.id))
            .filter(season_number.eq(season_num))
            .first_async::<Self>(conn)
            .await?)
    }

    /// Method deletes a season entry that belongs to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - season number we'd like to fetch.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::{Season, InsertableSeason};
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
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let season = Season::get(&conn, show_id, new_season.season_number).unwrap();
    ///
    /// assert_eq!(season.id, season_id);
    /// assert_eq!(season.season_number, 1);
    /// assert_eq!(season.added, Some("test".to_string()));
    /// assert_eq!(season.poster, Some("test".to_string()));
    ///
    /// let deleted = Season::delete(&conn, show_id, new_season.season_number).unwrap();
    /// assert_eq!(deleted, 1usize);
    ///
    /// let season = Season::get(&conn, show_id, new_season.season_number);
    /// assert!(season.is_err());
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn delete(
        conn: &crate::DbConnection,
        tv_id: i32,
        season_num: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::season::dsl::*;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show
            .find(tv_id)
            .get_result_async::<TVShow>(conn)
            .await?;

        let entry = season
            .filter(tvshowid.eq(tv_show.id))
            .filter(season_number.eq(season_num));

        let result = diesel::delete(entry).execute_async(conn).await?;
        Ok(result)
    }

    pub async fn get_first(
        conn: &crate::DbConnection,
        media: &TVShow,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::season::dsl::*;

        Ok(season
            .filter(tvshowid.eq(media.id))
            .order(season_number.asc())
            .first_async::<Self>(conn)
            .await?)
    }

    pub async fn get_by_id(
        conn: &crate::DbConnection,
        season_id: i32,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::season::dsl::*;

        Ok(season
            .filter(id.eq(season_id))
            .first_async::<Self>(conn)
            .await?)
    }
}

/// Struct representing a insertable season
/// Its exactly the same as [`Season`](Season) except it misses the tvshowid field and the id
/// field.
#[derive(Clone, Insertable, Serialize, Deserialize)]
#[table_name = "season"]
pub struct InsertableSeason {
    pub season_number: i32,
    pub added: String,
    pub poster: String,
}

impl InsertableSeason {
    /// Method inserts a new season and links it to a tv show based on the id specified.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of the tv show we'd like to discriminate against.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::{Season, InsertableSeason};
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
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let season = Season::get(&conn, show_id, new_season.season_number).unwrap();
    ///
    /// assert_eq!(season.id, season_id);
    /// assert_eq!(season.season_number, 1);
    /// assert_eq!(season.added, Some("test".to_string()));
    /// assert_eq!(season.poster, Some("test".to_string()));
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn insert(&self, conn: &crate::DbConnection, id: i32) -> Result<i32, DatabaseError> {
        use crate::schema::tv_show;

        // We check if the tv show exists
        // if it doesnt exist the ? operator would automatically
        // return Err(diesel::result::Error)
        let _ = tv_show::dsl::tv_show
            .find(id)
            .get_result_async::<TVShow>(conn)
            .await?;

        // We insert the tvshowid separately
        let query =
            diesel::insert_into(season::table).values((self.clone(), season::dsl::tvshowid.eq(id)));

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(query.returning(season::id)
                    .get_result_async(conn).await?)
            } else {
                query.execute_async(conn).await?;
                Ok(diesel::select(crate::last_insert_rowid).get_result_async(conn).await?)
            }
        }
    }
}

/// Struct used to update information about a season in the database.
/// All fields are updateable and optional except the primary key id
#[derive(Clone, AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name = "season"]
pub struct UpdateSeason {
    pub season_number: Option<i32>,
    pub tvshowid: Option<i32>,
    pub added: Option<String>,
    pub poster: Option<String>,
}

impl UpdateSeason {
    /// Method updates a seasons entry based on tv show id and season number.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - Season number we'd like to update.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::{Season, UpdateSeason, InsertableSeason};
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
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let season = Season::get(&conn, show_id, new_season.season_number).unwrap();
    ///
    /// assert_eq!(season.id, season_id);
    /// assert_eq!(season.season_number, 1);
    /// assert_eq!(season.added, Some("test".to_string()));
    /// assert_eq!(season.poster, Some("test".to_string()));
    ///
    /// let update_season = UpdateSeason {
    ///     season_number: None,
    ///     added: Some("test2".into()),
    ///     poster: None,
    ///     tvshowid: None,
    /// };
    ///
    /// let _ = update_season.update(&conn, show_id, 1);
    /// let season2 = Season::get(&conn, show_id, new_season.season_number).unwrap();
    ///
    /// assert_ne!(season.added, season2.added);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn update(
        self,
        conn: &crate::DbConnection,
        _id: i32,
        season_num: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::season::dsl::*;
        use crate::schema::tv_show;

        let tv = tv_show::dsl::tv_show
            .filter(id.eq(_id))
            .execute_async(conn)
            .await?;

        let entry = season
            .filter(tvshowid.eq(_id))
            .filter(season_number.eq(season_num));

        Ok(diesel::update(entry).set(self).execute_async(conn).await?)
    }
}
