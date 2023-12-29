#![warn(warnings)]

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;

use dim_core::errors::DimError;
use dim_core::scanner::daemon::FsWatcher;
use dim_database::compact_mediafile::CompactMediafile;
use dim_database::library::{InsertableLibrary, Library, MediaType};
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use dim_database::user::User;

use dim_extern_api::tmdb::TMDBMetadataProvider;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use http::StatusCode;
use serde::Deserialize;
use serde::Serialize;

use crate::error::DimErrorWrapper;
use crate::AppState;

/// Method maps to `POST /api/v1/library`, it adds a new library to the database, starts a new
/// scanner for it, then dispatches a event to all clients notifying them that a new library has
/// been created. This method can only be accessed by authenticated users. Method returns 200 OK
///
pub async fn library_post(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(new_library): Json<InsertableLibrary>,
) -> Response {
    if !user.has_role("owner") {
        return (
            StatusCode::UNAUTHORIZED,
            "User account is not allowed to add a library.".to_string(),
        )
            .into_response();
    }
    let mut lock = state.conn.writer().lock_owned().await;

    let mut tx = match dim_database::write_tx(&mut lock).await {
        Ok(tx) => tx,
        Err(err) => {
            tracing::error!(?err, "Error getting connection");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    let id = match new_library.insert(&mut tx).await {
        Ok(id) => id,
        Err(err) => {
            tracing::error!(?err, "Error inserting library");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    match tx.commit().await {
        Ok(_) => (),
        Err(err) => {
            tracing::error!(?err, "Error committing transaction");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    }
    drop(lock);

    let tx_clone = state.event_tx.clone();

    const TMDB_KEY: &str = "38c372f5bc572c8aadde7a802638534e";
    let provider = TMDBMetadataProvider::new(TMDB_KEY);

    let provider = match new_library.media_type {
        MediaType::Movie => Arc::new(provider.movies()) as Arc<_>,
        MediaType::Tv => Arc::new(provider.tv_shows()) as Arc<_>,
        _ => unreachable!(),
    };

    let mut fs_watcher = FsWatcher::new(
        state.conn.clone(),
        id,
        new_library.media_type,
        tx_clone.clone(),
        Arc::clone(&provider),
    );

    tokio::spawn(async move { fs_watcher.start_daemon().await });
    let mut conn = state.conn.clone();
    tokio::spawn(async move { dim_core::scanner::start(&mut conn, id, tx_clone, provider).await });

    Json(serde_json::json!({ "id": id })).into_response()
}

/// Method mapped to `DELETE /api/v1/library/<id>` deletes the library with the supplied id from the path.
pub async fn library_delete(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, DimErrorWrapper> {
    if !user.has_role("owner") {
        return Err(DimErrorWrapper(DimError::Unauthorized));
    }
    // First we mark the library as scheduled for deletion which will make the library and all its
    // content hidden. This is necessary because huge libraries take a long time to delete.
    {
        let mut lock = conn.writer().lock_owned().await;
        let mut tx = dim_database::write_tx(&mut lock).await.map_err(|err| {
            DimErrorWrapper(DimError::DatabaseError {
                description: err.to_string(),
            })
        })?;
        if Library::mark_hidden(&mut tx, id).await.map_err(|err| {
            DimErrorWrapper(DimError::DatabaseError {
                description: err.to_string(),
            })
        })? < 1
        {
            return Err(DimError::LibraryNotFound.into());
        }
        tx.commit().await.map_err(|err| {
            DimErrorWrapper(DimError::DatabaseError {
                description: err.to_string(),
            })
        })?;
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
            tracing::error!(reason = ?e, "Failed to delete library and its content.");
        } else {
            tracing::info!("Deleted library");
        }
    };

    tokio::spawn(delete_lib_fut);

    Ok(StatusCode::NO_CONTENT)
}

/// Method mapped to `GET /api/v1/library` returns a list of all libraries in the database
pub async fn library_get_all(State(state): State<AppState>) -> Response {
    let mut tx = match state.conn.read().begin().await {
        Ok(tx) => tx,
        Err(err) => {
            tracing::error!(?err, "Error getting connection");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    let libraries = Library::get_all(&mut tx).await;

    Json(libraries).into_response()
}

/// Method mapped to `GET /api/v1/library/<id>` returns info about the library with the supplied
/// id. Method can only be accessed by authenticated users.
///
pub async fn library_get_one(State(state): State<AppState>, Path(id): Path<i64>) -> Response {
    let mut tx = match state.conn.read().begin().await {
        Ok(tx) => tx,
        Err(err) => {
            tracing::error!(?err, "Error getting connection");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    let lib = match Library::get_one(&mut tx, id).await {
        Ok(library) => library,
        Err(err) => {
            tracing::error!(?err, "Error getting library");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    Json(lib).into_response()
}

/// Method mapped to `GET /api/v1/library/<id>/media` returns all the movies/tv shows that belong
/// to the library with the id supplied. Method can only be accessed by authenticated users.
///
pub async fn library_get_media(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Response {
    let mut result = HashMap::new();
    let mut tx = match conn.read().begin().await {
        Ok(tx) => tx,
        Err(err) => {
            tracing::error!(?err, "Error getting connection");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    let lib = match Library::get_one(&mut tx, id).await {
        Ok(library) => library,
        Err(err) => {
            tracing::error!(?err, "Error getting library");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    #[derive(Serialize)]
    struct Record {
        id: i64,
        name: String,
        poster_path: Option<String>,
    }

    let mut data = match sqlx::query_as!(
        Record,
        r#"SELECT _tblmedia.id, name, assets.local_path as poster_path FROM _tblmedia
        LEFT JOIN assets ON _tblmedia.poster = assets.id
        WHERE library_id = ? AND NOT media_type = "episode""#,
        id
    )
    .fetch_all(&mut *tx)
    .await
    {
        Ok(res) => res,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    if data.is_empty() {
        return (StatusCode::NOT_FOUND, "No media found".to_string()).into_response();
    }

    data.sort_by(|a, b| a.name.cmp(&b.name));

    result.insert(lib.name, data);

    Json(result).into_response()
}

#[derive(Deserialize)]
pub struct UnmatchedArgs {
    search: Option<String>,
}

/// Method mapped to `GET /api/v1/library/<id>/unmatched` returns a list of all unmatched medias
/// to be displayed in the library pages.
///
pub async fn library_get_unmatched(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<UnmatchedArgs>,
) -> Response {
    let mut tx = match conn.read().begin().await {
        Ok(tx) => tx,
        Err(err) => {
            tracing::error!(?err, "Error getting connection");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    // let mut files = CompactMediafile::unmatched_for_library(&mut tx, id)
    //     .await
    //     .map_err(|_| errors::DimError::NotFoundError)?;

    let mut files = match CompactMediafile::unmatched_for_library(&mut tx, id).await {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(?err, "Error getting unmatched files");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    // we want to pre-sort to ensure our tree is somewhat ordered.
    files.sort_by(|a, b| a.target_file.cmp(&b.target_file));

    if let Some(search) = params.search {
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

    let entry = crate::tree::Entry::build_with(
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
        files: Vec<crate::tree::Entry<Record>>,
    }

    let entries = match entry {
        crate::tree::Entry::Directory { files, .. } => files,
        _ => unreachable!(),
    };

    Json(Response {
        files: entries,
        count,
    })
    .into_response()
}
