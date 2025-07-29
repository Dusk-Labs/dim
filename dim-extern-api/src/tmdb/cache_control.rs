use crate::MediaSearchType;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use tokio::sync::broadcast;
use tokio::task::spawn_blocking;
use tokio::task::JoinHandle;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// How long we should sleep in-between cache evictions. Defaults to 15 seconds.
const EVICT_EVERY: Duration = Duration::from_millis(15_000);

/// The type of our hashmap we use for caching.
pub(super) type CacheMap = Arc<dashmap::DashMap<CacheKey, Option<CacheValue>>>;

/// The key type used within the [CacheMap], refers to [CacheValue]s.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum CacheKey {
    /// A search result
    Search { title: String, year: Option<i32> },
    /// Genre List
    GenreList { media_type: MediaSearchType },
    /// Searching by ID
    ById { id: String, ty: MediaSearchType },
    /// Search for an actor by id
    ActorById { id: String },
    /// Get all episodes for a season
    Episodes { id: String, season_number: u64 },
}

pub(super) type PendingRequestTx =
    broadcast::Sender<Result<Arc<str>, super::TMDBClientRequestError>>;

/// The value type used within the [CacheMap], refered to by [CacheKey]s.
#[derive(Clone)]
pub(super) enum CacheValue {
    /// The request responsible for fulfilling this data is currently in flight.
    RequestInFlight { tx: PendingRequestTx },
    /// The responses body as UTF-8, cached. This also has a TTL. Once the TTL is reached, the
    /// value should be ignored/discarded.
    Body { text: Arc<str>, ttl: Instant },
}

impl CacheValue {
    /// get the data out of the value, if it is still pending, wait for it and turn errors into None.
    pub(super) async fn data(&self) -> Result<Arc<str>, super::TMDBClientRequestError> {
        match self {
            CacheValue::RequestInFlight { tx } => tx
                .subscribe()
                .recv()
                .await
                .map_err(super::TMDBClientRequestError::RecvError)?,

            CacheValue::Body { text, .. } => Ok(Arc::clone(text)),
        }
    }

    pub fn mem_size(&self) -> usize {
        let body_size = match self {
            Self::Body { text, .. } => text.as_bytes().len(),
            _ => 0,
        };

        core::mem::size_of::<CacheValue>() + body_size
    }
}

/// Simple worker which handles cache eviction. It doesn't promise that our memory usage will
/// always be below our max target, but it does promise eventual consistency.
///
/// The policy is that expelling TTL'd items should be handled lazily, however it will randomly
/// evict items when our usage reaches our max.
pub(super) struct CacheEviction {
    cache: CacheMap,
    usage: Arc<AtomicUsize>,
    max_usage: usize,
}

impl CacheEviction {
    pub(super) fn new(cache: CacheMap, usage: Arc<AtomicUsize>, max_usage: usize) -> Self {
        Self {
            cache,
            usage,
            max_usage,
        }
    }

    pub(super) fn start_policy(self) -> AbortOnDropHandle {
        let (stop_tx, mut rx) = broadcast::channel(1);

        let handle = spawn_blocking(move || {
            while matches!(rx.try_recv(), Err(broadcast::error::TryRecvError::Empty)) {
                let now = Instant::now();
                let items = self.evict();
                let duration = now.elapsed().as_millis();

                tracing::debug!(
                    items = items,
                    duration_ms = duration,
                    "tmdb cache eviction finished."
                );

                thread::sleep(EVICT_EVERY);
            }
        });

        AbortOnDropHandle { handle, stop_tx }
    }

    fn evict(&self) -> usize {
        if self.usage.fetch_min(self.max_usage, Ordering::Relaxed) < self.max_usage {
            return 0;
        }

        // This is probably gonna be very slow as our cache grows.
        // we want to evict 5% of our items.
        // FIXME: Absolutely not ideal
        let cache_len = self.cache.len();
        let to_delete = cache_len / 20;

        let mut rng = SmallRng::from_entropy();

        self.cache.retain(|_, v| {
            let size = if let Some(ref v) = v {
                v.mem_size()
            } else {
                return true;
            };

            if size == 0 {
                return true;
            }

            if rng.gen_range(0..=cache_len) < to_delete {
                self.usage.fetch_sub(size, Ordering::Relaxed);
                return false;
            }

            true
        });

        let new_cache_len = self.cache.len();

        cache_len - new_cache_len
    }
}

pub struct AbortOnDropHandle {
    handle: JoinHandle<()>,
    stop_tx: broadcast::Sender<()>,
}

impl Drop for AbortOnDropHandle {
    fn drop(&mut self) {
        let _ = self.stop_tx.send(());
        self.handle.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_eviction() {
        let cache = CacheMap::default();
        // pretend we already have 100 items in the cache.
        let mut usage = 0;

        for i in 0..=10000 {
            let value = CacheValue::Body {
                text: format!("{i}").into(),
                ttl: Instant::now(),
            };

            usage += value.mem_size();

            cache.insert(
                CacheKey::Search {
                    title: format!("{i}").into(),
                    year: None,
                },
                Some(value),
            );
        }

        let usage = Arc::new(AtomicUsize::new(usage));
        let policy = CacheEviction::new(cache.clone(), usage.clone(), 0);
        let evicted = policy.evict();

        assert!(evicted < 10000);
        assert!(evicted > 0);
    }
}
