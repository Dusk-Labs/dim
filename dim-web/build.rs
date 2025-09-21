use std::env;
use std::error::Error;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("CARGO_TARGET_DIR").unwrap();

    let db_file = format!("{out_dir}/dim_dev.db");
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
