#[cfg(not(debug_assertions))]
use {std::env, std::path::Path, std::process::Command};

#[cfg(debug_assertions)]
fn main() {}

/// Build binary for release binaries automatically builds the web ui that gets embedded within dim
#[cfg(not(debug_assertions))]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dist");
    println!("{:?}", dest_path);

    let _yarn = Command::new("yarn")
        .arg("--cwd")
        .arg("web_ui")
        .arg("build")
        .status()
        .unwrap();

    println!("Built web ui");
}
