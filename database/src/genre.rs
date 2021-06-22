use crate::DatabaseError;

use serde::Deserialize;
use serde::Serialize;

/// Struct shows a single genre entry
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Genre {
    pub id: i64,
    /// Genre name, ie "Action"
    pub name: String,
}

/// Intermediary table showing the relationship between a media and a genre
#[derive(Clone, Debug, PartialEq)]
pub struct GenreMedia {
    pub id: i64,
    pub genre_id: i64,
    pub media_id: i64,
}

impl Genre {
    /*
    /// Method returns the entry of a genre if exists based on its name.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `query` - genre name
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
    pub async fn get_by_media(
        conn: &crate::DbConnection,
        query: i64,
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
    pub async fn get_by_media_and_genre(
        conn: &crate::DbConnection,
        genre_id: i64,
        media_id: i64,
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
    pub async fn delete(conn: &crate::DbConnection, genre_id: i64) -> Result<usize, DatabaseError> {
        use crate::schema::genre::dsl::*;

        Ok(diesel::delete(genre.filter(id.eq(genre_id)))
            .execute_async(conn)
            .await?)
    }
    */
}

/// Genre entry that can be inserted into the db.
#[derive(Clone)]
pub struct InsertableGenre {
    /// Genre name
    pub name: String,
}

impl InsertableGenre {
    /// Method inserts a new genre into the table otherwise returns the id of a existing entry
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i64, DatabaseError> {
        let tx = conn.begin().await.unwrap();
        todo!()
        /*
        use crate::schema::genre::dsl::*;

        Ok(retry_while!(DatabaseErrorKind::SerializationFailure, {
            conn
            .transaction::<_, _>(|conn| {
                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        let _ = diesel::sql_query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
                            .execute(conn);

                        let entry = genre.filter(name.ilike(self.name.clone())).first::<Genre>(conn);
                    } else {
                        let entry = genre
                            .filter(crate::upper(name).like(self.name.clone().to_uppercase()))
                            .first::<Genre>(conn);
                    }
                }

                if let Ok(x) = entry {
                    return Ok(x.id);
                }

                let query = diesel::insert_into(genre).values(self.clone());

                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        Ok(query.returning(id)
                            .get_result(conn)?)
                    } else {
                        query.execute(conn)?;
                        Ok(diesel::select(crate::last_insert_rowid).get_result(conn)?)
                    }
                }
            })
            .await
        })?)
        */
    }
}

/*
/// Struct which is used to pair a genre to a media
#[derive(Clone)]
pub struct InsertableGenreMedia {
    pub genre_id: i64,
    pub media_id: i64,
}

impl InsertableGenreMedia {
    /// Method inserts a new entry into the intermediary genre table linking a genre to a media
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
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
    pub async fn insert_pair(_genre_id: i64, _media_id: i64, conn: &crate::DbConnection) {
        use crate::schema::genre_media::dsl::*;

        let _ = retry_while!(DatabaseErrorKind::SerializationFailure, {
            conn.transaction::<_, _>(|conn| {
                cfg_if! {
                    if #[cfg(feature = "postgres")] {
                        let _ = diesel::sql_query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
                            .execute(conn);
                    }
                }

                if genre::table
                    .inner_join(genre_media)
                    .filter(media_id.eq(_media_id))
                    .filter(genre_id.eq(_genre_id))
                    .select(genre::dsl::id)
                    .first::<i64>(conn)
                    .is_ok()
                {
                    return Ok(());
                }

                let pair = Self {
                    genre_id: _genre_id,
                    media_id: _media_id,
                };
                diesel::insert_into(genre_media)
                    .values(pair)
                    .execute(conn)?;
                Ok(())
            })
            .await
        });
    }
}
*/
