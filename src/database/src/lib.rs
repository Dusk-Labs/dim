#![feature(rustc_private)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel_derive_enum;

use cfg_if::cfg_if;
use diesel::connection::Connection;
use diesel::{result::ConnectionError, RunQueryDsl};
use slog::Logger;
use std::sync::atomic::{AtomicBool, Ordering};

pub mod episode;
pub mod genre;
pub mod library;
pub mod media;
pub mod mediafile;
pub mod movie;
pub mod progress;
pub mod schema;
pub mod season;
pub mod streamable_media;
pub mod tv;
pub mod user;

#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("Features sqlite and postgres are mutually exclusive");

cfg_if! {
    if #[cfg(feature = "sqlite")] {
        pub type DbConnection = diesel::SqliteConnection;

        // Necessary for get_result like functionality for sqlite.
        no_arg_sql_function!(
            last_insert_rowid,
            diesel::sql_types::Integer,
            "Represents the SQL last_insert_row() function"
        );

        // Necessary to emulate ilike.
        use diesel::sql_types::Text;
        sql_function!(fn upper(x: Text) -> Text);
    } else {
        pub type DbConnection = diesel::PgConnection;
    }
}

lazy_static::lazy_static! {
    static ref MIGRATIONS_FLAG: AtomicBool = AtomicBool::new(false);
}

cfg_if! {
    if #[cfg(feature = "postgres")] {
        embed_migrations!("../../migrations/postgres");
    } else {
        embed_migrations!("../../migrations/sqlite");
    }
}

fn create_database(conn: &crate::DbConnection) -> Result<(), diesel::result::Error> {
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            let _ = diesel::sql_query("CREATE DATABASE dim").execute(conn)?;
            let _ = diesel::sql_query("CREATE DATABASE dim_devel").execute(conn)?;
            let _ = diesel::sql_query("CREATE DATABASE pg_trgm").execute(conn)?;
        } else {
            let _ = diesel::sql_query("PRAGMA journal_mode=WAL").execute(conn)?;
            let _ = diesel::sql_query("PRAGMA synchronous=NORMAL").execute(conn)?;
            let _ = diesel::sql_query("PRAGMA busy_timeout=50000").execute(conn)?;
            let _ = diesel::sql_query("PRAGMA foreign_keys = ON").execute(conn)?;
        }
    }
    Ok(())
}

/// Function runs all migrations embedded to make sure the database works as expected.
///
/// # Arguments
/// * `conn` - diesel connection
fn run_migrations(conn: &crate::DbConnection) -> Result<(), diesel_migrations::RunMigrationsError> {
    // TODO: Move the init.sql queries into here.
    embedded_migrations::run(conn)
}

/// Function which returns a Result<T, E> where T is a new connection session or E is a connection
/// error.
///
/// # Example
/// ```
/// use database::get_conn;
///
/// let conn = get_conn().unwrap(); // panics if connection failed.
/// ```
pub fn get_conn() -> Result<crate::DbConnection, diesel::result::ConnectionError> {
    // This is the URL for the database inside a docker container
    let conn = internal_get_conn(None)?;

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && dbg!(run_migrations(&conn)).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(conn)
}

/// Function returns a connection to the development table of dim. This is mainly used for unit
/// tests.
pub fn get_conn_devel() -> Result<crate::DbConnection, diesel::result::ConnectionError> {
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            let conn = internal_get_conn_custom(
                None,
                "postgres://postgres:dimpostgres@127.0.0.1/dim_devel",
                "postgres://postgres:dimpostgres@postgres/dim_devel",
            )?;
        } else {
            let mut conn = DbConnection::establish("./dim_devel.db;foreign_keys=true;")?;
            let _ = diesel::sql_query("PRAGMA journal_mode=WAL").execute(&conn);
            let _ = diesel::sql_query("PRAGMA synchronous=NORMAL").execute(&conn);
            let _ = diesel::sql_query("PRAGMA busy_timeout=50000").execute(&conn);
            let _ = diesel::sql_query("PRAGMA foreign_keys = ON").execute(&conn);
        }
    }

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && run_migrations(&conn).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(conn)
}

/// Function which returns a Result<T, E> where T is a new connection session or E is a connection
/// error. It takes in a logger instance.
///
/// # Arguments
/// * `log` - a Slog logger instance
pub fn get_conn_logged(log: &Logger) -> Result<DbConnection, diesel::result::ConnectionError> {
    // This is the URL for the database inside a docker container
    let conn = internal_get_conn(Some(&log))?;
    slog::info!(log, "Creating new database connection");

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && dbg!(run_migrations(&conn)).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(conn)
}

fn internal_get_conn(
    log: Option<&Logger>,
) -> Result<DbConnection, diesel::result::ConnectionError> {
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            internal_get_conn_custom(
                log,
                "postgres://postgres:dimpostgres@127.0.0.1/dim",
                "postgres://postgres:dimpostgres@postgres/dim",
            )
        } else {
            let mut conn = DbConnection::establish("./dim.db")?;
            let _ = diesel::sql_query("PRAGMA foreign_keys=ON;").execute(&conn).unwrap();
            let _ = diesel::sql_query("PRAGMA journal_mode=WAL").execute(&conn);
            let _ = diesel::sql_query("PRAGMA synchronous=NORMAL").execute(&conn);
            let _ = diesel::sql_query("PRAGMA busy_timeout=50000").execute(&conn);
            Ok(conn)
        }
    }
}

fn internal_get_conn_custom(
    log: Option<&Logger>,
    main: &str,
    fallback: &str,
) -> Result<DbConnection, diesel::result::ConnectionError> {
    let conn = DbConnection::establish(main);

    let conn = if conn.is_ok() {
        conn
    } else {
        DbConnection::establish(fallback)
    };

    if conn.is_ok() {
        return Ok(conn?);
    }

    if let Err(e) = conn {
        if let ConnectionError::BadConnection(_) = e {
            let conn = DbConnection::establish("postgres://postgres:dimpostgres@127.0.0.1/");

            let conn = if conn.is_ok() {
                conn
            } else {
                DbConnection::establish("postgres://postgres:dimpostgres@postgres/")
            }?;

            if let Some(log) = log {
                slog::warn!(
                    log,
                    "Database dim seems to not exist, creating...standby..."
                );
            }
            let _ = create_database(&conn);
        }
    };

    Ok(internal_get_conn(log)?)
}
