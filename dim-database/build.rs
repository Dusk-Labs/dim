use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    println!(
        "cargo:warning=Generating {:?} from latest migrations.",
        db_file
    );

    let _ = fs::remove_file(&db_file);

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(db_file.as_ref())?.create_if_missing(true),
        )
        .await?;

    sqlx::migrate!().run(&pool).await.map_err(|e| {
        println!("cargo:error=Migration failed: {:?}", e);
        e
    })?;

    println!("cargo:warning=Built database {}.", db_file);

    println!("cargo:rerun-if-changed=database/src/build.rs");
    println!("cargo:rerun-if-changed=database/migrations");

    Ok(())
}
