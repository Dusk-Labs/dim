use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("CARGO_TARGET_DIR").unwrap();
    let db_file = format!("{out_dir}/dim_dev.db");
    println!("cargo:rustc-env=DATABASE_URL=sqlite://{db_file}");

    if Path::new("../ui/build").exists() {
        println!("cargo:rustc-cfg=feature=\"embed_ui\"");
    } else {
        println!("cargo:warning=`ui/build` does not exist.");
        println!("cargo:warning=If you wish to embed the webui, run `yarn build` in `ui`.");
    }

    println!("cargo:rerun-if-changed=ui/build");
    println!("cargo:rerun-if-changed=build.rs");
}
