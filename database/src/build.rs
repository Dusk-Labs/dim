use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // The env vars are not necessarily provided by a .env file, so ignore err.
    dotenv::dotenv().ok();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| {
        println!("cargo:error=CARGO_MANIFEST_DIR is not set");
        e
    })?);

    let mut db_file = env::var("DATABASE_URL").map_err(|e| {
        println!("cargo:error=DATABASE_URL is not set");
        e
    })?;

    if db_file.starts_with("sqlite://") {
        db_file = db_file.split_off(9);
    }

    let mut db_path = manifest_dir.clone();
    db_path.pop();
    db_path.push(&db_file);

    println!(
        "cargo:warning=Generating {:?} from latest migrations.",
        db_file
    );

    let _ = fs::remove_file(db_path.to_string_lossy().as_ref());
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(db_path.to_string_lossy().as_ref())?
                .create_if_missing(true),
        )
        .await?;

    sqlx::migrate!().run(&pool).await.map_err(|e| {
        println!("cargo:error=Migration failed: {:?}", e);
        e
    })?;

    println!(
        "cargo:warning=Built database {}.",
        db_path.to_string_lossy().as_ref()
    );

    println!("cargo:rerun-if-changed=database/src/build.rs");
    println!("cargo:rerun-if-changed=database/migrations");

    Ok(())
}
