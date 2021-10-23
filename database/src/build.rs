use anyhow::*;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().context("loading .env")?;

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR is not set")?);

    let mut db_file = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    if db_file.starts_with("sqlite://") {
        db_file = db_file.split_off(9);
    }

    let mut db_path = manifest_dir.clone();
    db_path.pop();
    db_path.push(&db_file);

    if !Path::new(&db_path).exists() {
        println!(
            "cargo:warning=Generating {:?} from latest migrations.",
            db_file
        );

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::from_str(db_path.to_string_lossy().as_ref())?
                    .create_if_missing(true),
            )
            .await?;

        if let Err(e) = sqlx::migrate!().run(&pool).await {
            println!("cargo:error=Migration failed: {:?}", e);
            return Err(anyhow!("cannot perform migration"));
        }
        println!(
            "cargo:warning=Built database {}.",
            db_path.to_string_lossy().as_ref()
        );
    }

    println!("cargo:rerun-if-changed=ui/build");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
