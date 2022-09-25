use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::SqliteConnection;

use tracing::debug_span;
use tracing::Instrument;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::OwnedMutexGuard;

#[derive(Debug, Clone)]
pub struct SqlitePool {
    pub writer: Arc<Mutex<SqliteConnection>>,
    reader: Pool<Sqlite>,
}

impl SqlitePool {
    pub fn new(writer: SqliteConnection, reader: Pool<Sqlite>) -> Self {
        Self {
            writer: Arc::new(Mutex::new(writer)),
            reader,
        }
    }

    pub fn read(&self) -> Pool<Sqlite> {
        self.reader.clone()
    }

    pub fn writer(&self) -> Arc<Mutex<SqliteConnection>> {
        self.writer.clone()
    }

    pub fn read_ref(&self) -> &Pool<Sqlite> {
        &self.reader
    }
}

pub async fn write_tx(
    lock: &mut OwnedMutexGuard<SqliteConnection>,
) -> Result<crate::Transaction<'_>, sqlx::Error> {
    use sqlx::Connection;

    let tx = lock.begin().instrument(debug_span!("TxBegin")).await?;

    Ok(tx)
}
