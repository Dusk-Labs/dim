use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::{media::Media, user::User};
use serde::{ Serialize};
use crate::schema::progress;

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
    pub fn set(conn: &diesel::PgConnection, delta: i32, uid: String, mid: i32) -> Result<usize, DieselError> {
        diesel::insert_into(progress::table)
            .values((progress::delta.eq(delta), progress::media_id.eq(mid), progress::user_id.eq(uid)))
            .on_conflict((progress::media_id, progress::user_id))
            .do_update()
            .set(progress::delta.eq(delta))
            .execute(conn)
    }

    pub fn get_for_media_user(conn: &diesel::PgConnection, uid: String, mid: i32) -> Result<i32, DieselError> {
        use crate::schema::progress::dsl::*;

        match progress.filter(media_id.eq(mid)).filter(user_id.eq(uid)).select(delta).first::<i32>(conn) {
            Ok(x) => Ok(x),
            Err(DieselError::NotFound) => Ok(0),
            Err(e) => Err(e)
        }
    }
}
