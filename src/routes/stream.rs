use crate::core::DbConnection;
use crate::core::EventTx;
use crate::errors;
use auth::Wrapper as Auth;
use database::schema::mediafile::dsl::*;
use diesel::prelude::*;
use events::{Message, PushEventType};
use pushevent::Event;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::{json, json::JsonValue};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use streamer::{ffmpeg::FFmpeg, FFMPEG_BIN};

/*
#[derive(Debug)]
pub struct StreamReportForm {
    /*
    pub frame: u64,
    pub fps: f64,
    pub stream_0_0_q: i64,
    pub bitrate: String,
    pub total_size: u64,
    pub out_time_ms: u64,
    pub out_time: String,
    pub dup_frames: u64,
    pub drop_frames: u64,
    pub speed: String,
    pub progress: String,
    */
    lel: i32,
}

impl<'a> rocket::data::FromData<'a> for StreamReportForm {
    type Error = ();
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open().take(180);
        let mut string = String::new();
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(_) => Failure((Status::InternalServerError, ())),
        };

        // Returning `Borrowed` here means we get `Borrowed` in `from_data`.
        Transform::Borrowed(outcome)
    }

    fn from_data(_: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        // Retrieve a borrow to the now transformed `String` (an &str). This
        // is only correct because we know we _always_ return a `Borrowed` from
        // `transform` above.
        let string = dbg!(outcome.borrowed()?);

        // Return successfully.
        Success(Self { lel: 123 })
    }
}
*/

#[get("/stream/start/<_id>?<seek>&<vcodec>&<acodec>&<_brate>&<_res>")]
pub fn start_stream(
    conn: DbConnection,
    _id: i32,
    _user: Auth,
    seek: Option<u64>,
    vcodec: Option<String>,
    acodec: Option<String>,
    _brate: Option<u64>,
    _res: Option<u64>,
    event_tx: State<Arc<Mutex<EventTx>>>,
) -> Result<JsonValue, errors::DimError> {
    let mediafile_id = mediafile
        .filter(media_id.eq(Some(_id)))
        .select(id)
        .first::<i32>(conn.as_ref())?;

    let seek = seek.unwrap_or(0);
    let _vcodec = vcodec.unwrap_or("copy".to_string());
    let _acodec = acodec.unwrap_or("aac".to_string());

    let event_tx = event_tx.lock().unwrap().clone();

    let mut stream = FFmpeg::new(FFMPEG_BIN, mediafile_id, event_tx)?;
    let uuid = stream.stream(seek)?;
    return Ok(json!({ "uuid": uuid }));
}

#[delete("/stream/<uuid>")]
pub fn stop_stream(uuid: String, _user: Auth) -> Result<Status, errors::DimError> {
    FFmpeg::stop(uuid)?;
    Ok(Status::Ok)
}

#[get("/stream/static/<uuid>/<path..>")]
pub fn return_static(uuid: String, path: PathBuf, _user: Auth) -> Option<NamedFile> {
    let full_path = Path::new("./transcoding").join(uuid);
    NamedFile::open(full_path.join(path)).ok()
}

#[post("/stream/<uuid>/progress")]
// Note: This is a fucking insecure as fuck function, generate a random seed for ffmpeg maybe?
// For now we assume that as soon as ffmpeg starts reporting it is transcoding therefore we
// dispatch ready event
pub fn ffmpeg_progress(uuid: String, event_tx: State<Arc<Mutex<EventTx>>>) -> Result<(), ()> {
    let tx = event_tx.lock().unwrap();
    let new_event = Box::new(Message {
        id: -1, // we dont need a id for this so we ignore it with -1
        event_type: PushEventType::EventStreamIsReady,
    });

    let _ = tx.send(Event::new(format!("/events/stream/{}", uuid), new_event));
    Ok(())
}
