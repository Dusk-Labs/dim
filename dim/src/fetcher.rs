use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::time::Duration;

use crate::core::*;

use slog::debug;
use slog::error;
use slog::Logger;

use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use std::collections::HashSet;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::path::PathBuf;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PosterType {
    Banner(String),
    Season(String),
    Episode(String),
}

impl PartialOrd for PosterType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PosterType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PosterType::Banner(_), PosterType::Banner(_)) => Ordering::Equal,
            (PosterType::Banner(_), PosterType::Season(_)) => Ordering::Greater,
            (PosterType::Banner(_), PosterType::Episode(_)) => Ordering::Greater,
            (PosterType::Season(_), PosterType::Banner(_)) => Ordering::Less,
            (PosterType::Season(_), PosterType::Season(_)) => Ordering::Equal,
            (PosterType::Season(_), PosterType::Episode(_)) => Ordering::Greater,
            (PosterType::Episode(_), PosterType::Banner(_)) => Ordering::Less,
            (PosterType::Episode(_), PosterType::Season(_)) => Ordering::Less,
            (PosterType::Episode(_), PosterType::Episode(_)) => Ordering::Equal,
        }
    }
}

impl From<PosterType> for String {
    fn from(poster: PosterType) -> Self {
        match poster {
            PosterType::Banner(st) | PosterType::Season(st) | PosterType::Episode(st) => st,
        }
    }
}

use std::lazy::SyncLazy;
use tokio::sync::Mutex;
use std::collections::BinaryHeap;

static PROCESSING_QUEUE: SyncLazy<Mutex<BinaryHeap<PosterType>>> = SyncLazy::new(|| Mutex::new(BinaryHeap::new()));

async fn process_receiver(log: Logger, mut rx: UnboundedReceiver<PosterType>) {
    let mut poster_cache = HashSet::<PosterType>::new();

    while let Some(poster) = rx.recv().await {
        if !poster_cache.contains(&poster) {
            debug!(log, "Inserting {:?} into queue", poster);
            {
                let mut lock = PROCESSING_QUEUE.lock().await;
                lock.push(poster.clone());
            }
            poster_cache.insert(poster);
        }
    }
}

async fn process_queue(log: Logger) {
    let mut timer = tokio::time::interval(Duration::from_millis(100));

    loop {
        // sleep for a bit to avoid constantly locking the queue.
        timer.tick().await;

        let mut lock = PROCESSING_QUEUE.lock().await;
        while let Some(poster) = lock.pop() {
            let url: String = poster.clone().into();

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
                    error!(log, "Failed to cache {} locally, appending back into queue", &url);
                    lock.push(poster);
                }
                Err(e) => {
                    error!(log, "Failed to cache {} locally, e={:?}", url, e);
                    lock.push(poster);
                },
            }
        }

        assert!(lock.is_empty());

        tokio::task::yield_now().await;
    }
}

/// Function creates a task that fetches and caches posters from various sources.
pub async fn tmdb_poster_fetcher(log: Logger) {
    let (tx, rx): (UnboundedSender<PosterType>, UnboundedReceiver<PosterType>) = 
                       unbounded_channel();

    tokio::spawn(process_receiver(log.clone(), rx));
    tokio::spawn(process_queue(log.clone()));

    METADATA_FETCHER_TX
        .set(CloneOnDeref::new(tx))
        .expect("Failed to set METADATA_FETCHER_TX");
}
