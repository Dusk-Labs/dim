pub use auth::AuthError;

use displaydoc::Display;
use thiserror::Error;
use std::sync::Arc;

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
