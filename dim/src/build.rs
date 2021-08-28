use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::process::Command;

fn main() {
    if Path::new("../ui/build").exists() {
        println!("cargo:rustc-cfg=feature=\"embed_ui\"");
    } else {
        println!("cargo:warning=`ui/build` does not exist.");
        println!("cargo:warning=If you wish to embed the webui, run `yarn build` in `ui`.");
    }

    let mut manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or("./".into()));
    let mut migrations_path = manifest_dir.clone();
    migrations_path.pop();
    manifest_dir.pop();
    manifest_dir.push("dim_dev.db");
    migrations_path.push("database/migrations/*.sql");

    if !Path::new(&manifest_dir).exists() {
        println!("cargo:warning=Generating {:?} from latest migrations.", manifest_dir);
        println!("cargo:warning=Using migrations located at {:?}", migrations_path);
        let process = Command::new("sqlite3")
            .args(&["-init", migrations_path.to_string_lossy().as_ref(), manifest_dir.to_string_lossy().as_ref()])
            .output()
            .expect("Failed to generate dim_dev.db from latest migrations.");

        println!("cargo:warning={}", String::from_utf8_lossy(&process.stdout));
        println!("cargo:warning=Sqlite3 exited with {:?}.", process.status.code());
    }

    println!("cargo:rerun-if-changed=ui/build");
    println!("cargo:rerun-if-changed=build.rs");
}
