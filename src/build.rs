use std::env;
use std::path::Path;
use std::process::Command;

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
