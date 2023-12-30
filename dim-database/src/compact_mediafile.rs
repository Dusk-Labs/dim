use crate::DatabaseError;
use crate::Transaction;

use std::path::Path;
use std::path::PathBuf;

/// A compact version of `MediaFile`. Useful in cases where we need to request some basic info for
/// a lot of mediafiles, and as such a `SELECT *` is not viable.
#[derive(Clone)]
pub struct CompactMediafile {
    pub id: i64,
    pub name: String,
    pub duration: Option<i64>,
    pub target_file: PathBuf,
}

/// Intermediary record which is later converted into a `CompactMediafile`. This is needed because
/// sqlx doesnt support deserializing into a `PathBuf`.
struct Record {
    id: i64,
    name: String,
    duration: Option<i64>,
    target_file: String,
}

impl From<Record> for CompactMediafile {
    fn from(
        Record {
            id,
            name,
            duration,
            target_file,
        }: Record,
    ) -> Self {
        Self {
            id,
            name,
            duration,
            target_file: Path::new(&target_file).to_path_buf(),
        }
    }
}

impl CompactMediafile {
    /// Method will return all the unmatched mediafiles for a specific library.
    pub async fn unmatched_for_library(
        tx: &mut Transaction<'_>,
        library_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            Record,
            r#"SELECT id, raw_name as name, duration, target_file FROM mediafile
               WHERE library_id = ? AND media_id IS NULL"#,
            library_id
        )
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    /// Method will return all mediafiles for a media id.
    pub async fn all_for_media(
        tx: &mut Transaction<'_>,
        media_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            Record,
            r#"SELECT id, raw_name as name, duration, target_file FROM mediafile
               WHERE mediafile.media_id = ?"#,
            media_id
        )
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    /// Method will return all mediafiles for a tv show.
    pub async fn all_for_tv(
        tx: &mut Transaction<'_>,
        tv_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            Record,
            "SELECT mediafile.id, raw_name as name, duration, target_file FROM mediafile
             INNER JOIN episode ON mediafile.media_id = episode.id
             INNER JOIN _tblseason ON episode.seasonid = _tblseason.id
             WHERE _tblseason.tvshowid = ?
             GROUP BY episode.id
             ",
            tv_id
        )
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }
}
