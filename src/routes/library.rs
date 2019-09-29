use crate::core::DbConnection;
use crate::core::EventTx;
use crate::routes::general::construct_standard;
use dim_database::library::{InsertableLibrary, Library};
use dim_scanners;
use dim_events::event::{Message, PushEventType, Event};
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_slog::SyncLogger;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[get("/")]
pub fn library_get(conn: DbConnection, _log: SyncLogger) -> Json<Vec<Library>> {
    Json(Library::get_all(&conn))
}

#[post("/", format = "application/json", data = "<new_library>")]
pub fn library_post(
    conn: DbConnection,
    new_library: Json<InsertableLibrary>,
    log: SyncLogger,
    event_tx: State<Arc<Mutex<EventTx>>>,
) -> Result<Status, Status> {
    match new_library.insert(&conn) {
        Ok(id) => {
            let tx = event_tx.lock().unwrap();
            let tx_clone = tx.clone();
            std::thread::spawn(move || {
                dim_scanners::start(id, log.get(), tx_clone).unwrap();
            });

            let event_message = Message {
                id,
                event_type: PushEventType::EventNewLibrary,
            };

            let event = Event::new("/events/library", event_message);
            let _ = tx.send(event);
            Ok(Status::Created)
        }
        Err(_) => Err(Status::NotImplemented),
    }
}

#[delete("/<id>")]
pub fn library_delete(conn: DbConnection, id: i32, event_tx: State<Arc<Mutex<EventTx>>>) -> Result<Status, Status> {
    match Library::delete(&conn, id) {
        Ok(_) => {
            let event_message = Message {
                id,
                event_type: PushEventType::EventRemoveLibrary,
            };

            let event = Event::new("/events/library", event_message);
            let _ = event_tx.lock().unwrap().send(event);
            Ok(Status::NoContent)
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<id>")]
pub fn get_self(conn: DbConnection, id: i32) -> Result<Json<Library>, Status> {
    match Library::get_one(&conn, id) {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<id>/media")]
pub fn get_all_library(
    conn: DbConnection,
    id: i32,
) -> Result<Json<HashMap<String, Vec<JsonValue>>>, Status> {
    let mut result: HashMap<String, Vec<JsonValue>> = HashMap::new();
    if let Ok(lib) = Library::get_one(&conn, id) {
        match Library::get(&conn, id) {
            Ok(mut data) => {
                data.sort_by(|a, b| a.name.cmp(&b.name));
                let out = data
                    .iter()
                    .map(|x| construct_standard(&conn, x, None))
                    .collect::<Vec<JsonValue>>();
                result.insert(lib.name, out);
                Ok(Json(result))
            }
            Err(_) => Err(Status::NotFound),
        }
    } else {
        Err(Status::NotFound)
    }
}
