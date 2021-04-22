use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;
use crate::routes::construct_standard;
//use crate::scanners;

use auth::Wrapper as Auth;

use database::library::InsertableLibrary;
use database::library::Library;
use database::mediafile::MediaFile;

use events::Message;
use events::PushEventType;

use rocket::{http::Status, State};
use rocket_contrib::json::{Json, JsonValue};

use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use std::sync::Mutex;

use futures::stream;
use futures::StreamExt;

use slog::Logger;
use tokio_diesel::*;

/// Method maps to `GET /api/v1/library` and returns a list of all libraries in te database.
/// This method can only be accessed by authenticated users.
///
/// # Arguments * `conn` - database connection
/// * `_log` - logger
/// * `_user` - Authentication middleware
#[get("/")]
pub async fn library_get(conn: State<'_, DbConnection>, _user: Auth) -> Json<Vec<Library>> {
    Json({
        let mut x = Library::get_all(&conn).await;
        x.sort_by(|a, b| a.name.cmp(&b.name));
        x
    })
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
#[post("/", format = "application/json", data = "<new_library>")]
pub async fn library_post(
    conn: State<'_, DbConnection>,
    new_library: Json<InsertableLibrary>,
    log: State<'_, Logger>,
    event_tx: State<'_, Arc<Mutex<EventTx>>>,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    let id = new_library.insert(&conn).await?;
    let tx = event_tx.lock().unwrap();
    let tx_clone = tx.clone();

    // TODO: Throw this into the thread map
    /*
    std::thread::spawn(move || {
        scanners::start(id, log.get(), tx_clone).unwrap();
    });
    */

    let event = Message {
        id,
        event_type: PushEventType::EventNewLibrary,
    };

    let _ = tx.send(serde_json::to_string(&event).unwrap());

    Ok(Status::Created)
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
#[delete("/<id>")]
pub async fn library_delete(
    conn: State<'_, DbConnection>,
    id: i32,
    event_tx: State<'_, Arc<Mutex<EventTx>>>,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "sqlite")] {
            use database::media::Media;
            use database::mediafile::MediaFile;
            use diesel::prelude::*;

            diesel::sql_query("PRAGMA foreign_keys = ON").execute_async(&conn).await?;
            Media::delete_by_lib_id(&conn, id).await?;
            MediaFile::delete_by_lib_id(&conn, id).await?;
        }
    }

    Library::delete(&conn, id).await?;

    let event = Message {
        id,
        event_type: PushEventType::EventRemoveLibrary,
    };

    let _ = event_tx
        .lock()
        .unwrap()
        .send(serde_json::to_string(&event).unwrap());

    Ok(Status::NoContent)
}

/// Method mapped to `GET /api/v1/library/<id>` returns info about the library with the supplied
/// id. Method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library we want info of
/// * `_user` - Auth middleware
#[get("/<id>")]
pub async fn get_self(
    conn: State<'_, DbConnection>,
    id: i32,
    _user: Auth,
) -> Result<Json<Library>, errors::DimError> {
    Ok(Json(Library::get_one(&conn, id).await?))
}

/// Method mapped to `GET /api/v1/library/<id>/media` returns all the movies/tv shows that belong
/// to the library with the id supplied. Method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library we want media of
/// * `_user` - Auth middleware
#[get("/<id>/media")]
pub async fn get_all_library(
    conn: State<'_, DbConnection>,
    id: i32,
    user: Auth,
) -> Result<Json<HashMap<String, Vec<JsonValue>>>, errors::DimError> {
    let mut result = HashMap::new();
    let lib = Library::get_one(&conn, id).await?;
    let mut data = Library::get(&conn, id).await?;

    data.sort_by(|a, b| a.name.cmp(&b.name));
    let out = stream::iter(data)
        .filter_map(|x| async { construct_standard(&conn, &x.into(), &user).await.ok() })
        .collect::<Vec<JsonValue>>()
        .await;

    result.insert(lib.name, out);

    Ok(Json(result))
}

/// Method mapped to `GET` /api/v1/library/<id>/unmatched` returns a list of all unmatched medias
/// to be displayed in the library pages.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library
/// * `_user` - auth middleware
// NOTE: construct_standard on a mediafile will yield buggy deltas
#[get("/<id>/unmatched")]
pub async fn get_all_unmatched_media(
    conn: State<'_, DbConnection>,
    id: i32,
    user: Auth,
) -> Result<Json<HashMap<String, Vec<JsonValue>>>, errors::DimError> {
    let mut result = HashMap::new();
    let lib = Library::get_one(&conn, id).await?;

    let filtered = MediaFile::get_by_lib_null_media(&conn, &lib).await?
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
                construct_standard(&conn, &v.into(), &user).await
                    .ok()
                    .and_then(|x| Some((k, x)))
            }
        })
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(k, v)| result.entry(k).or_insert(vec![]).push(v));

    Ok(Json(result))
}
