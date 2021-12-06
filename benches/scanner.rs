use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::path::PathBuf;
use std::time::Instant;

use dim::scanners::*;

const MEDIA_DIRECTORY: &str = "/home/hinach4n/media/anime";

async fn invoke_scanner(root: String, tx: dim::core::EventTx, files: &[PathBuf]) -> usize {
    use database::get_conn;
    use database::get_conn_devel;
    use database::library::InsertableLibrary;
    use database::library::Library;
    use database::library::MediaType;
    use database::set_conn;

    let conn = get_conn().await.unwrap();

    let extractor = get_extractor(&tx);
    let matcher = get_matcher(&tx);

    let library = InsertableLibrary {
        name: "bench".into(),
        locations: vec![MEDIA_DIRECTORY.to_string()],
        media_type: MediaType::Tv,
    };

    let mut tx = conn.write().begin().await.unwrap();
    let id = library.insert(&mut tx).await.unwrap();
    tx.commit().await.unwrap();

    let mut futures = Vec::new();

    for file in files {
        futures.push(async move {
            if let Ok(mfile) = extractor
                .mount_file(file.to_path_buf(), id, MediaType::Tv)
                .await
            {
                matcher.match_tv(mfile).await.unwrap();
            }
        })
    }

    futures::future::join_all(futures).await;

    let mut tx = conn.write().begin().await.unwrap();
    Library::delete(&mut tx, id).await.unwrap();
    tx.commit().await.unwrap();

    files.len()
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench");

    group.sample_size(10);
    group.bench_function("scanner", move |b| {
        b.iter_custom(|iters| {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();

            let files = rt
                .block_on(async {
                    use database::*;

                    let conn = get_conn_devel().await.unwrap();
                    set_conn(conn);
                    get_subfiles([MEDIA_DIRECTORY].into_iter()).await
                })
                .unwrap();

            let start = Instant::now();
            for _ in 0..iters {
                let tx_clone = tx.clone();
                rt.block_on(invoke_scanner("./bench_out".into(), tx_clone, &files));
            }

            let ret = start.elapsed();
            println!("finished scanning {} files", files.len());
            ret
        });
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
