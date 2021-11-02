use cfg_if::cfg_if;

use once_cell::sync::OnceCell;

use crate::utils::ffpath;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tracing::{info, instrument};

pub mod asset;
pub mod episode;
pub mod error;
pub mod genre;
pub mod library;
pub mod media;
pub mod mediafile;
pub mod movie;
pub mod progress;
pub mod season;
#[cfg(test)]
pub mod tests;
pub mod tv;
pub mod user;
pub mod utils;

pub use crate::error::DatabaseError;

#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("Features sqlite and postgres are mutually exclusive");

cfg_if! {
    if #[cfg(feature = "sqlite")] {
        pub type DbConnection = sqlx::SqlitePool;

    } else {
        pub type DbConnection = sqlx::PgPool;
    }
}

lazy_static::lazy_static! {
    static ref MIGRATIONS_FLAG: AtomicBool = AtomicBool::new(false);
}

static __GLOBAL: OnceCell<crate::DbConnection> = OnceCell::new();

cfg_if! {
    if #[cfg(feature = "postgres")] {
        const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations/postgres");
    } else {
        const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/");
    }
}

/// Function runs all migrations embedded to make sure the database works as expected.
///
/// # Arguments
/// * `conn` - diesel connection
async fn run_migrations(conn: &crate::DbConnection) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(conn).await
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
            dbg!(err);
        } else {
            MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
        }
    }

    Ok(conn.clone())
}

pub fn try_get_conn() -> Option<&'static crate::DbConnection> {
    __GLOBAL.get()
}

#[cfg(all(feature = "sqlite", test))]
pub async fn get_conn_memory() -> sqlx::Result<crate::DbConnection> {
    let pool = sqlx::Pool::connect(":memory:").await?;
    let _ = dbg!(run_migrations(&pool).await);
    Ok(pool)
}

/// Function returns a connection to the development table of dim. This is mainly used for unit
/// tests.
pub async fn get_conn_devel() -> sqlx::Result<crate::DbConnection> {
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            let pool = internal_get_conn_custom(
                None,
                "postgres://postgres:dimpostgres@127.0.0.1/dim_devel",
            ).await?;
        } else {
            let pool = sqlx::Pool::connect("./dim_dev.db").await?;
        }
    }

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
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            internal_get_conn_custom(
                "postgres://postgres:dimpostgres@127.0.0.1/dim"
            ).await
        } else {
                sqlx::Pool::connect_with(
                    sqlx::sqlite::SqliteConnectOptions::from_str(ffpath("config/dim.db"))?
                    .create_if_missing(true)
                    .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                ).await
        }
    }
}

#[cfg(feature = "postgres")]
#[async_recursion::async_recursion]
async fn internal_get_conn_custom(main: &str) -> sqlx::Result<DbConnection> {
    let pool = sqlx::Pool::connect(main).await;

    if pool.is_ok() {
        return pool;
    }

    let pool = sqlx::Pool::connect("postgres://postgres:dimpostgres@127.0.0.1/").await;

    if let Some(log) = log {
        warn!("Database dim seems to not exist, creating...standby...");
    }
    let _ = create_database(&pool?);

    Ok(internal_get_conn(log).await?)
}

#[cfg(feature = "postgres")]
async fn create_database(conn: &crate::DbConnection) -> sqlx::Result<()> {
    sqlx::query_unchecked!("CREATE DATABASE dim")
        .execute(conn)
        .await?;
    sqlx::query_unchecked!("CREATE DATABASE dim_devel")
        .execute(conn)
        .await?;
    sqlx::query_unchecked!("CREATE DATABASE pg_trgm")
        .execute(conn)
        .await?;

    Ok(())
}
