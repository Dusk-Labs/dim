use std::env;
use std::path::Path;

fn main() {
    // CARGO_TARGET_DIR is not set by Cargo automatically; fall back to deriving
    // the target directory from OUT_DIR (target/<profile>/build/<crate>/out).
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
        let out = env::var("OUT_DIR").expect("OUT_DIR not set");
        Path::new(&out)
            .ancestors()
            .nth(4)
            .expect("OUT_DIR has unexpected structure")
            .to_string_lossy()
            .into_owned()
    });
    let db_file = format!("{target_dir}/dim_dev.db");
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
