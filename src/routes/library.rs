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

#[get("/")]
pub fn library_get(conn: DbConnection, _log: SyncLogger, _user: Auth) -> Json<Vec<Library>> {
    Json({
        let mut x = Library::get_all(conn.as_ref());
        x.sort_by(|a, b| a.name.cmp(&b.name));
        x
    })
}

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

#[get("/<id>")]
pub fn get_self(
    conn: DbConnection,
    id: i32,
    _user: Auth,
) -> Result<Json<Library>, errors::DimError> {
    Ok(Json(Library::get_one(conn.as_ref(), id)?))
}

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
