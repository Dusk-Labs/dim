use crate::core::*;

use slog::debug;
use slog::error;
use slog::Logger;

use priority_queue::PriorityQueue;
use tokio::sync::Mutex;

use std::collections::HashSet;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::path::PathBuf;
use std::time::Duration;

use once_cell::sync::Lazy;

static PROCESSING_QUEUE: Lazy<Mutex<PriorityQueue<String, usize>>> =
    Lazy::new(|| Mutex::new(Default::default()));
static POSTER_CACHE: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(Default::default()));

pub async fn insert_into_queue(log: &Logger, poster: String, priority: usize) {
    let mut cache_lock = POSTER_CACHE.lock().await;

    if !cache_lock.contains(&poster) {
        debug!(log, "Inserting {:?} into queue", poster);
        {
            let mut lock = PROCESSING_QUEUE.lock().await;
            lock.push(poster.clone(), priority);
        }
        cache_lock.insert(poster);
    }
}

pub async fn bump_priority(log: &Logger, poster: String, priority: usize) {
    debug!(log, "Bumping priority of {:?} to {}", &poster, priority);
    let mut lock = PROCESSING_QUEUE.lock().await;
    lock.push_increase(poster, priority);
}

async fn process_queue(log: Logger) {
    loop {
        let mut lock = PROCESSING_QUEUE.lock().await;
        if lock.is_empty() {
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        }

        if let Some((url, priority)) = lock.pop() {
            debug!(log, "Trying to cache {}", url);
            match reqwest::get(url.as_str()).await {
                Ok(resp) => {
                    if let Some(fname) = resp.url().path_segments().and_then(|segs| segs.last()) {
                        let meta_path = METADATA_PATH.get().unwrap();
                        let mut out_path = PathBuf::from(meta_path);
                        out_path.push(fname);

                        debug!(log, "Caching {} -> {:?}", url, out_path);

                        if let Ok(mut file) = File::create(out_path) {
                            if let Ok(bytes) = resp.bytes().await {
                                let mut content = Cursor::new(bytes);
                                if copy(&mut content, &mut file).is_ok() {
                                    continue;
                                }
                            }
                        }
                    }
                    error!(
                        log,
                        "Failed to cache {} locally, appending back into queue", &url
                    );
                    lock.push(url, priority);
                }
                Err(e) => {
                    error!(log, "Failed to cache {} locally, e={:?}", url, e);
                    lock.push(url, priority);
                }
            }
        }

        tokio::task::yield_now().await;
    }
}

/// Function creates a task that fetches and caches posters from various sources.
pub async fn tmdb_poster_fetcher(log: Logger) {
    tokio::spawn(process_queue(log.clone()));
}
