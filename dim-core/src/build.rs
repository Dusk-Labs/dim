use std::env;

fn main() {
    let out_dir = env::var("CARGO_TARGET_DIR").unwrap();
    let db_file = format!("{out_dir}/dim_dev.db");
    println!("cargo:rustc-env=DATABASE_URL=sqlite://{db_file}");

    println!("cargo:rerun-if-changed=build.rs");
}
