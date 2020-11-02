use crate::schema::progress;
use crate::{media::Media, user::User};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Queryable, Debug, Identifiable, Associations, Serialize)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Media, foreign_key = "media_id")]
#[table_name = "progress"]
pub struct Progress {
    pub id: i32,
    pub delta: i32,
    pub media_id: i32,
    pub user_id: String,
}

impl Progress {
    pub fn set(
        conn: &crate::DbConnection,
        delta: i32,
        uid: String,
        mid: i32,
    ) -> Result<usize, DieselError> {
        // NOTE: We could use `on_conflict` here but the diesel backend for sqlite doesnt support
        // this yet.
        if diesel::insert_into(progress::table)
            .values((
                progress::delta.eq(delta),
                progress::media_id.eq(mid),
                progress::user_id.eq(uid.clone()),
            ))
            .execute(conn)
            .is_err()
        {
            diesel::update(
                progress::table
                    .filter(progress::media_id.eq(mid))
                    .filter(progress::user_id.eq(uid)),
            )
            .set(progress::delta.eq(delta))
            .execute(conn)
        } else {
            Ok(1)
        }
    }

    pub fn get_for_media_user(
        conn: &crate::DbConnection,
        uid: String,
        mid: i32,
    ) -> Result<i32, DieselError> {
        use crate::schema::progress::dsl::*;

        match progress
            .filter(media_id.eq(mid))
            .filter(user_id.eq(uid))
            .select(delta)
            .first::<i32>(conn)
        {
            Ok(x) => Ok(x),
            Err(DieselError::NotFound) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub fn get_total_time_spent_watching(
        conn: &crate::DbConnection,
        uid: String,
    ) -> Result<i32, DieselError> {
        use crate::schema::progress::dsl::*;

        Ok(progress
            .filter(user_id.eq(uid))
            .select(delta)
            .load::<i32>(conn)?
            .iter()
            .sum())
    }
}
