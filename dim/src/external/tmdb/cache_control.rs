use std::sync::Arc;

use tokio::sync::broadcast;

use crate::external::MediaSearchType;

/// The type of our hashmap we use for caching.
///
/// The current implementation is using [flurry](https://docs.rs/flurry)
///
pub(super) type CacheMap = Arc<dashmap::DashMap<CacheKey, Option<CacheValue>>>;

/// The key type used within the [CacheMap], refers to [CacheValue]s.
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum CacheKey {
    /// A search result
    Search { title: String, year: Option<i32> },
    /// Genre List
    GenreList { media_type: MediaSearchType },
}

pub(super) type PendingRequestTx =
    broadcast::Sender<Result<Arc<str>, super::TMDBClientRequestError>>;

/// The value type used within the [CacheMap], refered to by [CacheKey]s.
#[derive(Clone)]

pub(super) enum CacheValue {
    /// The request responsible for fulfilling this data is currently in flight.
    RequestInFlight { tx: PendingRequestTx },
    /// The responses body as UTF-8, cached.
    Body { text: Arc<str> },
}

impl CacheValue {
    /// get the data out of the value, if it is still pending, wait for it and turn errors into None.
    pub(super) async fn data(&self) -> Option<Arc<str>> {
        match self {
            CacheValue::RequestInFlight { tx } => {
                tx.subscribe().recv().await.map(|o| o.ok()).ok().flatten()
            }
            CacheValue::Body { text } => Some(Arc::clone(text)),
        }
    }
}
