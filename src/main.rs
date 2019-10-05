#![feature(rustc_private)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate lazy_static;

extern crate dim_database;
extern crate dim_scanners;
extern crate dim_streamer;
extern crate dotenv;
extern crate log;
extern crate rocket;
extern crate rocket_slog;
extern crate serde;
extern crate slog;
extern crate sloggers;

pub mod routes;
pub mod schema;
#[macro_use]
pub mod macros;
pub mod core;
pub mod tests;

const VERSION: &str = "0.0.3";

fn main() {
    println!("Running Dim v{}", VERSION);
    core::launch();
}
