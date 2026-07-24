use std::env;
use std::error::Error;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
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

    let git_tag_output = Command::new("git")
        .args(&["describe", "--abbrev=0"])
        .output()
        .unwrap();
    let git_tag = String::from_utf8(git_tag_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_TAG={}", git_tag);

    let git_sha_256_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_sha_256 = String::from_utf8(git_sha_256_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_SHA_256={}", git_sha_256);

    if Path::new("../ui/build").exists() {
        println!("cargo:rustc-cfg=feature=\"embed_ui\"");
    } else {
        println!("cargo:warning=`ui/build` does not exist.");
        println!("cargo:warning=If you wish to embed the webui, run `yarn build` in `ui`.");
    }

    println!("cargo:rerun-if-changed=ui/build");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
