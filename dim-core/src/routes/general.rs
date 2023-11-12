use crate::core::DbConnection;
use crate::errors;

use dim_database::user::User;
use serde::Serialize;

use dim_database::genre::*;

use warp::reply;

pub mod filters {
    use dim_database::DbConnection;

    use dim_database::user::User;
    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use crate::routes::global_filters::with_auth;

    use super::super::global_filters::with_state;
    use serde::Deserialize;

    pub fn search(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        #[derive(Deserialize)]
        struct SearchArgs {
            query: Option<String>,
            year: Option<i32>,
            library_id: Option<i32>,
            genre: Option<String>,
            quick: Option<bool>,
        }

        warp::path!("api" / "v1" / "search")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and(warp::query::query::<SearchArgs>())
            .and_then(
                |auth: User, conn: DbConnection, args: SearchArgs| async move {
                    super::search(
                        conn,
                        args.query,
                        args.year,
                        args.library_id,
                        args.genre,
                        args.quick,
                        auth,
                    )
                    .await
                    .map_err(|e| reject::custom(e))
                },
            )
    }
}

pub async fn search(
    conn: DbConnection,
    query: Option<String>,
    year: Option<i32>,
    _library_id: Option<i32>,
    genre: Option<String>,
    _quick: Option<bool>,
    _user: User,
) -> Result<warp::reply::Json, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    if let Some(query_string) = query {
        let query_string = query_string
            .split(' ')
            .map(|x| format!("%{}%", x))
            .collect::<Vec<_>>()
            .as_slice()
            .join(" ");

        return search_by_name(&mut tx, &query_string, 15).await;
    }

    if let Some(x) = genre {
        let genre_id = Genre::get_by_name(&mut tx, x).await?.id;
        return search_by_genre(&mut tx, genre_id).await;
    }

    if let Some(x) = year {
        return search_by_release_year(&mut tx, x as i64).await;
    }

    Err(errors::DimError::NotFoundError)
}

async fn search_by_name(
    conn: &mut dim_database::Transaction<'_>,
    query: &str,
    limit: i64,
) -> Result<warp::reply::Json, errors::DimError> {
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
    .fetch_all(conn)
    .await
    .map_err(|_| errors::DimError::NotFoundError)?;

    Ok(reply::json(&data))
}

async fn search_by_genre(
    conn: &mut dim_database::Transaction<'_>,
    genre_id: i64,
) -> Result<warp::reply::Json, errors::DimError> {
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
    .fetch_all(conn)
    .await
    .map_err(|_| errors::DimError::NotFoundError)?;

    Ok(reply::json(&data))
}

async fn search_by_release_year(
    conn: &mut dim_database::Transaction<'_>,
    year: i64,
) -> Result<warp::reply::Json, errors::DimError> {
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
    .fetch_all(conn)
    .await
    .map_err(|_| errors::DimError::NotFoundError)?;

    Ok(warp::reply::json(&data))
}
