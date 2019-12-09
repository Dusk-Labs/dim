use pushevent::SerializableEvent;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Message {
    pub id: i32,
    #[serde(flatten)]
    pub event_type: PushEventType,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum PushEventType {
    EventNewCard,
    EventRemoveCard,
    EventNewLibrary,
    EventRemoveLibrary,
    EventStreamIsReady,
    EventStreamStats(HashMap<String, String>),
}

impl SerializableEvent for Message {
    /// Serialize method used as a intermediary to serialize the struct into a json string and
    /// return it.
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
