use crate::external::MediaSearchType;

use std::sync::Arc;
use tokio::sync::broadcast;

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
    Body {
        text: Arc<str>,
        ttl: std::time::Instant,
    },
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
