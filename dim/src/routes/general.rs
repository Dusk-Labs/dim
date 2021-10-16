use crate::core::DbConnection;
use crate::errors;

use auth::Wrapper as Auth;
use serde::Serialize;

use database::genre::*;

use tokio::task::spawn_blocking;

use std::fs;
use std::io;
use std::path::PathBuf;

use warp::reply;

pub mod filters {
    use database::DbConnection;

    use auth::Wrapper as Auth;

    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use super::super::global_filters::with_state;
    use serde::Deserialize;

    pub fn get_directory_structure(
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "filebrowser" / ..)
            .and(warp::path::tail())
            .and(auth::with_auth())
            .and_then(|tail: warp::path::Tail, user: Auth| async move {
                super::get_directory_structure(tail.as_str().into(), user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

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
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and(warp::query::query::<SearchArgs>())
            .and_then(
                |auth: Auth, conn: DbConnection, args: SearchArgs| async move {
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

pub fn enumerate_directory<T: AsRef<std::path::Path>>(path: T) -> io::Result<Vec<String>> {
    let mut dirs: Vec<String> = fs::read_dir(path)?
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| {
            !x.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
                && !x.path().is_file()
        })
        .map(|x| {
            let path = x.path().to_string_lossy().to_string().replace("\\", "/");
            if cfg!(windows) {
                path.replace("C:", "")
            } else {
                path
            }
        })
        .collect::<Vec<_>>();

    dirs.sort();
    Ok(dirs)
}

pub async fn get_directory_structure(
    path: PathBuf,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let path_prefix = "C:/";
        } else {
            let path_prefix = "/";
        }
    }

    let path = if path.starts_with(path_prefix) {
        path
    } else {
        let mut new_path = PathBuf::new();
        new_path.push(path_prefix);
        new_path.push(path);
        new_path
    };

    Ok(reply::json(
        &spawn_blocking(|| enumerate_directory(path))
            .await
            .unwrap()?,
    ))
}

pub async fn search(
    conn: DbConnection,
    query: Option<String>,
    year: Option<i32>,
    _library_id: Option<i32>,
    genre: Option<String>,
    _quick: Option<bool>,
    _user: Auth,
) -> Result<warp::reply::Json, errors::DimError> {
    if let Some(query_string) = query {
        let query_string = query_string
            .split(' ')
            .map(|x| format!("%{}%", x))
            .collect::<Vec<_>>()
            .as_slice()
            .join(" ");

        return search_by_name(&conn, &query_string, 15).await;
    }

    if let Some(x) = genre {
        let genre_id = Genre::get_by_name(&conn, x).await?.id;
        return search_by_genre(&conn, genre_id).await;
    }

    if let Some(x) = year {
        return search_by_release_year(&conn, x as i64).await;
    }

    Err(errors::DimError::NotFoundError)
}

async fn search_by_name(
    conn: &DbConnection,
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
    conn: &DbConnection,
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
    conn: &DbConnection,
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
