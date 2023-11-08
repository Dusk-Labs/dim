use displaydoc::Display;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Debug, Error, Display, Serialize)]
pub enum Error {
    /// Provided external id not found by provider.
    InvalidExternalId,
    /// Movie scanner error: {0:?}
    MovieScanner(#[from] super::movie::Error),
    /// Tv show scanner error: {0:?}
    TvScanner(#[from] super::tv_show::Error),
    /// Mediafile insert error: {0:?}
    MediafileError(#[from] super::mediafile::Error),
    /// Failed to dispatch websocket event: {0:?}
    EventDispatch(#[serde(skip)] Arc<tokio::sync::mpsc::error::SendError<std::string::String>>),
    /// Database error: {0:?}
    DatabaseError(
        #[from]
        #[serde(skip)]
        Arc<sqlx::Error>,
    ),
    /// Library supplied doesnt exist: {0:?}
    LibraryNotFound(#[serde(skip)] dim_database::DatabaseError),
}
