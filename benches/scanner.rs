use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use serde::Deserialize;
use std::fs;
use std::fs::copy;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

const FILES: &'static str = include_str!("./bench_1.txt");
const DEMO: &'static str = "./benches/bench_1.mkv";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Directory,
    File,
    Report,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename = "lowercase", alias = "type")]
    fs_type: EntryType,
    name: Option<String>,
    contents: Option<Vec<Self>>,

    directories: Option<u64>,
    files: Option<u64>,
}

pub fn walk_entry(entry: Entry, mut root: PathBuf) {
    if let Some(name) = entry.name {
        root.push(name);
    }

    match entry.fs_type {
        EntryType::Directory => {
            create_dir_all(&root);
        }
        EntryType::File => {
            let mut link = std::env::current_dir().unwrap();
            link.push(DEMO);
            copy(link, &root);
        }
        _ => {}
    }

    if let Some(contents) = entry.contents {
        for e in contents {
            walk_entry(e, root.clone());
        }
    }
}

fn mount(out_dir: String) {
    let root = PathBuf::from(out_dir);

    let entry: Vec<Entry> = serde_json::from_str(FILES).expect("Got invalid json");

    for e in entry {
        if let EntryType::Report = e.fs_type {
            println!(
                "Written {} directories and {} files.",
                e.directories.unwrap(),
                e.files.unwrap()
            );
            continue;
        }

        walk_entry(e, root.clone());
    }
}

fn invoke_scanner(root: String, tx: dim::core::EventTx) {
    use database::get_conn_devel;
    use database::library::InsertableLibrary;
    use database::library::Library;
    use database::library::MediaType;

    use dim::scanners::tv_show::TvShowScanner;
    use dim::scanners::MediaScanner;

    let mut link = std::env::current_dir().unwrap();
    link.push("bench_out");
    let conn = get_conn_devel().unwrap();

    let library = InsertableLibrary {
        name: "bench".into(),
        location: link.to_str().unwrap().to_string(),
        media_type: MediaType::Tv,
    };

    let id = library.insert(&conn).unwrap();

    let library = Library::get_one(&conn, id).unwrap();

    let scanner = TvShowScanner::new(conn, id, log, tx).unwrap();
    scanner.start(None);

    let conn = get_conn_devel().unwrap();
    Library::delete(&conn, id).unwrap();

    println!("finished scanning");
}

fn bench(c: &mut Criterion) {
    println!("=======Writing dummy files out to the fs======");
    if fs::create_dir("./bench_out").is_ok() {
        mount("./bench_out".into());
    }
    println!("=======DONE=======");

    let mut group = c.benchmark_group("bench");

    group.sample_size(10);
    group.bench_function("scanner", move |b| {
        b.iter_custom(|iters| {
            let logger = dim::build_logger(false);
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            let (tx2, rx2) = std::sync::mpsc::channel();

            dim::core::METADATA_FETCHER_TX.set(dim::core::CloneOnDeref::new(tx2));
            dim::core::METADATA_PATH.set(String::new());

            let start = Instant::now();
            for _ in 0..iters {
                let tx_clone = tx.clone();
                let log_clone = logger.clone();
                invoke_scanner("./bench_out".into(), log_clone, tx_clone);
            }

            let ret = start.elapsed();
            ret
        });
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
