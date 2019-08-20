#![feature(rustc_private)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde;

pub mod schema;
pub mod episode;
pub mod library;
pub mod media;
pub mod season; 
pub mod tv;
pub mod mediafile;

pub fn get_conn() -> Result<diesel::PgConnection, diesel::result::ConnectionError> {
    use diesel::pg::PgConnection;
    use diesel::connection::Connection;
    let conn = PgConnection::establish("postgres://postgres:dimpostgres@postgres/dim");

    if conn.is_ok() {
        conn
    } else {
        PgConnection::establish("postgres://postgres:dimpostgres@127.0.0.1/dim")
    }
}
