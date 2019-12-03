use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    /*
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dist");
    println!("{:?}", dest_path);
    std::fs::create_dir(dest_path.clone());
    let _yarn = Command::new("yarn")
        .arg("--cwd")
        .arg("web_ui")
        .arg("build")
        .status()
        .unwrap();

    println!("Built web ui");

    let _cp = Command::new("cp")
        .arg("web_ui/build")
        .arg(dest_path)
        .output()
        .unwrap();

    panic!("Copied files");
    */
}
