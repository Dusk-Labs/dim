use crate::episode::Episode;
use crate::library::MediaType;
use crate::media::Media;
use crate::schema::progress;
use crate::user::User;
use crate::DatabaseError as DieselError;

use diesel::prelude::*;
use futures::stream::iter;
use futures::StreamExt;
use tokio_diesel::*;

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
    pub async fn set(
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

        if diesel::update(
            progress::table
                .filter(progress::media_id.eq(mid))
                .filter(progress::user_id.eq(uid.clone())),
        )
        .set((
            progress::delta.eq(delta),
            progress::populated.eq(timestamp as i32),
        ))
        .execute_async(conn)
        .await?
            == 0
        {
            Ok(diesel::insert_into(progress::table)
                .values((
                    progress::delta.eq(delta),
                    progress::media_id.eq(mid),
                    progress::user_id.eq(uid),
                    progress::populated.eq(timestamp as i32),
                ))
                .execute_async(conn)
                .await?)
        } else {
            Ok(1)
        }
    }

    pub async fn get_for_media_user(
        conn: &crate::DbConnection,
        uid: String,
        mid: i32,
    ) -> Result<Self, DieselError> {
        use crate::schema::progress::dsl::*;

        match progress
            .filter(media_id.eq(mid))
            .filter(user_id.eq(uid.clone()))
            .first_async::<Self>(conn)
            .await
        {
            Ok(x) => Ok(x),
            Err(tokio_diesel::AsyncError::Error(diesel::result::Error::NotFound)) => Ok(Self {
                id: 0,
                delta: 0,
                media_id: mid,
                user_id: uid,
                populated: 0,
            }),
            Err(e) => Err(DieselError::AsyncError(e)),
        }
    }

    pub async fn get_total_time_spent_watching(
        conn: &crate::DbConnection,
        uid: String,
    ) -> Result<i32, DieselError> {
        use crate::schema::progress::dsl::*;

        Ok(progress
            .filter(user_id.eq(uid))
            .select(delta)
            .load_async::<i32>(conn)
            .await?
            .iter()
            .sum())
    }

    pub async fn get_total_for_media(
        conn: &crate::DbConnection,
        media: &Media,
        uid: String,
    ) -> Result<i32, DieselError> {
        match media.media_type {
            Some(MediaType::Tv) => Ok(Self::get_total_for_tv(conn, uid, media).await?),
            _ => Ok(Self::get_for_media_user(conn, uid, media.id)
                .await
                .map(|x| x.delta)?),
        }
    }

    pub async fn get_total_for_tv(
        conn: &crate::DbConnection,
        uid: String,
        media: &Media,
    ) -> Result<i32, DieselError> {
        let episodes = Episode::get_all_of_tv(conn, media)
            .await?
            .iter()
            .map(|x| (uid.clone(), x.clone()))
            .collect::<Vec<_>>();

        Ok(iter(episodes)
            .filter_map(|(uid, x)| async move {
                Self::get_for_media_user(conn, uid, x.media.id).await.ok()
            })
            .map(|x| x.delta)
            .collect::<Vec<_>>()
            .await
            .iter()
            .sum())
    }

    pub async fn get_continue_watching(
        conn: &crate::DbConnection,
        uid: String,
        count: usize,
    ) -> Result<Vec<Media>, DieselError> {
        use crate::schema::episode;
        use crate::schema::progress::dsl::*;
        use crate::schema::season;
        use crate::schema::streamable_media;
        use crate::schema::*;

        use super::tv::TVShow;

            
        let result = progress
            .filter(populated.ne(0))
            .filter(user_id.eq(uid))
            .inner_join(
                media::dsl::media.inner_join(
                    streamable_media::dsl::streamable_media.inner_join(
                        episode::dsl::episode
                            .inner_join(season::dsl::season.inner_join(tv_show::dsl::tv_show)),
                    ),
                ),
            )
            .select((tv_show::id, populated));

        cfg_if::cfg_if! {
            if #[cfg(feature = "postgres")] {
                let result = result.distinct_on(tv_show::id);
            } else {
                let result = result.group_by(tv_show::id);
            }
        }
        
        let mut result = result
            .load_async::<(i32, i32)>(conn)
            .await?;

        result.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(iter(result)
            .filter_map(|show| async move { TVShow { id: show.0 }.upgrade(conn).await.ok() })
            .collect::<Vec<Media>>()
            .await)
    }
}
