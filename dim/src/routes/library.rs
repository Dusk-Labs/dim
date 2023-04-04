use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;
use crate::external::tmdb::TMDBMetadataProvider;
use crate::json;
use crate::scanner;
use crate::scanner::daemon::FsWatcher;
use crate::tree;

use dim_database::compact_mediafile::CompactMediafile;
use dim_database::library::InsertableLibrary;
use dim_database::library::Library;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;

use dim_database::user::User;

use std::collections::HashMap;
use std::sync::Arc;

use warp::http::StatusCode;
use warp::reply;

use serde::Deserialize;
use serde::Serialize;

use tracing::error;
use tracing::info;
use tracing::instrument;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub mod filters {
    use warp::reject;
    use warp::Filter;

    use super::super::global_filters::with_auth;
    use super::super::global_filters::with_db;

    use dim_database::DbConnection;

    use super::super::global_filters::with_state;
    use super::*;

    use crate::core::EventTx;

    pub fn library_get(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library")
            .and(warp::get())
            .and(with_db(conn.clone()))
            .and(with_auth(conn))
            .and_then(|conn, auth| async move {
                super::library_get(conn, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn library_post(
        conn: DbConnection,
        event_tx: EventTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library")
            .and(warp::post())
            .and(warp::body::json::<InsertableLibrary>())
            .and(with_auth(conn.clone()))
            .and(with_state::<EventTx>(event_tx))
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |new_library: InsertableLibrary,
                 user: User,
                 event_tx: EventTx,
                 conn: DbConnection| async move {
                    super::library_post(conn, new_library, event_tx, user)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn library_delete(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library" / i64)
            .and(warp::delete())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, user: User, conn: DbConnection| async move {
                super::library_delete(id, user, conn)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn library_get_self(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "library" / i64)
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, user: User, conn: DbConnection| async move {
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
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|id: i64, user: User, conn: DbConnection| async move {
                super::get_all_library(conn, id, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_all_unmatched_media(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct Args {
            search: Option<String>,
        }

        warp::path!("api" / "v1" / "library" / i64 / "unmatched")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and(warp::filters::query::query::<Args>())
            .and_then(
                |id: i64, user: User, conn: DbConnection, Args { search }: Args| async move {
                    super::get_all_unmatched_media(conn, id, user, search)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }
}

/// # GET `/api/v1/library`
/// Method maps to `GET /api/v1/library` and returns a list of all libraries in te database.
///
/// # Request
/// An authenticated request must be made to this endpoint. No other inputs are required.
///
/// ## Example
/// ```text
/// curl -X GET http://127.0.0.1:8000/api/v1/library -H "Authroization: ...."
/// ```
///
/// # Response
/// This method will return `200 OK` as well as a JSON payload of the following format:
/// ```json
/// [
///   {
///     "id": number,
///     "name": string,
///     "media_type": "movie" | "tv",
///     "media_count": number,
///   },
///   ...
/// ]
/// ```
pub async fn library_get(
    conn: DbConnection,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;

    let mut libraries = Library::get_all(&mut tx).await;
    libraries.sort_by(|a, b| a.name.cmp(&b.name));

    let mut reply = vec![];

    for library in libraries.into_iter() {
        let library_size = Library::get_size(&mut tx, library.id).await?;

        reply.push(json!({
            "id": library.id,
            "name": library.name,
            "media_type": library.media_type,
            "media_count": library_size,
        }));
    }

    Ok(reply::json(&reply))
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
    mut conn: DbConnection,
    new_library: InsertableLibrary,
    event_tx: EventTx,
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    let id = new_library.insert(&mut tx).await?;
    tx.commit().await?;
    drop(lock);

    let tx_clone = event_tx.clone();

    let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");

    let provider = match new_library.media_type {
        MediaType::Movie => Arc::new(provider.movies()) as Arc<_>,
        MediaType::Tv => Arc::new(provider.tv_shows()) as Arc<_>,
        _ => unreachable!(),
    };

    let mut fs_watcher = FsWatcher::new(
        conn.clone(),
        id,
        new_library.media_type,
        tx_clone.clone(),
        Arc::clone(&provider),
    );

    tokio::spawn(async move { fs_watcher.start_daemon().await });
    tokio::spawn(async move { scanner::start(&mut conn, id, tx_clone, provider).await });

    Ok(reply::json(&json!({ "id": id })))
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
#[instrument(err, skip(conn, _user), fields(auth.user = _user.username.as_str()))]
pub async fn library_delete(
    id: i64,
    _user: User,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::DimError> {
    // First we mark the library as scheduled for deletion which will make the library and all its
    // content hidden. This is necessary because huge libraries take a long time to delete.
    {
        let mut lock = conn.writer().lock_owned().await;
        let mut tx = dim_database::write_tx(&mut lock).await?;
        if Library::mark_hidden(&mut tx, id).await? < 1 {
            return Err(errors::DimError::LibraryNotFound);
        }
        tx.commit().await?;
    }

    let delete_lib_fut = async move {
        let inner = async {
            let mut lock = conn.writer().lock_owned().await;
            let mut tx = dim_database::write_tx(&mut lock).await?;

            Library::delete(&mut tx, id).await?;
            Media::delete_by_lib_id(&mut tx, id).await?;
            MediaFile::delete_by_lib_id(&mut tx, id).await?;

            tx.commit().await?;

            Ok::<_, dim_database::error::DatabaseError>(())
        };

        if let Err(e) = inner.await {
            error!(reason = ?e, "Failed to delete library and its content.");
        } else {
            info!("Deleted library");
        }
    };

    tokio::spawn(delete_lib_fut);

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
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    Ok(reply::json(&Library::get_one(&mut tx, id).await?))
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
    _user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut result = HashMap::new();
    let mut tx = conn.read().begin().await?;
    let lib = Library::get_one(&mut tx, id).await?;

    #[derive(Serialize)]
    struct Record {
        id: i64,
        name: String,
        poster_path: Option<String>,
    }

    let mut data = sqlx::query_as!(
        Record,
        r#"SELECT _tblmedia.id, name, assets.local_path as poster_path FROM _tblmedia
        LEFT JOIN assets ON _tblmedia.poster = assets.id
        WHERE library_id = ? AND NOT media_type = "episode""#,
        id
    )
    .fetch_all(&mut tx)
    .await
    .map_err(|_| errors::DimError::NotFoundError)?;

    data.sort_by(|a, b| a.name.cmp(&b.name));

    result.insert(lib.name, data);

    Ok(reply::json(&result))
}

/// Method mapped to `GET` /api/v1/library/<id>/unmatched` returns a list of all unmatched medias
/// to be displayed in the library pages.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library
/// * `_user` - auth middleware
/// * `search` - query to fuzzy match against
// NOTE: construct_standard on a mediafile will yield buggy deltas
pub async fn get_all_unmatched_media(
    conn: DbConnection,
    id: i64,
    _user: User,
    search: Option<String>,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;

    let mut files = CompactMediafile::unmatched_for_library(&mut tx, id)
        .await
        .map_err(|_| errors::DimError::NotFoundError)?;

    // we want to pre-sort to ensure our tree is somewhat ordered.
    files.sort_by(|a, b| a.target_file.cmp(&b.target_file));

    if let Some(search) = search {
        let matcher = SkimMatcherV2::default();

        let mut matched_files = files
            .into_iter()
            .filter_map(|x| {
                let file_string = x.target_file.to_string_lossy();

                matcher
                    .fuzzy_match(&file_string, &search)
                    .map(|score| (x, score))
            })
            .collect::<Vec<_>>();

        matched_files.sort_by(|(_, a), (_, b)| b.cmp(&a));

        files = matched_files.into_iter().map(|(file, _)| file).collect();
    }

    let count = files.len();

    #[derive(Serialize)]
    struct Record {
        id: i64,
        name: String,
        duration: Option<i64>,
        file: String,
    }

    let entry = tree::Entry::build_with(
        files,
        |x| {
            x.target_file
                .iter()
                .map(|x| x.to_string_lossy().to_string())
                .collect()
        },
        |k, v| Record {
            id: v.id,
            name: v.name,
            duration: v.duration,
            file: k.to_string(),
        },
    );

    #[derive(Serialize)]
    struct Response {
        count: usize,
        files: Vec<tree::Entry<Record>>,
    }

    let entries = match entry {
        tree::Entry::Directory { files, .. } => files,
        _ => unreachable!(),
    };

    Ok(reply::json(&Response {
        files: entries,
        count,
    }))
}
