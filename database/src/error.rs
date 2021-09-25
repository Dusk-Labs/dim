use err_derive::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(display = "{:?}", _0)]
    DatabaseError(sqlx::error::Error),
}

impl From<sqlx::error::Error> for DatabaseError {
    fn from(e: sqlx::error::Error) -> DatabaseError {
        Self::DatabaseError(e)
    }
}
