use crate::DatabaseError;
use crate::Transaction;

use std::path::Path;
use std::path::PathBuf;

pub struct UnmatchedMediafile {
    pub id: i64,
    pub name: String,
    pub duration: Option<i64>,
    pub target_file: PathBuf,
}

impl UnmatchedMediafile {
    pub async fn all_for_library(
        tx: &mut Transaction<'_>,
        library_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        struct Record {
            id: i64,
            name: String,
            duration: Option<i64>,
            target_file: String,
        }

        Ok(sqlx::query_as!(
            Record,
            r#"SELECT id, raw_name as name, duration, target_file FROM mediafile
               WHERE library_id = ? AND media_id IS NULL"#,
            library_id
        )
        .fetch_all(tx)
        .await?
        .into_iter()
        .map(|Record { id, name, duration, target_file }| {
            Self {
                id,
                name,
                duration,
                target_file: Path::new(&target_file).to_path_buf()
            }
        })
        .collect())
    }
}
