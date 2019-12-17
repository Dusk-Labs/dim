use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;
use crate::routes::general::construct_standard;
use auth::Wrapper as Auth;
use database::{
    library::{InsertableLibrary, Library},
    mediafile::MediaFile,
};
use events::{Message, PushEventType};
use pushevent::Event;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_slog::SyncLogger;
use scanners;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Method maps to `GET /api/v1/library` and returns a list of all libraries in te database.
/// This method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `_log` - logger
/// * `_user` - Authentication middleware
#[get("/")]
pub fn library_get(conn: DbConnection, _log: SyncLogger, _user: Auth) -> Json<Vec<Library>> {
    Json({
        let mut x = Library::get_all(conn.as_ref());
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
pub fn library_post(
    conn: DbConnection,
    new_library: Json<InsertableLibrary>,
    log: SyncLogger,
    event_tx: State<Arc<Mutex<EventTx>>>,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    let id = new_library.insert(conn.as_ref())?;
    let tx = event_tx.lock().unwrap();
    let tx_clone = tx.clone();

    // TODO: Throw this into the thread map
    std::thread::spawn(move || {
        scanners::start(id, log.get(), tx_clone).unwrap();
    });

    let event_message = Box::new(Message {
        id,
        event_type: PushEventType::EventNewLibrary,
    });

    let event = Event::new("/events/library".to_string(), event_message);
    let _ = tx.send(event);
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
pub fn library_delete(
    conn: DbConnection,
    id: i32,
    event_tx: State<Arc<Mutex<EventTx>>>,
    _user: Auth,
) -> Result<Status, errors::DimError> {
    let _ = Library::delete(conn.as_ref(), id)?;
    let event_message = Box::new(Message {
        id,
        event_type: PushEventType::EventRemoveLibrary,
    });

    let event = Event::new("/events/library".to_string(), event_message);
    let _ = event_tx.lock().unwrap().send(event);
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
pub fn get_self(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<Json<Library>, errors::DimError> {
    Ok(Json(Library::get_one(conn.as_ref(), id)?))
}

/// Method mapped to `GET /api/v1/library/<id>/media` returns all the movies/tv shows that belong
/// to the library with the id supplied. Method can only be accessed by authenticated users.
///
/// # Arguments
/// * `conn` - database connection
/// * `id` - id of the library we want media of
/// * `_user` - Auth middleware
#[get("/<id>/media")]
pub fn get_all_library(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<Json<HashMap<String, Vec<JsonValue>>>, errors::DimError> {
    let mut result = HashMap::new();
    let lib = Library::get_one(conn.as_ref(), id)?;
    let mut data = Library::get(conn.as_ref(), id)?;

    data.sort_by(|a, b| a.name.cmp(&b.name));
    let out = data
        .iter()
        .map(|x| construct_standard(&conn, x, false))
        .collect::<Vec<JsonValue>>();

    result.insert(lib.name.clone(), out);

    if let Ok(x) = MediaFile::get_by_lib_null_media(conn.as_ref(), &lib) {
        result.insert(
            "Unmatched Media".into(),
            x.into_iter()
                .map(|x| construct_standard(&conn, &x.into(), false))
                .collect::<Vec<JsonValue>>(),
        );
    }

    Ok(Json(result))
}
