#![feature(rustc_private)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate rocket_contrib;

pub mod core;
pub mod macros;
pub mod routes;
pub mod schema;
pub mod tests;

const VERSION: &str = "0.0.3";

fn main() {
    println!("Running Dim v{}", VERSION);
    core::launch();
}
