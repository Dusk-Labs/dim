#[cfg(not(debug_assertions))]
use std::{env, path::Path, process::Command};

#[cfg(debug_assertions)]
fn main() {}

/// Build binary for release binaries automatically builds the web ui that gets embedded within dim
#[cfg(not(debug_assertions))]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dist");
    println!("{:?}", dest_path);

    let _ = Command::new("yarn")
        .arg("--cwd")
        .arg("ui")
        .status()
        .unwrap();

    let build_log = Command::new("yarn")
        .arg("--cwd")
        .arg("ui")
        .arg("build")
        .status()
        .unwrap();

    println!("Built web ui");
}
