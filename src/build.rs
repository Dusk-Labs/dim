use fs_extra::dir::{copy, CopyOptions};
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

    /*unneeded for now as rust_embed doesnt work relative to OUT_DIR
    let _ = std::fs::create_dir(dest_path.clone());
    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: false,
        depth: 0,
    };
    copy("web_ui/build", dest_path, &options).expect("Build.rs failed at copying web_ui/build dir");
    println!("Copied web files");
    */
}
