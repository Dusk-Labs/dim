use displaydoc::Display;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Debug, Display, Error)]
pub enum DatabaseError {
    /// Generic database error: {0:?}
    DatabaseError(Arc<sqlx::error::Error>),
}

impl From<sqlx::error::Error> for DatabaseError {
    fn from(e: sqlx::error::Error) -> DatabaseError {
        Self::DatabaseError(e.into())
    }
}
