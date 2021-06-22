use std::convert::{TryFrom, TryInto};

use crate::opt_update;
use crate::tv::TVShow;
use crate::DatabaseError;

use serde::{Deserialize, Serialize};

/// Struct represents a season entry in the database.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Season {
    pub id: i64,
    /// Season number
    pub season_number: i64,
    /// Foreign key to the tv show we'd like to link against
    pub tvshowid: i64,
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
    pub async fn get_all(
        conn: &crate::DbConnection,
        tv_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id , season_number ,
                    tvshowid , added, poster FROM season WHERE id = ?"#,
            tv_id
        )
        .fetch_all(conn)
        .await?)
    }

    /// Method returns the season based on the season number belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - season number we'd like to fetch.
    ///
    pub async fn get(
        conn: &crate::DbConnection,
        tv_id: i64,
        season_num: i64,
    ) -> Result<Season, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id , season_number ,
                    tvshowid , added, poster FROM season WHERE id = ? AND season_number = ?"#,
            tv_id,
            season_num
        )
        .fetch_one(conn)
        .await?)
    }

    /// Method deletes a season entry that belongs to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - season number we'd like to fetch.
    ///
    pub async fn delete(
        conn: &crate::DbConnection,
        tv_id: i64,
        season_num: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!(
            "DELETE FROM season where tvshowid = ? AND season_number = ?",
            tv_id,
            season_num
        )
        .execute(conn)
        .await?
        .last_insert_rowid() as usize)
    }

    pub async fn get_first(
        conn: &crate::DbConnection,
        media_id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id , season_number ,
                    tvshowid , added, poster FROM season WHERE id = ?
                    ORDER BY season_number ASC"#,
            media_id,
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn get_by_id(
        conn: &crate::DbConnection,
        season_id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id , season_number ,
                    tvshowid , added, poster FROM season WHERE season_number = ?
                    ORDER BY season_number ASC"#,
            season_id,
        )
        .fetch_one(conn)
        .await?)
    }
}

/// Struct representing a insertable season
/// Its exactly the same as [`Season`](Season) except it misses the tvshowid field and the id
/// field.
#[derive(Clone, Serialize, Deserialize)]
pub struct InsertableSeason {
    pub season_number: i64,
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
    pub async fn insert(&self, conn: &crate::DbConnection, id: i64) -> Result<i64, DatabaseError> {
        sqlx::query!("SELECT * FROM tv_show WHERE id = ?", id).fetch_one(conn).await?;

        Ok({
            let result = sqlx::query!(
                "SELECT id FROM season WHERE season_number = ? AND tvshowid = ?",
                id,
                self.season_number
            )
            .fetch_optional(conn)
            .await?;

            if let Some(season) = result {
                return Ok(season.id.try_into().unwrap());
            }

            sqlx::query!("INSERT INTO season (season_number, added, poster, tvshowid) VALUES ($1, $2, $3, $4)", self.season_number, self.added, self.poster, id).execute(conn).await?.last_insert_rowid()
        })
    }
}

/// Struct used to update information about a season in the database.
/// All fields are updateable and optional except the primary key id
#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct UpdateSeason {
    pub season_number: Option<i64>,
    pub tvshowid: Option<i64>,
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
    pub async fn update(
        self,
        conn: &crate::DbConnection,
        tv_id: i64,
        season_num: i64,
    ) -> Result<usize, DatabaseError> {
        let tx = conn.begin().await?;

        let row = sqlx::query!(
            "SELECT season.id FROM season 
            INNER JOIN tv_show WHERE tv_show.id = ? 
            AND season.season_number = ?",
            tv_id,
            season_num
        )
        .fetch_one(conn)
        .await?;


        opt_update!(conn, tx,
            "UPDATE season SET season_number = ? WHERE id = ?" => (self.season_number, row.id),
            "UPDATE season SET tvshowid = ? WHERE id = ?" => (self.tvshowid, row.id),
            "UPDATE season SET added = ? WHERE id = ?" => (self.added, row.id),
            "UPDATE season SET poster = ? WHERE id = ?" => (self.poster, row.id)
        );

        Ok(1)
    }
}
