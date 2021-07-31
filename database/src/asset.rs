use crate::DatabaseError;

#[derive(Debug, Clone, Default)]
pub struct Asset {
    pub id: i64,
    pub remote_url: Option<String>,
    pub local_path: String,
    pub file_ext: String,
}

impl Asset {
    pub async fn get_by_id(conn: &crate::DbConnection, id: i64) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(Asset,
                "SELECT * FROM assets WHERE id = ?",
                id).fetch_one(conn).await?)
    }

    pub async fn get_of_user(conn: &crate::DbConnection, username: &str) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(Asset,
                r#"SELECT assets.* FROM assets
                INNER JOIN users ON users.picture = assets.id
                WHERE users.username = ?"#,
                username).fetch_one(conn).await?)
    }

    pub async fn into_media_poster(
        &self,
        conn: &crate::DbConnection,
        media_id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            r#"INSERT INTO media_posters (media_id, asset_id)
                VALUES ($1, $2) RETURNING id as "id!: i64""#,
            media_id,
            self.id
        )
        .fetch_one(conn)
        .await?
        .id)
    }

    pub async fn into_media_backdrop(
        &self,
        conn: &crate::DbConnection,
        media_id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            r#"INSERT INTO media_backdrops (media_id, asset_id)
                VALUES ($1, $2) RETURNING id as "id!: i64""#,
            media_id,
            self.id
        )
        .fetch_one(conn)
        .await?
        .id)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InsertableAsset {
    pub remote_url: Option<String>,
    pub local_path: String,
    pub file_ext: String,
}

impl InsertableAsset {
    pub async fn insert(self, conn: &crate::DbConnection) -> Result<Asset, DatabaseError> {
        Ok(sqlx::query_as_unchecked!(
            Asset,
            "INSERT INTO assets (remote_url, local_path, file_ext)
                VALUES ($1, $2, $3)
                RETURNING id, remote_url, local_path, file_ext",
            self.remote_url,
            self.local_path,
            self.file_ext
        )
        .fetch_one(conn)
        .await?)
    }
}
