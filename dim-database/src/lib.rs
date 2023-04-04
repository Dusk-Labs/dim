// FIXME: We have a shim in dim/utils but we cant depend on dim because itd be a circular dep.

use crate::utils::ffpath;

use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use sqlx::ConnectOptions;
use tracing::{info, instrument};

use once_cell::sync::OnceCell;

pub mod asset;
pub mod compact_mediafile;
pub mod episode;
pub mod error;
pub mod genre;
pub mod library;
pub mod media;
pub mod mediafile;
pub mod movie;
pub mod progress;
pub mod query_ext;
pub mod rw_pool;
pub mod season;
pub mod tv;
pub mod user;
pub mod utils;

#[cfg(test)]
pub mod tests;

pub use crate::error::DatabaseError;
/// Ugly hack because of a shitty deadlock in `Pool`
pub use crate::rw_pool::write_tx;
pub use dim_auth::generate_key;
pub use dim_auth::set_key;

pub type DbConnection = rw_pool::SqlitePool;
pub type Transaction<'tx> = sqlx::Transaction<'tx, sqlx::Sqlite>;

lazy_static::lazy_static! {
    static ref MIGRATIONS_FLAG: AtomicBool = AtomicBool::new(false);
}

static __GLOBAL: OnceCell<DbConnection> = OnceCell::new();
const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/");

/// Function runs all migrations embedded to make sure the database works as expected.
///
/// # Arguments
/// * `conn` - diesel connection
async fn run_migrations(conn: &crate::DbConnection) -> Result<(), sqlx::migrate::MigrateError> {
    let mut lock = conn.writer().lock_owned().await;
    MIGRATOR.run(&mut *lock).await
}

/// Function which returns a Result<T, E> where T is a new connection session or E is a connection
/// error.
pub async fn get_conn() -> sqlx::Result<crate::DbConnection> {
    let conn = if let Some(conn) = __GLOBAL.get() {
        conn
    } else {
        let conn = internal_get_conn().await?;
        let _ = __GLOBAL.set(conn);
        __GLOBAL.get().unwrap()
    };

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) {
        if let Err(err) = run_migrations(conn).await {
            panic!(
                "Failed to run migrations (maybe you need to delete the old database?): {:?}",
                err
            );
        } else {
            MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
        }
    }

    Ok(conn.clone())
}

#[doc(hidden)]
pub fn set_conn(conn: crate::DbConnection) {
    __GLOBAL.set(conn).unwrap();
}

pub fn try_get_conn() -> Option<&'static crate::DbConnection> {
    __GLOBAL.get()
}

pub async fn get_conn_memory() -> sqlx::Result<crate::DbConnection> {
    let pool = sqlx::Pool::connect(":memory:").await?;
    let connection: sqlx::pool::PoolConnection<sqlx::Sqlite> = pool.acquire().await?;
    let rw = connection.detach();
    let pool = rw_pool::SqlitePool::new(rw, pool);
    let _ = run_migrations(&pool).await?;

    Ok(pool)
}

/// Function returns a connection to the development table of dim. This is mainly used for unit
/// tests.
#[doc(hidden)]
pub async fn get_conn_devel() -> sqlx::Result<crate::DbConnection> {
    let rw_only = sqlx::sqlite::SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename("./dim_dev.db")
        .connect()
        .await?;

    let rd_only = sqlx::pool::PoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .read_only(true)
                .create_if_missing(true)
                .filename("./dim_dev.db"),
        )
        .await?;

    let pool = rw_pool::SqlitePool::new(rw_only, rd_only);

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && run_migrations(&pool).await.is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(pool)
}

/// Function which returns a Result<T, E> where T is a new connection session or E is a connection
/// error. It takes in a logger instance.
///
/// # Arguments
/// * `log` - a Slog logger instance
#[instrument]
pub async fn get_conn_logged() -> sqlx::Result<DbConnection> {
    // This is the URL for the database inside a docker container
    let conn = if let Some(conn) = __GLOBAL.get() {
        conn
    } else {
        let conn = internal_get_conn().await?;
        let _ = __GLOBAL.set(conn);
        __GLOBAL.get().unwrap()
    };

    info!("Creating new database connection");

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && dbg!(run_migrations(&conn).await).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(conn.clone())
}

async fn internal_get_conn() -> sqlx::Result<DbConnection> {
    let rw_only = sqlx::sqlite::SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(ffpath("config/dim.db"))
        .connect()
        .await?;

    let rd_only = sqlx::pool::PoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(ffpath("config/dim.db"))?
                .read_only(true)
                .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                .create_if_missing(true),
        )
        .await?;

    Ok(rw_pool::SqlitePool::new(rw_only, rd_only))
}

#[doc(hidden)]
pub async fn get_conn_file(file: &str) -> sqlx::Result<crate::DbConnection> {
    let rw_only = sqlx::sqlite::SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(file)
        .connect()
        .await?;

    let rd_only = sqlx::pool::PoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(file)?
                .read_only(true)
                .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                .create_if_missing(true),
        )
        .await?;

    let pool = rw_pool::SqlitePool::new(rw_only, rd_only);

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    Ok(pool)
}
