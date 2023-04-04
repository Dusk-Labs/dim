use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum Error {
    /// Failed to dispatch event to websocket clients: {0:?}
    EventDispatch(tokio::sync::mpsc::error::SendError<String>),
    /// Failed to open read-only transaction: {0:?}
    ReadTransaction(sqlx::Error),
    /// Failed to query library: {0:?}
    LibraryQuery(dim_database::DatabaseError),
    /// Failed to query asset: {0:?}
    AssetQuery(dim_database::DatabaseError),
    /// Failed to query media: {0:?}
    MediaQuery(dim_database::DatabaseError),
}
