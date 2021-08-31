use crate::library::MediaType;
use crate::media::Media;
use crate::DatabaseError as DieselError;

use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Progress {
    pub id: i64,
    pub delta: i64,
    pub media_id: i64,
    pub user_id: String,
    pub populated: i64,
}

impl Progress {
    pub async fn set(
        conn: &crate::DbConnection,
        delta: i64,
        uid: String,
        mid: i64,
    ) -> Result<usize, DieselError> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Ok(sqlx::query!(
            "INSERT OR REPLACE INTO progress (delta, media_id, user_id, populated)
            VALUES ($1, $2, $3, $4)",
            delta,
            mid,
            uid,
            timestamp
        )
        .execute(conn)
        .await?
        .rows_affected() as usize)
    }

    pub async fn get_for_media_user(
        conn: &crate::DbConnection,
        uid: String,
        mid: i64,
    ) -> Result<Self, DieselError> {
        Ok(sqlx::query_as!(
            Progress,
            "SELECT progress.* FROM progress
            WHERE user_id = ?
            AND media_id = ?",
            uid,
            mid
        )
        .fetch_optional(conn)
        .await?
        .unwrap_or(Self {
            media_id: mid,
            user_id: uid,
            ..Default::default()
        }))
    }

    pub async fn get_total_time_spent_watching(
        conn: &crate::DbConnection,
        uid: String,
    ) -> Result<i32, DieselError> {
        Ok(sqlx::query!(
            "SELECT COALESCE(SUM(progress.delta), 0) as total FROM progress
                WHERE progress.user_id = ?",
            uid
        )
        .fetch_one(conn)
        .await?
        .total
        .unwrap_or_default())
    }

    pub async fn get_total_for_media(
        conn: &crate::DbConnection,
        media: &Media,
        uid: String,
    ) -> Result<i64, DieselError> {
        match media.media_type {
            MediaType::Tv => Ok(Self::get_total_for_tv(conn, uid, media.id).await? as i64),
            _ => Ok(Self::get_for_media_user(conn, uid, media.id)
                .await
                .map(|x| x.delta)?),
        }
    }

    pub async fn get_total_for_tv(
        conn: &crate::DbConnection,
        uid: String,
        tv_id: i64,
    ) -> Result<i32, DieselError> {
        Ok(sqlx::query!(
            "SELECT COALESCE(SUM(progress.delta), 0) as total FROM _tblmedia
            JOIN progress ON progress.media_id = _tblmedia.id
            JOIN episode ON episode.id = _tblmedia.id
            JOIN season on season.id = episode.seasonid
            JOIN tv_show ON tv_show.id = season.tvshowid
            
            WHERE tv_show.id = ?
            AND progress.user_id = ?",
            tv_id,
            uid
        )
        .fetch_one(conn)
        .await?
        .total)
    }

    pub async fn get_continue_watching(
        conn: &crate::DbConnection,
        uid: String,
        count: i64,
    ) -> Result<Vec<i64>, DieselError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT _tblmedia.id  FROM _tblmedia

            JOIN tv_show on tv_show.id = _tblmedia.id
            JOIN season on season.tvshowid = tv_show.id
            JOIN episode on episode.seasonid = season.id
            JOIN progress on progress.media_id = episode.id

            WHERE NOT progress.populated = 0
            AND progress.user_id = ?

            GROUP BY _tblmedia.id
            ORDER BY progress.populated DESC
            LIMIT ?"#,
            uid,
            count
        )
        .fetch_all(conn)
        .await?)
    }
}
