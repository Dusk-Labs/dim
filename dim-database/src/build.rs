use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("CARGO_TARGET_DIR").unwrap();

    let db_file = format!("{out_dir}/dim_dev.db");
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
