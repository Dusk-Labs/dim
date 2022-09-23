use super::types::EventType;
use super::types::Table;
use super::Event;
use super::Reactor;
use crate::core::EventTx;

use events::Message;
use events::PushEventType;

use async_trait::async_trait;

pub type Error = std::convert::Infallible;

pub struct EventReactor {
    ws_event_tx: Option<EventTx>,
}

impl EventReactor {
    pub fn new() -> Self {
        Self { ws_event_tx: None }
    }

    pub fn with_websocket(mut self, event_tx: EventTx) -> Self {
        self.ws_event_tx = Some(event_tx);
        self
    }

    pub fn handle_library(&mut self, event: Event) -> Result<(), Error> {
        let Some(event_tx) = self.ws_event_tx.as_mut() else {
            return Ok(());
        };

        assert_eq!(event.table, Table::Library);

        // TODO (val): should we also handle updates to the library? Such as renaming etc.
        let event_type = match event.event_type {
            EventType::Insert => PushEventType::EventNewLibrary,
            EventType::Delete => PushEventType::EventRemoveLibrary,
            _ => return Ok(()),
        };

        let event = Message {
            id: event.id,
            event_type,
        };

        event_tx
            .send(serde_json::to_string(&event).expect("Serializing event should never fail"))
            .unwrap();

        Ok(())
    }
}

#[async_trait]
impl Reactor for EventReactor {
    type Error = Error;

    async fn react(&mut self, event: Event) -> Result<(), Self::Error> {
        match event.table {
            Table::Library => self.handle_library(event),
            _ => return Ok(()),
        }
    }
}
