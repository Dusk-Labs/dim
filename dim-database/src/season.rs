use crate::opt_update;
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
    /// Id of the asset pointing to the poster.
    pub poster: Option<String>,
}

impl Season {
    /// Method returns all of the seasons that are linked to a tv show based on a tvshow id
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    pub async fn get_all(
        conn: &mut crate::Transaction<'_>,
        tv_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id as "id!", season_number, tvshowid, added, poster as "poster?"
            FROM season
            WHERE tvshowid = ?
            ORDER BY season_number ASC"#,
            tv_id
        )
        .fetch_all(&mut **conn)
        .await?)
    }

    /// Method returns the season based on the season number belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - season number we'd like to fetch.
    pub async fn get(
        conn: &mut crate::Transaction<'_>,
        tv_id: i64,
        season_num: i64,
    ) -> Result<Season, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id as "id!", season_number ,
                    tvshowid , added, poster as "poster?"
               FROM season WHERE id = ? AND season_number = ?"#,
            tv_id,
            season_num
        )
        .fetch_one(&mut **conn)
        .await?)
    }

    /// Method deletes a season entry that belongs to a tv show.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `tv_id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - season number we'd like to fetch.
    pub async fn delete(
        conn: &mut crate::Transaction<'_>,
        tv_id: i64,
        season_num: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!(
            "DELETE FROM season where tvshowid = ? AND season_number = ?",
            tv_id,
            season_num
        )
        .execute(&mut **conn)
        .await?
        .rows_affected() as usize)
    }

    pub async fn delete_by_id(
        conn: &mut crate::Transaction<'_>,
        season_id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(
            sqlx::query!("DELETE FROM _tblseason where id = ?", season_id)
                .execute(&mut **conn)
                .await?
                .rows_affected() as usize,
        )
    }

    /// Method will return the oldest season for a tv show that is available.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `tv_id` - id of the tv show.
    pub async fn get_first(
        conn: &mut crate::Transaction<'_>,
        tv_id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id as "id!", season_number, tvshowid, added, poster as "poster?"
            FROM season
            WHERE tvshowid = ?
            ORDER BY season_number ASC"#,
            tv_id,
        )
        .fetch_one(&mut **conn)
        .await?)
    }

    pub async fn get_by_id(
        conn: &mut crate::Transaction<'_>,
        season_id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id, season_number, tvshowid, added, poster as "poster?"
            FROM season WHERE id = ?"#,
            season_id,
        )
        .fetch_one(&mut **conn)
        .await?)
    }

    pub async fn get_tvshowid(
        conn: &mut crate::Transaction<'_>,
        season_id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(
            sqlx::query!("SELECT tvshowid FROM season WHERE id = ?", season_id)
                .fetch_one(&mut **conn)
                .await?
                .tvshowid,
        )
    }

    pub async fn count_children(
        tx: &mut crate::Transaction<'_>,
        season_id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            "SELECT COUNT(episode.id) AS count FROM episode WHERE episode.seasonid = ?",
            season_id
        )
        .fetch_one(&mut **tx)
        .await?
        .count as _)
    }
}

/// Struct representing a insertable season
/// Its exactly the same as [`Season`] except it misses the tvshowid field and the id
/// field.
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct InsertableSeason {
    pub season_number: i64,
    pub added: String,
    pub poster: Option<i64>,
}

impl InsertableSeason {
    /// Method inserts a new season and links it to a tv show based on the id specified.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `id` - id of the tv show we'd like to link this season to
    pub async fn insert(
        &self,
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            r#"INSERT INTO _tblseason (season_number, added, poster, tvshowid)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO UPDATE
            SET poster = $3
            RETURNING id as "id!: i64""#,
            self.season_number,
            self.added,
            self.poster,
            id
        )
        .fetch_one(&mut **conn)
        .await?
        .id)
    }
}

/// Struct used to update information about a season in the database.
/// All fields are updateable and optional except the primary key id
#[derive(Clone, Default, Deserialize, PartialEq, Debug)]
pub struct UpdateSeason {
    pub season_number: Option<i64>,
    pub tvshowid: Option<i64>,
    pub added: Option<String>,
    pub poster: Option<i64>,
}

impl UpdateSeason {
    /// Method updates a seasons entry based on tv show id and season number.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `id` - id of the tv show we'd like to discriminate against.
    /// * `season_num` - Season number we'd like to update.
    pub async fn update(
        self,
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        opt_update!(conn,
            "UPDATE _tblseason SET season_number = $1 WHERE id = ?2" => (self.season_number, id),
            "UPDATE _tblseason SET tvshowid = $1 WHERE id = ?2" => (self.tvshowid, id),
            "UPDATE _tblseason SET added = $1 WHERE id = ?2" => (self.added, id),
            "UPDATE _tblseason SET poster = $1 WHERE id = ?2" => (self.poster, id)
        );

        Ok(1)
    }
}
