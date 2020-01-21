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

use diesel::{connection::Connection, pg::PgConnection, result::ConnectionError, RunQueryDsl};
use slog::Logger;
use std::sync::atomic::{AtomicBool, Ordering};

pub mod episode;
pub mod genre;
pub mod library;
pub mod media;
pub mod mediafile;
pub mod movie;
pub mod schema;
pub mod season;
pub mod streamablemedia;
pub mod tv;
pub mod user;

lazy_static::lazy_static! {
    static ref MIGRATIONS_FLAG: AtomicBool = AtomicBool::new(false);
}

embed_migrations!("../../migrations");

fn create_database(conn: &diesel::PgConnection) -> Result<(), diesel::result::Error> {
    let _ = diesel::sql_query("CREATE DATABASE dim").execute(conn)?;
    let _ = diesel::sql_query("CREATE DATABASE dim_devel").execute(conn)?;
    let _ = diesel::sql_query("CREATE DATABASE pg_trgm").execute(conn)?;
    Ok(())
}

/// Function runs all migrations embedded to make sure the database works as expected.
///
/// # Arguments
/// * `conn` - diesel connection
fn run_migrations(
    conn: &diesel::PgConnection,
) -> Result<(), diesel_migrations::RunMigrationsError> {
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
pub fn get_conn() -> Result<diesel::PgConnection, diesel::result::ConnectionError> {
    // This is the URL for the database inside a docker container
    let conn = internal_get_conn(None)?;

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && run_migrations(&conn).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(conn)
}

/// Function returns a connection to the development table of dim. This is mainly used for unit
/// tests.
pub fn get_conn_devel() -> Result<diesel::PgConnection, diesel::result::ConnectionError> {
    let conn = internal_get_conn_custom(
        None,
        "postgres://postgres:dimpostgres@127.0.0.1/dim_devel",
        "postgres://postgres:dimpostgres@postgres/dim_devel",
    )?;

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
pub fn get_conn_logged(
    log: &Logger,
) -> Result<diesel::PgConnection, diesel::result::ConnectionError> {
    // This is the URL for the database inside a docker container
    let conn = internal_get_conn(Some(&log))?;
    slog::info!(log, "Creating new database connection");

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && run_migrations(&conn).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(conn)
}

fn internal_get_conn(
    log: Option<&Logger>,
) -> Result<diesel::PgConnection, diesel::result::ConnectionError> {
    internal_get_conn_custom(
        log,
        "postgres://postgres:dimpostgres@127.0.0.1/dim",
        "postgres://postgres:dimpostgres@postgres/dim",
    )
}

fn internal_get_conn_custom(
    log: Option<&Logger>,
    main: &str,
    fallback: &str,
) -> Result<diesel::PgConnection, diesel::result::ConnectionError> {
    let conn = PgConnection::establish(main);

    let conn = if conn.is_ok() {
        conn
    } else {
        PgConnection::establish(fallback)
    };

    if conn.is_ok() {
        return Ok(conn?);
    }

    if let Err(e) = conn {
        if let ConnectionError::BadConnection(_) = e {
            let conn = PgConnection::establish("postgres://postgres:dimpostgres@127.0.0.1/");

            let conn = if conn.is_ok() {
                conn
            } else {
                PgConnection::establish("postgres://postgres:dimpostgres@postgres/")
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
