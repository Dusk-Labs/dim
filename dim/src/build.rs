use std::path::Path;
use std::process::Command;

fn main() {
    if Path::new("../ui/build").exists() {
        println!("cargo:rustc-cfg=feature=\"embed_ui\"");
    } else {
        println!("cargo:warning=`ui/build` does not exist.");
        println!("cargo:warning=If you wish to embed the webui, run `yarn build` in `ui`.");
    }

    if !Path::new("./dim_dev.db").exists() {
        println!("cargo:warning=Generating dim_dev.db from latest migrations.");
        Command::new("sqlite3")
            .args(&["-init", "database/migrations/*.sql", "dim_dev.db"])
            .output()
            .expect("Failed to generate dim_dev.db from latest migrations.");
    }

    println!("cargo:rerun-if-changed=ui/build");
}
