#![feature(rustc_private, once_cell, async_closure, box_syntax)]
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

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use slog::Logger;

use std::lazy::SyncOnceCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

pub mod episode;
pub mod error;
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

pub use crate::error::DatabaseError;

#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("Features sqlite and postgres are mutually exclusive");

cfg_if! {
    if #[cfg(feature = "sqlite")] {
        pub type Manager = ConnectionManager<diesel::SqliteConnection>;
        pub type DbConnection = Pool<Manager>;

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
        pub type Manager = ConnectionManager<diesel::PgConnection>;
        pub type DbConnection = Pool<Manager>;
    }
}

lazy_static::lazy_static! {
    static ref MIGRATIONS_FLAG: AtomicBool = AtomicBool::new(false);
}

static __GLOBAL: SyncOnceCell<crate::DbConnection> = SyncOnceCell::new();

#[derive(Debug)]
struct Pragmas;

#[cfg(not(feature = "postgres"))]
impl<E> diesel::r2d2::CustomizeConnection<diesel::SqliteConnection, E> for Pragmas {
    fn on_acquire(&self, conn: &mut diesel::SqliteConnection) -> Result<(), E> {
        use diesel::connection::Connection;
        conn.execute("PRAGMA busy_timeout=5000").unwrap();
        conn.execute("PRAGMA journal_mode=wal").unwrap();
        conn.execute("PRAGMA synchronous=NORMAL").unwrap();
        conn.execute("PRAGMA wal_checkpoint(FULL)").unwrap();
        conn.execute("PRAGMA wal_autocheckpoint = 1000").unwrap();
        conn.execute("PRAGMA foreign_keys = ON").unwrap();

        Ok(())
    }
}

cfg_if! {
    if #[cfg(feature = "postgres")] {
        embed_migrations!("../../migrations/postgres");
    } else {
        embed_migrations!("../../migrations/sqlite");
    }
}

fn create_database(_conn: &crate::DbConnection) -> Result<(), diesel::result::Error> {
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            use crate::diesel::RunQueryDsl;
            let conn = _conn.get().unwrap();
            let _ = diesel::sql_query("CREATE DATABASE dim").execute(&conn)?;
            let _ = diesel::sql_query("CREATE DATABASE dim_devel").execute(&conn)?;
            let _ = diesel::sql_query("CREATE DATABASE pg_trgm").execute(&conn)?;
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
    let conn = conn.get().unwrap();
    embedded_migrations::run(&conn)
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
pub fn get_conn() -> Result<crate::DbConnection, r2d2::Error> {
    let conn = __GLOBAL.get_or_try_init(|| -> Result<_, _> { internal_get_conn(None) })?;

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) {
        if let Err(err) = run_migrations(conn) {
            dbg!(err);
        } else {
            MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
        }
    }

    Ok(conn.clone())
}

/// Function returns a connection to the development table of dim. This is mainly used for unit
/// tests.
pub fn get_conn_devel() -> Result<crate::DbConnection, r2d2::Error> {
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            let pool = internal_get_conn_custom(
                None,
                "postgres://postgres:dimpostgres@127.0.0.1/dim_devel",
            )?;
        } else {
            let manager = Manager::new("./dim_dev.db");
            let pool = Pool::builder()
                .max_size(1)
                .min_idle(Some(1))
                .connection_customizer(Box::new(Pragmas))
                .build(manager)?;
        }
    }

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && run_migrations(&pool).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(pool)
}

/// Function which returns a Result<T, E> where T is a new connection session or E is a connection
/// error. It takes in a logger instance.
///
/// # Arguments
/// * `log` - a Slog logger instance
pub fn get_conn_logged(log: &Logger) -> Result<DbConnection, r2d2::Error> {
    // This is the URL for the database inside a docker container
    let conn = __GLOBAL.get_or_try_init(|| -> Result<_, _> { internal_get_conn(Some(log)) })?;
    slog::info!(log, "Creating new database connection");

    if !MIGRATIONS_FLAG.load(Ordering::SeqCst) && dbg!(run_migrations(&conn)).is_ok() {
        MIGRATIONS_FLAG.store(true, Ordering::SeqCst);
    }

    Ok(conn.clone())
}

fn internal_get_conn(_log: Option<&Logger>) -> Result<DbConnection, r2d2::Error> {
    cfg_if! {
        if #[cfg(feature = "postgres")] {
            internal_get_conn_custom(
                _log,
                "postgres://postgres:dimpostgres@127.0.0.1/dim"
            )
        } else {
            let manager = Manager::new("./dim.db");
            // FIXME: Theres a weird bug with `Pool` or `sqlite` where the pragmas above are not respected and are ignored.
            // This yields database errors at runtime.
            let pool = Pool::builder()
                .max_size(1)
                .min_idle(Some(1))
                .connection_customizer(Box::new(Pragmas))
                .build(manager)?;

            Ok(pool)
        }
    }
}

#[allow(dead_code)]
fn internal_get_conn_custom(
    log: Option<&Logger>,
    main: &str,
) -> Result<DbConnection, r2d2::Error> {
    let manager = Manager::new(main);
    let pool = Pool::builder().build_unchecked(manager);

    if pool.get_timeout(Duration::from_millis(2000)).is_ok() {
        return Ok(pool);
    }

    let manager = Manager::new("postgres://postgres:dimpostgres@127.0.0.1/");
    let pool = Pool::builder().build(manager);

    if let Some(log) = log {
        slog::warn!(
            log,
            "Database dim seems to not exist, creating...standby..."
        );
    }
    let _ = create_database(&pool?);

    Ok(internal_get_conn(log)?)
}
