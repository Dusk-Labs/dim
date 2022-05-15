pub use auth::AuthError;

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum DatabaseError {
    /// Generic database error: {0:?}
    DatabaseError(sqlx::error::Error),
}

impl From<sqlx::error::Error> for DatabaseError {
    fn from(e: sqlx::error::Error) -> DatabaseError {
        Self::DatabaseError(e)
    }
}
