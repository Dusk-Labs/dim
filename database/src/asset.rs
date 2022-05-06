use crate::user::UserID;
use crate::DatabaseError;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Asset {
    pub id: i64,
    pub remote_url: Option<String>,
    pub local_path: String,
    pub file_ext: String,
}

impl Asset {
    pub async fn get_by_id(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(
            sqlx::query_as!(Asset, "SELECT * FROM assets WHERE id = ?", id)
                .fetch_one(&mut *conn)
                .await?,
        )
    }

    pub async fn get_of_user(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
            Asset,
            r#"SELECT assets.* FROM assets
                INNER JOIN users ON users.picture = assets.id
                WHERE users.id = ?"#,
            uid
        )
        .fetch_one(&mut *conn)
        .await?)
    }

    pub async fn into_media_poster(
        &self,
        conn: &mut crate::Transaction<'_>,
        media_id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            r#"INSERT INTO media_posters (media_id, asset_id)
                VALUES ($1, $2) RETURNING id as "id!: i64""#,
            media_id,
            self.id
        )
        .fetch_one(&mut *conn)
        .await?
        .id)
    }

    pub async fn into_media_backdrop(
        &self,
        conn: &mut crate::Transaction<'_>,
        media_id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            r#"INSERT INTO media_backdrops (media_id, asset_id)
                VALUES ($1, $2) RETURNING id as "id!: i64""#,
            media_id,
            self.id
        )
        .fetch_one(&mut *conn)
        .await?
        .id)
    }

    pub async fn get_url_by_file(
        conn: &mut crate::Transaction<'_>,
        path: &PathBuf,
    ) -> Result<String, DatabaseError> {
        let cleaned_path: &str = &path.to_string_lossy();
        Ok(sqlx::query!(
            r#"SELECT remote_url as "remote_url!" FROM assets WHERE Local_path = ?"#,
            cleaned_path
        )
        .fetch_one(&mut *conn)
        .await?
        .remote_url)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InsertableAsset {
    pub remote_url: Option<String>,
    pub local_path: String,
    pub file_ext: String,
}

impl InsertableAsset {
    pub async fn insert(self, conn: &mut crate::Transaction<'_>) -> Result<Asset, DatabaseError> {
        let local_path = self.local_path.clone();

        if let Ok(x) = sqlx::query_as_unchecked!(
            Asset,
            "SELECT * FROM assets WHERE local_path = ?",
            local_path
        )
        .fetch_one(&mut *conn)
        .await
        {
            return Ok(x);
        }

        sqlx::query_as_unchecked!(
            Asset,
            "INSERT OR IGNORE INTO assets (remote_url, local_path, file_ext)
                VALUES ($1, $2, $3)",
            self.remote_url,
            self.local_path,
            self.file_ext
        )
        .execute(&mut *conn)
        .await?;

        // NOTE: asset is guaranteed to be in the table if we get here

        let result = sqlx::query_as_unchecked!(
            Asset,
            "SELECT * FROM assets WHERE local_path = ?",
            local_path
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result)
    }
}
