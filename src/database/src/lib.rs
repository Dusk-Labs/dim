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
