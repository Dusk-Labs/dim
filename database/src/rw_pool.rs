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

    // NOTE: the queries below caused some weird sqlx erros that looks like:
    //
    // thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: Database(SqliteError { code: 262, message: "database table is locked" })', /home/mental/Desktop/open_projects/dim/database/src/rw_pool.rs:51:10
    // thread 'tokio-runtime-worker' panicked at 'error occurred while dropping a transaction: cannot rollback - no transaction is active', /home/mental/.cargo/git/checkouts/sqlx-f05f33ba4f5c3036/694a2ac/sqlx-core/src/sqlite/transaction.rs:78:21
    //
    // the error was appearing at the begin exclusive query but commenting it out removes it
    // and it doesn't look like it breaks anything.

    // sqlx::query("END").execute(&mut tx).await.unwrap();

    // sqlx::query("BEGIN EXCLUSIVE")
    //     .execute(&mut tx)
    //     .await
    //     .unwrap();

    Ok(tx)
}
