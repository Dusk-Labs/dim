// #![deny(warnings)]

use std::future::Future;
use std::marker::PhantomData;

use async_trait::async_trait;

use displaydoc::Display;

use tokio::sync::broadcast;

use parking_lot::RwLock;

use super::{Result as QueryResult, *};
use core::result::Result;

/// The User-Agent is generated from the package name "dim" and the version so "dim/0.x.y.z"
pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// The base url used to access TMDB;
pub const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

// -- TMDB API Data Models

#[derive(Deserialize, Clone, Debug)]
struct SearchResponse {
    results: Vec<Option<TMDBMediaObject>>,
}

#[derive(Deserialize, Clone, Debug)]
struct TMDBMediaObject {
    pub id: u64,
    #[serde(rename(deserialize = "title", deserialize = "name"))]
    pub title: String,
    #[serde(rename(deserialize = "release_date", deserialize = "first_air_date"))]
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub vote_average: Option<f64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<u64>>,
    #[serde(skip_deserializing)]
    pub genres: Vec<String>,
    pub runtime: Option<u64>,
}

// -- TMDBClient

#[derive(Debug, Display, Clone, thiserror::Error)]
enum TMDBClientRequestError {
    /// The body of a response was not value UTF-8.
    InvalidUTF8Body,
    /// the error comes from reqwest.
    ReqwestError(#[from] Arc<reqwest::Error>),
}

/// Internal TMDB client type used for building and making requests.
struct TMDBClient {
    provider: TMDBMetadataProvider,
}

impl TMDBClient {
    async fn genre_detail(&self, genre_id: u64) -> Result<String, TMDBClientRequestError> {
        let url = format!("{TMDB_BASE_URL}/genre/{genre_id}/list");
        let args: Vec<_> = vec![("api_key", self.provider.api_key.as_ref())]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let req = self
            .provider
            .http_client
            .get(url)
            .query(&args)
            .send()
            .await
            .map_err(|err| TMDBClientRequestError::ReqwestError(Arc::new(err)))?;

        let body = req
            .bytes()
            .await
            .map_err(|err| TMDBClientRequestError::ReqwestError(Arc::new(err)))?;

        std::str::from_utf8(&body)
            .map_err(|_| TMDBClientRequestError::InvalidUTF8Body)
            .map(|st| st.to_string())
    }

    async fn search(
        &self,
        media_type: MediaType,
        title: &str,
        year: Option<i32>,
    ) -> Result<String, TMDBClientRequestError> {
        let url = format!("{TMDB_BASE_URL}/search/{media_type}");
        let args: Vec<_> = vec![
            ("api_key", self.provider.api_key.as_ref()),
            ("language", "en-US"),
            ("query", title),
            ("page", "1"),
            ("include_adult", "false"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .chain(
            year.into_iter()
                .map(|n| ("year".to_string(), n.to_string())),
        )
        .collect();

        let req = self
            .provider
            .http_client
            .get(url)
            .query(&args)
            .send()
            .await
            .map_err(|err| TMDBClientRequestError::ReqwestError(Arc::new(err)))?;

        let body = req
            .bytes()
            .await
            .map_err(|err| TMDBClientRequestError::ReqwestError(Arc::new(err)))?;

        std::str::from_utf8(&body)
            .map_err(|_| TMDBClientRequestError::InvalidUTF8Body)
            .map(|st| st.to_string())
    }
}

// -- cache control

/// The type of our hashmap we use for caching.
///
/// The current implementation is using [flurry](https://docs.rs/flurry)
///
type CacheMap = Arc<dashmap::DashMap<CacheKey, RwLock<Option<CacheValue>>>>;

/// The key type used within the [CacheMap], refers to [CacheValue]s.
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CacheKey {
    /// A search result
    Search { title: String, year: Option<i32> },
}

type PendingRequestTx = broadcast::Sender<Result<Arc<str>, TMDBClientRequestError>>;

/// The value type used within the [CacheMap], refered to by [CacheKey]s.
#[derive(Clone)]
enum CacheValue {
    /// The request responsible for fulfilling this data is currently in flight.
    RequestInFlight { tx: PendingRequestTx },
    /// The responses body as UTF-8, cached.
    Body { text: Arc<str> },
}

impl CacheValue {
    /// get the data out of the value, if it is still pending, wait for it and turn errors into None.
    async fn data(&self) -> Option<Arc<str>> {
        match self {
            CacheValue::RequestInFlight { tx } => {
                tx.subscribe().recv().await.map(|o| o.ok()).ok().flatten()
            }
            CacheValue::Body { text } => Some(Arc::clone(text)),
        }
    }
}

// -- TMDBMetadataProvider

/// TMDB Metadata Provider implements `ExternalQuery` and handles request coalescing and caching locally.
pub struct TMDBMetadataProvider {
    api_key: Arc<str>,
    http_client: reqwest::Client,
    cache: CacheMap,
}

impl Clone for TMDBMetadataProvider {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            http_client: self.http_client.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl TMDBMetadataProvider {
    pub fn new(api_key: String) -> Self {
        let http_client = reqwest::ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("building this client should never fail.");

        let api_key: Arc<str> = api_key.into_boxed_str().into();

        Self {
            api_key,
            http_client,
            cache: Default::default(),
        }
    }

    /// insert a default [CacheValue] if the slot at a given key is not present.
    fn insert_value_if_empty(&self, key: &CacheKey) -> (CacheValue, bool) {
        // grab the entry or instert RwLock::new(None) if not present.
        let entry = self.cache.entry(key.clone()).or_default();

        // fast path: cache hits, no writers and the value is present.
        {
            let read_guard = entry.value().read();
            if let Some(value) = read_guard.as_ref() {
                return (value.clone(), false);
            }
        }

        // slow path: get a write guard, if the slot is still uninit when we acquire; initialize it.s
        let mut slot = entry.value().write();

        match slot.as_ref() {
            // someone initialized the slot before we got the write guard, use their value.
            Some(value) => (value.clone(), false),
            // we're still first, initialize the value and keep going.
            None => {
                let (tx, _) = broadcast::channel(1);
                let value = CacheValue::RequestInFlight { tx };
                slot.replace(value.clone());
                (value, true)
            }
        }
    }

    /// perform request coalescing; when two futures are made with the same key the duplicates wait for the original to broadcast the results.
    async fn coalesce_request<F, Fut>(
        &self,
        key: &CacheKey,
        make_request_future: F,
    ) -> Result<Arc<str>, Error>
    where
        F: FnOnce(TMDBClient) -> Fut,
        Fut: Future<Output = Result<Arc<str>, TMDBClientRequestError>> + Send + 'static,
    {
        let (value, we_own_future) = { self.insert_value_if_empty(key) };

        match (we_own_future, value) {
            // only if we own the future that was spawned should we upate the
            // value once we get the response.
            (true, CacheValue::RequestInFlight { tx }) => {
                let client = TMDBClient {
                    provider: self.clone(),
                };

                let tx_ = tx.clone();
                let fut = make_request_future(client);
                tokio::spawn(async move {
                    let output = fut.await;
                    let _ = tx_.send(output);
                });

                match { CacheValue::RequestInFlight { tx } }.data().await {
                    Some(text) => {
                        let body = Arc::clone(&text);

                        let _ = match self.cache.get(key) {
                            Some(rw_lock) => rw_lock.write().replace(CacheValue::Body { text }),
                            None => unreachable!("slot was None when original task got its result"),
                        };

                        Ok(body)
                    }
                    // if an error was yeeted back during the request then purge the cache entry.
                    None => {
                        // clear the cache for values that could not be populated.
                        let _ = self.cache.remove(&key);
                        Err(Error::Timeout)
                    }
                }
            }

            (_, value) => value.data().await.ok_or(Error::Timeout),
        }
    }

    async fn search(
        &self,
        title: &str,
        year: Option<i32>,
        media_type: MediaType,
    ) -> QueryResult<Vec<ExternalMedia>> {
        let title = title.to_string();
        let key = CacheKey::Search {
            title: title.clone(),
            year,
        };

        let st = self
            .coalesce_request(&key, |client| async move {
                let body = client
                    .search(media_type, &title, year)
                    .await
                    .map(|st| st.into_boxed_str().into());

                body
            })
            .await?;

        let search = serde_json::from_str::<SearchResponse>(&st).map_err(Error::other)?;

        for media in search.results.iter_mut() {
            if let Some(media_object) = media {
                let key = CacheKey::GenreDetail {
                    id: media_object.genre_ids,
                };

                self.coalesce_request(key, make_request_future)
            }
        }

        todo!()
    }

    async fn search_by_id(&self, external_id: &str) -> QueryResult<ExternalMedia> {
        todo!()
    }

    async fn actors(&self, external_id: &str) -> QueryResult<Vec<ExternalActor>> {
        todo!()
    }
}

// -- TMDBMetadataProviderRef<T>

/// Used to key [TMDBMetadataProviderRef] to search for TV shows, compliments [Movies].
pub struct TvShows;

/// Used to key [TMDBMetadataProviderRef] to search for movies, compliments [TvShows].
pub struct Movies;

/// An instance of [TMDBMetadataProvider] with a generic parameter to infer the [MediaType] for searches.
pub struct TMDBMetadataProviderRef<K>
where
    K: Send + Sync + 'static,
{
    pub provider: TMDBMetadataProvider,
    _key: PhantomData<K>,
}

mod sealed {
    use super::{MediaType, Movies, TvShows};

    /// Used to associate a constant [MediaType] with another type.
    pub trait AssocMediaTypeConst {
        const MEDIA_TYPE: MediaType;
    }

    impl AssocMediaTypeConst for TvShows {
        const MEDIA_TYPE: MediaType = MediaType::Tv;
    }

    impl AssocMediaTypeConst for Movies {
        const MEDIA_TYPE: MediaType = MediaType::Tv;
    }
}

#[async_trait]
impl<K> ExternalQuery for TMDBMetadataProviderRef<K>
where
    K: sealed::AssocMediaTypeConst + Send + Sync + 'static,
{
    async fn search(&self, title: &str, year: Option<i32>) -> QueryResult<Vec<ExternalMedia>> {
        self.provider.search(title, year, K::MEDIA_TYPE).await
    }

    async fn search_by_id(&self, external_id: &str) -> QueryResult<ExternalMedia> {
        todo!()
    }

    async fn actors(&self, external_id: &str) -> QueryResult<Vec<ExternalActor>> {
        todo!()
    }
}

#[async_trait]
impl<K> ExternalQueryShow for TMDBMetadataProviderRef<K>
where
    K: sealed::AssocMediaTypeConst + Send + Sync + 'static,
{
    async fn seasons_for_id(&self, external_id: &str) -> QueryResult<Vec<ExternalSeason>> {
        todo!()
    }

    async fn episodes_for_season(&self, season_id: &str) -> QueryResult<Vec<ExternalEpisode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sanity_check_tmdb_works() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e".into());
        let provider_shows: TMDBMetadataProviderRef<TvShows> = TMDBMetadataProviderRef {
            provider,
            _key: PhantomData,
        };

        provider_shows
            .search("letterkenny", None)
            .await
            .expect("search results should exist");
    }
}
