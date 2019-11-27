#![feature(rustc_private)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate rocket_contrib;

use clap::{App, Arg};

pub mod core;
pub mod macros;
pub mod routes;
pub mod schema;
pub mod tests;

const VERSION: &str = "0.0.3";
const DESCRIPTION: &str = "Dim, a media manager fueled by dark forces.";

fn main() {
    let matches = App::new("Dim")
        .version(VERSION)
        .about(DESCRIPTION)
        .author("Valerian G.")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debug mode? Print all logs to stdout"),
        );

    let matches = matches.get_matches();
    let debug = if cfg!(debug_assertions) {
        matches.is_present("debug") || true
    } else {
        matches.is_present("debug")
    };

    core::launch(debug);
}
