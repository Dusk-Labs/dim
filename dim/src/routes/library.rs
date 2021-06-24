use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;
use crate::routes::construct_standard;
use crate::scanners;

use auth::Wrapper as Auth;

use database::library::InsertableLibrary;
use database::library::Library;
use database::mediafile::MediaFile;
use database::media::Media;

use events::Message;
use events::PushEventType;

use std::collections::HashMap;
use std::convert::Infallible;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use futures::stream;
use futures::StreamExt;

use slog::Logger;

use warp::http::StatusCode;
use warp::reply;
use warp::Filter;

use serde_json::Value;

pub fn library_routes(
    conn: DbConnection,
    logger: slog::Logger,
    event_tx: EventTx,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::library_get(conn.clone())
        .or(filters::library_post(
            conn.clone(),
            logger.clone(),
            event_tx.clone(),
        ))
        .or(filters::library_delete(conn.clone(), event_tx.clone()))
        .or(filters::library_get_self(conn.clone()))
        .or(filters::get_all_of_library(conn.clone()))
        .or(filters::get_all_unmatched_media(conn.clone()))
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use std::sync::Arc;
    use std::sync::Mutex;

    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use super::super::global_filters::with_db;

    use auth::Wrapper as Auth;

    use database::DbConnection;

    use super::super::global_filters::with_state;
    use super::*;

    use crate::core::EventTx;

    pub fn library_get(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library")
            .and(warp::get())
            .and(with_db(conn))
            .and(auth::with_auth())
            .and_then(super::library_get)
    }

    pub fn library_post(
        conn: DbConnection,
        logger: slog::Logger,
        event_tx: EventTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library")
            .and(warp::post())
            .and(warp::body::json::<InsertableLibrary>())
            .and(auth::with_auth())
            .and(with_state::<EventTx>(event_tx))
            .and(with_state::<slog::Logger>(logger))
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |new_library: InsertableLibrary,
                 user: Auth,
                 event_tx: EventTx,
                 logger: slog::Logger,
                 conn: DbConnection| async move {
                    super::library_post(conn, new_library, logger, event_tx, user)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn library_delete(
        conn: DbConnection,
        event_tx: EventTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library" / i64)
            .and(warp::delete())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and(with_state::<EventTx>(event_tx))
            .and_then(
                |id: i64, user: Auth, conn: DbConnection, event_tx: EventTx| async move {
                    super::library_delete(id, user, conn, event_tx)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn library_get_self(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library" / i64)
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, user: Auth, conn: DbConnection| async move {
                super::get_self(conn, id, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_all_of_library(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library" / i64 / "media")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, user: Auth, conn: DbConnection| async move {
                super::get_all_library(conn, id, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_all_unmatched_media(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library" / i64 / "unmatched")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, user: Auth, conn: DbConnection| async move {
                super::get_all_unmatched_media(conn, id, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

/// Method maps to `GET /api/v1/library` and returns a list of all libraries in te database.
/// This method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `_log` - logger
/// * `_user` - Authentication middleware
pub async fn library_get(conn: DbConnection, _user: Auth) -> Result<impl warp::Reply, Infallible> {
    Ok(reply::json(&{
        let mut x = Library::get_all(&conn).await;
        x.sort_by(|a, b| a.name.cmp(&b.name));
        x
    }))
}

/// Method maps to `POST /api/v1/library`, it adds a new library to the database, starts a new
/// scanner for it, then dispatches a event to all clients notifying them that a new library has
/// been created. This method can only be accessed by authenticated users. Method returns 200 OK
///
/// # Arguments
/// * `conn` - database connection
/// * `new_library` - new library information posted by client
/// * `log` - logger
/// * `_user` - Auth middleware
pub async fn library_post(
    conn: DbConnection,
    new_library: InsertableLibrary,
    log: Logger,
    event_tx: EventTx,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let id = new_library.insert(&conn).await?;
    let tx_clone = event_tx.clone();
    let log_clone = log.clone();

    tokio::spawn(async move {
        let _ = scanners::start(id, log_clone, tx_clone).await;
    });

    let media_type = new_library.media_type;
    let tx_clone = event_tx.clone();
    let log_clone = log.clone();

    tokio::spawn(async move {
        let watcher = scanners::scanner_daemon::FsWatcher::new(log_clone, id, media_type, tx_clone).await;

        watcher
            .start_daemon()
            .await
            .expect("Something went wrong with the fs-watcher");
    });

    let event = Message {
        id,
        event_type: PushEventType::EventNewLibrary,
    };

    let _ = event_tx.send(serde_json::to_string(&event).unwrap());

    Ok(StatusCode::CREATED)
}

/// Method mapped to `DELETE /api/v1/library/<id>` is used to delete a library from the database.
/// It deletes the database based on the parameter `id`, then dispatches a event notifying all
/// clients that the database with this id has been removed. Method can only be accessed by
/// authenticated users.
///
/// # Arguments:
/// * `conn` - database connection
/// * `id` - id of the library we want to delete
/// * `event_tx` - channel over which to dispatch events
/// * `_user` - Auth middleware
// NOTE: Should we only allow the owner to add/remove libraries?
pub async fn library_delete(
    id: i64,
    _user: Auth,
    conn: DbConnection,
    event_tx: EventTx,
) -> Result<impl warp::Reply, errors::DimError> {
    Media::delete_by_lib_id(&conn, id).await?;
    MediaFile::delete_by_lib_id(&conn, id).await?;
    Library::delete(&conn, id).await?;

    let event = Message {
        id,
        event_type: PushEventType::EventRemoveLibrary,
    };

    let _ = event_tx.send(serde_json::to_string(&event).unwrap());

    Ok(StatusCode::NO_CONTENT)
}

/// Method mapped to `GET /api/v1/library/<id>` returns info about the library with the supplied
/// id. Method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library we want info of
/// * `_user` - Auth middleware
pub async fn get_self(
    conn: DbConnection,
    id: i64,
    _user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(&Library::get_one(&conn, id).await?))
}

/// Method mapped to `GET /api/v1/library/<id>/media` returns all the movies/tv shows that belong
/// to the library with the id supplied. Method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library we want media of
/// * `_user` - Auth middleware
pub async fn get_all_library(
    conn: DbConnection,
    id: i64,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut result = HashMap::new();
    let lib = Library::get_one(&conn, id).await?;
    let mut data = Media::get_all(&conn, id).await?;

    data.sort_by(|a, b| a.name.cmp(&b.name));
    let out = stream::iter(data)
        .filter_map(|x| async { construct_standard(&conn, &x.into(), &user).await.ok() })
        .collect::<Vec<Value>>()
        .await;

    result.insert(lib.name, out);

    Ok(reply::json(&result))
}

/// Method mapped to `GET` /api/v1/library/<id>/unmatched` returns a list of all unmatched medias
/// to be displayed in the library pages.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library
/// * `_user` - auth middleware
// NOTE: construct_standard on a mediafile will yield buggy deltas
pub async fn get_all_unmatched_media(
    conn: DbConnection,
    id: i64,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut result = HashMap::new();

    let filtered = MediaFile::get_by_lib_null_media(&conn, id)
        .await?
        .into_iter()
        .map(|x| {
            let mut path = Path::new(&x.target_file).to_path_buf();
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            path.pop();

            let dir = path.file_name();
            let group = dir
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or(file_name);

            (group, x)
        })
        .collect::<Vec<_>>();

    stream::iter(filtered)
        .filter_map(|(k, v)| {
            let (k, v) = (k.clone(), v.clone());
            async {
                let (k, v) = (k, v);
                construct_standard(&conn, &v.into(), &user)
                    .await
                    .ok()
                    .and_then(|x| Some((k, x)))
            }
        })
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(k, v)| result.entry(k).or_insert(vec![]).push(v));

    Ok(reply::json(&result))
}
