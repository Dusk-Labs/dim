use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;
use crate::routes::construct_standard;
use crate::routes::construct_standard_quick;

use auth::Wrapper as Auth;
use cfg_if::cfg_if;

use database::episode::Episode;
use database::genre::*;
use database::library::MediaType;
use database::media::Media;
use database::mediafile::MediaFile;
use database::progress::Progress;
use database::season::Season;

use futures::stream;
use futures::StreamExt;

use tokio::task::spawn_blocking;

use std::fs;
use std::io;
use std::path::PathBuf;

use warp::reply;
use warp::Filter;
use warp::Rejection;

pub fn general_router(
    conn: DbConnection,
    logger: slog::Logger,
    event_tx: EventTx,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    filters::search(conn.clone())
        .or(filters::magnet(conn, logger, event_tx))
        .or(filters::get_directory_structure())
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use database::DbConnection;

    use auth::Wrapper as Auth;

    use database::library::Library;
    use warp::reject;
    use warp::Filter;
    use warp::Rejection;
    use warp::Reply;

    use crate::core::EventTx;
    use crate::errors::DimError;

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

    pub fn magnet(
        conn: DbConnection,
        logger: slog::Logger,
        event_tx: EventTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        #[derive(Deserialize)]
        struct MagnetMedia {
            link: String,
            lib: i64,
        }

        warp::path!("api" / "v1" / "magnet")
            .and(warp::post())
            .and(auth::with_auth())
            .and(warp::body::json::<MagnetMedia>())
            .and(with_state::<EventTx>(event_tx))
            .and(with_state::<slog::Logger>(logger))
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |user: Auth,
                 magnet: MagnetMedia,
                 event_tx: EventTx,
                 logger: slog::Logger,
                 conn: DbConnection| async move {
                    let library = Library::get_one(&conn, magnet.lib).await.map_err(|err| {
                        slog::error!(logger, "Database Error: {:?}", err);
                        reject::custom(DimError::DatabaseError)
                    })?;

                    tokio::process::Command::new("deluge-console")
                        .args(&[
                            "-L",
                            "debug",
                            "add",
                            "--path",
                            library.location.as_str(),
                            magnet.link.as_str(),
                        ])
                        .spawn()
                        .map_err(|_| reject::custom(DimError::IOError))?;

                    Ok::<_, Rejection>(warp::reply())
                },
            )
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
    library_id: Option<i32>,
    genre: Option<String>,
    quick: Option<bool>,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let quick = quick.unwrap_or(false);

    if let Some(query_string) = query {
        let query_string = query_string
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice()
            .join("% %");

        let mut items = Vec::new();
        for x in Media::get_search(&conn, &query_string, 15).await? {
            if quick {
                if let Ok(x) = construct_standard_quick(&x).await {
                    items.push(x);
                }
            } else {
                if let Ok(x) = construct_standard(&conn, &x, &user).await {
                    items.push(x);
                }
            }
        }

        return Ok(reply::json(&items));
    }

    if let Some(x) = genre {
        let genre_row = Genre::get_by_name(&conn, x).await?.id;
        let mut items = Vec::new();

        for x in Media::get_of_genre(&conn, genre_row).await? {
            if quick {
                if let Ok(x) = construct_standard_quick(&x).await {
                    items.push(x);
                }
            } else {
                if let Ok(x) = construct_standard(&conn, &x, &user).await {
                    items.push(x);
                }
            }
        }

        return Ok(reply::json(&items));
    }

    if let Some(x) = year {
        let mut items = Vec::new();

        for x in Media::get_of_year(&conn, x as i64).await? {
            if quick {
                if let Ok(x) = construct_standard_quick(&x).await {
                    items.push(x);
                }
            } else {
                if let Ok(x) = construct_standard(&conn, &x, &user).await {
                    items.push(x);
                }
            }
        }

        return Ok(reply::json(&items));
    }

    Err(errors::DimError::NotFoundError)
}
