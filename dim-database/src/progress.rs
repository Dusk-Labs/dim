use crate::library::MediaType;
use crate::media::Media;
use crate::user::UserID;
use crate::DatabaseError as DieselError;

use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Serialize)]
pub struct Progress {
    pub id: i64,
    pub delta: i64,
    pub media_id: i64,
    pub user_id: UserID,
    pub populated: i64,
}

impl Progress {
    pub async fn set(
        conn: &mut crate::Transaction<'_>,
        delta: i64,
        uid: UserID,
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
        .execute(&mut *conn)
        .await?
        .rows_affected() as usize)
    }

    pub async fn get_for_media_user(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
        mid: i64,
    ) -> Result<Self, DieselError> {
        Ok(sqlx::query_as!(
            Progress,
            r#"SELECT id, user_id as "user_id: UserID", delta, media_id, populated FROM progress
            WHERE user_id = ?
            AND media_id = ?"#,
            uid,
            mid
        )
        .fetch_optional(&mut *conn)
        .await?
        .unwrap_or(Self {
            id: Default::default(),
            media_id: mid,
            user_id: uid,
            delta: Default::default(),
            populated: Default::default(),
        }))
    }

    pub async fn get_total_time_spent_watching(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
    ) -> Result<i32, DieselError> {
        Ok(sqlx::query!(
            "SELECT COALESCE(SUM(progress.delta), 0) as total FROM progress
                WHERE progress.user_id = ?",
            uid
        )
        .fetch_one(&mut *conn)
        .await?
        .total
        .unwrap_or_default())
    }

    pub async fn get_total_for_media(
        conn: &mut crate::Transaction<'_>,
        media: &Media,
        uid: UserID,
    ) -> Result<i64, DieselError> {
        match media.media_type {
            MediaType::Tv => Ok(Self::get_total_for_tv(conn, uid, media.id).await? as i64),
            _ => Ok(Self::get_for_media_user(conn, uid, media.id)
                .await
                .map(|x| x.delta)?),
        }
    }

    pub async fn get_progress_for_media(
        conn: &mut crate::Transaction<'_>,
        id: i64,
        uid: UserID,
    ) -> Result<(i64, i64), DieselError> {
        #[derive(sqlx::FromRow)]
        struct Record {
            delta: i64,
            duration: i64,
        }

        // FIXME: Use query_as macro instead of query_as function when https://github.com/launchbadge/sqlx/issues/1249 is fixed.
        let record = sqlx::query_as::<_, Record>(
            "SELECT progress.delta, MAX(mediafile.duration) duration FROM _tblmedia
            INNER JOIN mediafile ON mediafile.media_id = _tblmedia.id
            LEFT OUTER JOIN progress ON progress.media_id = _tblmedia.id AND progress.user_id = ?
            WHERE _tblmedia.id = ?
            GROUP BY _tblmedia.id
            LIMIT 1",
        )
        .bind(uid)
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;

        Ok((record.delta, record.duration))
    }

    pub async fn get_total_for_tv(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
        tv_id: i64,
    ) -> Result<i32, DieselError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            total: i32,
        }

        Ok(sqlx::query_as::<_, Row>(
            "SELECT COALESCE(SUM(progress.delta), 0) as total FROM _tblmedia
            JOIN progress ON progress.media_id = _tblmedia.id
            JOIN episode ON episode.id = _tblmedia.id
            JOIN season on season.id = episode.seasonid
            
            WHERE season.tvshowid = ?
            AND progress.user_id = ?",
        )
        .bind(tv_id)
        .bind(uid)
        .fetch_one(&mut *conn)
        .await?
        .total)
    }

    pub async fn get_continue_watching(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
        count: i64,
    ) -> Result<Vec<i64>, DieselError> {
        Ok(sqlx::query_scalar(
            r#"SELECT _tblmedia.id  FROM _tblmedia

            JOIN season on season.tvshowid = _tblmedia.id
            JOIN episode on episode.seasonid = season.id
            JOIN progress on progress.media_id = episode.id
            JOIN library on library.id = _tblmedia.library_id

            WHERE NOT progress.populated = 0
            AND progress.user_id = ?
            AND NOT library.hidden

            GROUP BY _tblmedia.id
            ORDER BY progress.populated DESC
            LIMIT ?"#,
        )
        .bind(uid)
        .bind(count)
        .fetch_all(&mut *conn)
        .await?)
    }
}
