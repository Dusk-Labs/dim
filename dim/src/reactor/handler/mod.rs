mod error;

use super::types::EventType;
use super::types::Table;
use super::Event;
use super::Reactor;
use crate::core::EventTx;

use dim_database::asset::Asset;
use dim_database::library::Library;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::rw_pool::SqlitePool;

use dim_events::Message;
use dim_events::PushEventType;

use async_trait::async_trait;
use std::path::PathBuf;

pub type Error = error::Error;

pub struct EventReactor {
    pool: SqlitePool,
    ws_event_tx: Option<EventTx>,
}

impl EventReactor {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            ws_event_tx: None,
        }
    }

    pub fn with_websocket(mut self, event_tx: EventTx) -> Self {
        self.ws_event_tx = Some(event_tx);
        self
    }

    async fn handle_library(&mut self, event: Event) -> Result<(), Error> {
        let Some(event_tx) = self.ws_event_tx.as_mut() else {
            return Ok(());
        };

        assert_eq!(event.table, Table::Library);

        // TODO (val): should we also handle updates to the library? Such as renaming etc.
        let event_type = match event.event_type {
            EventType::Insert => PushEventType::EventNewLibrary,
            EventType::Delete => PushEventType::EventRemoveLibrary,
            EventType::Update => {
                // NOTE: Library usually get marked as hidden before being deleted as a UX
                // optimization. Sometimes library deletes can take a while.
                let mut tx = self
                    .pool
                    .read_ref()
                    .begin()
                    .await
                    .map_err(Error::ReadTransaction)?;

                let Library { hidden, .. } = Library::get_one(&mut tx, event.id)
                    .await
                    .map_err(Error::LibraryQuery)?;

                if !hidden {
                    return Ok(());
                }

                PushEventType::EventRemoveLibrary
            }
        };

        let event = Message {
            id: event.id,
            event_type,
        };

        event_tx
            .send(serde_json::to_string(&event).expect("Serializing event should never fail"))
            .map_err(Error::EventDispatch)?;

        Ok(())
    }

    async fn handle_assets(&mut self, event: Event) -> Result<(), Error> {
        assert_eq!(event.table, Table::Assets);

        if !matches!(event.event_type, EventType::Insert) {
            return Ok(());
        }

        let mut tx = self
            .pool
            .read_ref()
            .begin()
            .await
            .map_err(Error::ReadTransaction)?;

        let Asset {
            remote_url,
            local_path,
            ..
        } = Asset::get_by_id(&mut tx, event.id)
            .await
            .map_err(Error::AssetQuery)?;

        if let Some(remote_url) = remote_url {
            let path = PathBuf::from(local_path);

            if let Some(local_file) = path.iter().last().map(|x| x.to_string_lossy().to_string()) {
                crate::fetcher::insert_into_queue(remote_url, local_file, false).await;
            }
        }

        Ok(())
    }

    async fn handle_media(&mut self, event: Event) -> Result<(), Error> {
        let Some(event_tx) = self.ws_event_tx.as_mut() else {
            return Ok(());
        };

        assert_eq!(event.table, Table::Media);

        let mut tx = self
            .pool
            .read_ref()
            .begin()
            .await
            .map_err(Error::ReadTransaction)?;

        let event_type = match event.event_type {
            EventType::Insert => {
                let (library_id, media_type) = Media::get_compact(&mut tx, event.id)
                    .await
                    .map_err(Error::MediaQuery)?;

                if !matches!(media_type, MediaType::Movie | MediaType::Tv) {
                    return Ok(());
                }

                PushEventType::EventNewCard { lib_id: library_id }
            }
            EventType::Delete => PushEventType::EventRemoveCard,
            _ => return Ok(()),
        };

        let event = Message {
            id: event.id,
            event_type,
        };

        event_tx
            .send(serde_json::to_string(&event).expect("Serializing event should never fail"))
            .map_err(Error::EventDispatch)?;

        Ok(())
    }
}

#[async_trait]
impl Reactor for EventReactor {
    type Error = Error;

    async fn react(&mut self, event: Event) -> Result<(), Self::Error> {
        match event.table {
            Table::Library => self.handle_library(event).await,
            Table::Media => self.handle_media(event).await,
            Table::Assets => self.handle_assets(event).await,
        }
    }
}
