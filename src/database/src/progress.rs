use crate::episode::Episode;
use crate::library::MediaType;
use crate::media::Media;
use crate::schema::progress;
use crate::user::User;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

use std::time::SystemTime;

#[derive(Queryable, Debug, Identifiable, Associations, Serialize)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Media, foreign_key = "media_id")]
#[table_name = "progress"]
pub struct Progress {
    pub id: i32,
    pub delta: i32,
    pub media_id: i32,
    pub user_id: String,
    pub populated: i32,
}

impl Progress {
    pub fn set(
        conn: &crate::DbConnection,
        delta: i32,
        uid: String,
        mid: i32,
    ) -> Result<usize, DieselError> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // NOTE: We could use `on_conflict` here but the diesel backend for sqlite doesnt support
        // this yet.
        if diesel::insert_into(progress::table)
            .values((
                progress::delta.eq(delta),
                progress::media_id.eq(mid),
                progress::user_id.eq(uid.clone()),
                progress::populated.eq(timestamp as i32),
            ))
            .execute(conn)
            .is_err()
        {
            diesel::update(
                progress::table
                    .filter(progress::media_id.eq(mid))
                    .filter(progress::user_id.eq(uid)),
            )
            .set((
                progress::delta.eq(delta),
                progress::populated.eq(timestamp as i32),
            ))
            .execute(conn)
        } else {
            Ok(1)
        }
    }

    pub fn get_for_media_user(
        conn: &crate::DbConnection,
        uid: String,
        mid: i32,
    ) -> Result<Self, DieselError> {
        use crate::schema::progress::dsl::*;

        match progress
            .filter(media_id.eq(mid))
            .filter(user_id.eq(uid.clone()))
            .first::<Self>(conn)
        {
            Ok(x) => Ok(x),
            Err(DieselError::NotFound) => Ok(Self {
                id: 0,
                delta: 0,
                media_id: mid,
                user_id: uid,
                populated: 0,
            }),
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

    pub fn get_total_for_media(
        conn: &crate::DbConnection,
        media: &Media,
        uid: String,
    ) -> Result<i32, DieselError> {
        match media.media_type {
            Some(MediaType::Tv) => Self::get_total_for_tv(conn, uid, media),
            _ => Self::get_for_media_user(conn, uid, media.id).map(|x| x.delta),
        }
    }

    pub fn get_total_for_tv(
        conn: &crate::DbConnection,
        uid: String,
        media: &Media,
    ) -> Result<i32, DieselError> {
        let episodes = Episode::get_all_of_tv(conn, media)?;

        Ok(episodes
            .iter()
            .filter_map(|x| Self::get_for_media_user(conn, uid.clone(), x.media.id).ok())
            .map(|x| x.delta)
            .sum())
    }
}
