use crate::AppState;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::response::Response;

use dim_database::genre::*;
use dim_database::DatabaseError;

use http::StatusCode;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum AuthError {
    /// Not Found.
    NotFoundError,
    /// Not logged in.
    InvalidCredentials,
    /// database: {0}
    Database(#[from] DatabaseError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFoundError => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            Self::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}

#[derive(Deserialize)]
pub struct SearchArgs {
    query: Option<String>,
    year: Option<i32>,
    _library_id: Option<i32>,
    genre: Option<String>,
    _quick: Option<bool>,
}

pub async fn search(
    State(AppState { conn, .. }): State<AppState>,
    Query(search_args): Query<SearchArgs>,
) -> Result<impl IntoResponse, AuthError> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;

    if let Some(query_string) = search_args.query {
        let query_string = query_string
            .split(' ')
            .map(|x| format!("%{}%", x))
            .collect::<Vec<_>>()
            .as_slice()
            .join(" ");

        if let Ok(x) = search_by_name(&mut tx, &query_string, 15).await {
            return Ok(Json(x).into_response());
        }
    }

    if let Some(x) = search_args.genre {
        let genre_id = Genre::get_by_name(&mut tx, x).await?.id;
        if let Ok(x) = search_by_genre(&mut tx, genre_id).await {
            return Ok(Json(x).into_response());
        }
    }

    if let Some(x) = search_args.year {
        if let Ok(x) = search_by_release_year(&mut tx, x as i64).await {
            return Ok(Json(x).into_response());
        }
    }

    Err(AuthError::NotFoundError)
}

async fn search_by_name(
    conn: &mut dim_database::Transaction<'_>,
    query: &str,
    limit: i64,
) -> Result<Value, AuthError> {
    #[derive(Serialize)]
    struct Record {
        id: i64,
        library_id: i64,
        name: String,
        poster_path: Option<String>,
    }

    let data = sqlx::query_as!(
        Record,
        r#"SELECT _tblmedia.id, library_id, name, assets.local_path as poster_path FROM _tblmedia
           LEFT JOIN assets on _tblmedia.poster = assets.id
           WHERE NOT media_type = "episode"
           AND UPPER(name) LIKE ?
           LIMIT ?"#,
        query,
        limit
    )
    .fetch_all(&mut **conn)
    .await
    .map_err(DatabaseError::from)?;

    Ok(json!(&data))
}

async fn search_by_genre(
    conn: &mut dim_database::Transaction<'_>,
    genre_id: i64,
) -> Result<Value, AuthError> {
    #[derive(Serialize)]
    struct Record {
        id: i64,
        library_id: i64,
        name: String,
        poster_path: Option<String>,
    }

    let data = sqlx::query_as!(
        Record,
        r#"SELECT _tblmedia.id, library_id, name, assets.local_path as poster_path
                FROM _tblmedia
                LEFT JOIN assets on _tblmedia.poster = assets.id
                INNER JOIN genre_media ON genre_media.media_id = _tblmedia.id
                WHERE NOT media_type = "episode"
                AND genre_media.genre_id = ?
                "#,
        genre_id,
    )
    .fetch_all(&mut **conn)
    .await
    .map_err(DatabaseError::from)?;

    Ok(json!(&data))
}

async fn search_by_release_year(
    conn: &mut dim_database::Transaction<'_>,
    year: i64,
) -> Result<Value, AuthError> {
    #[derive(Serialize)]
    struct Record {
        id: i64,
        library_id: i64,
        name: String,
        poster_path: Option<String>,
    }

    let data = sqlx::query_as!(
        Record,
        r#"SELECT _tblmedia.id, library_id, name, assets.local_path as poster_path
                FROM _tblmedia
            LEFT JOIN assets on _tblmedia.poster = assets.id
                WHERE NOT media_type = "episode"
                AND year = ?
                "#,
        year,
    )
    .fetch_all(&mut **conn)
    .await
    .map_err(DatabaseError::from)?;

    Ok(json!(&data))
}
