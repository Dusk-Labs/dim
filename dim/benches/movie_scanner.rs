use criterion::{criterion_group, criterion_main, Criterion};
use dim_database::get_conn_file;
use dim_database::library::InsertableLibrary;
use dim_database::library::Library;
use dim_database::library::MediaType;

use tokio::runtime;

use std::fs::hard_link;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

const TEST_MP4_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/scanner/tests/data/test.mp4"
);

fn generate_tag() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdef0123456789";

    let mut rng = rand::thread_rng();

    (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn temp_dir<'a>(files: impl IntoIterator<Item = &'a str>) {
    let tempdir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));

    for file in files.into_iter() {
        let file_path = tempdir.as_path().join(file);

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create parent dir");
        }

        let _ = File::create(file_path);
    }
}

pub fn temp_dir_symlink<'a>(
    files: impl Iterator<Item = impl AsRef<str>>,
    target_file: &'a str,
) -> Vec<PathBuf> {
    let tempdir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));

    let mut absolute = vec![];

    for file in files.into_iter() {
        let file_path = tempdir.as_path().join(file.as_ref());

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create parent dir");
        }

        let _ = hard_link(target_file, &file_path);

        absolute.push(file_path);
    }

    absolute
}

async fn bootstrap() -> dim_database::DbConnection {
    let files = (0..128)
        .map(|i| format!("Movie{i}.mkv"))
        .collect::<Vec<String>>();

    let _files = temp_dir_symlink(files.into_iter(), TEST_MP4_PATH);

    let outdir = env!("CARGO_TARGET_TMPDIR");
    let tag = generate_tag();

    let conn = get_conn_file(&format!("{outdir}/dim.{tag}.db"))
        .await
        .unwrap();

    conn
}

async fn create_library(conn: &mut dim_database::DbConnection) -> i64 {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.unwrap();

    let id = InsertableLibrary {
        name: "Tests".to_string(),
        locations: vec![],
        media_type: MediaType::Movie,
    }
    .insert(&mut tx)
    .await
    .expect("Failed to create test library.");

    tx.commit().await.expect("Failed to commit test library.");

    id
}

async fn delete_library(conn: &mut dim_database::DbConnection, id: i64) {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.unwrap();

    Library::delete(&mut tx, id).await.unwrap();

    tx.commit().await.expect("Failed to commit test library.");
}

fn movie_scanner_bench(c: &mut Criterion) {
    c.bench_function("Scan 128 files", move |b| {
        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(8)
            .enable_all()
            .build()
            .unwrap();

        b.to_async(rt).iter_custom(|iters| async move {
            let _path = env!("CARGO_TARGET_TMPDIR").to_string();
            let mut db = bootstrap().await;
            let mut total_elapsed = Duration::ZERO;

            for _ in 0..iters {
                let library_id = create_library(&mut db).await;
                let start = Instant::now();

                // scan_directory(&mut db, library_id, vec![path.clone()]).await;

                total_elapsed += start.elapsed();
                delete_library(&mut db, library_id).await;
            }

            total_elapsed
        })
    });
}

criterion_group!(benches, movie_scanner_bench);
criterion_main!(benches);
