use sqlx::Pool;
use sqlx::Sqlite;

use tracing::info_span;
use tracing::Instrument;

use std::future::Future;

#[derive(Debug, Clone)]
pub struct SqlitePool {
    writer: Pool<Sqlite>,
    reader: Pool<Sqlite>,
}

impl SqlitePool {
    pub fn new(writer: Pool<Sqlite>, reader: Pool<Sqlite>) -> Self {
        Self { writer, reader }
    }

    pub fn read(&self) -> Pool<Sqlite> {
        self.reader.clone()
    }

    pub async fn write(&self) -> Result<crate::Transaction<'_>, sqlx::Error> {
        let mut tx = self.writer.begin().instrument(info_span!("TxBegin")).await?;

        sqlx::query("END").execute(&mut tx).await?;
        sqlx::query("BEGIN EXCLUSIVE").execute(&mut tx).await?;

        Ok(tx)
    }

    pub fn read_ref(&self) -> &Pool<Sqlite> {
        &self.reader
    }

    pub fn write_ref(&self) -> &Pool<Sqlite> {
        &self.writer
    }
}
