use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::str::FromStr;

use async_trait::async_trait;

use tokio::sync::broadcast;

use crate::external::{Result as QueryResult, *};
use core::result::Result;

use super::cache_control::{CacheKey, CacheMap, CacheValue};
use super::raw_client::TMDBClient;
use super::*;

/// TMDB Metadata Provider implements `ExternalQuery` and handles request coalescing and caching locally.
pub struct TMDBMetadataProvider {
    pub(super) api_key: Arc<str>,
    pub(super) http_client: reqwest::Client,
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
    pub fn new(api_key: &str) -> Self {
        let http_client = reqwest::ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("building this client should never fail.");

        let api_key: Arc<str> = api_key.to_owned().into_boxed_str().into();

        Self {
            api_key,
            http_client,
            cache: Default::default(),
        }
    }

    /// curry this metadata provider to supply search results for TV shows.
    #[inline(always)]
    pub fn tv_shows(&self) -> MetadataProviderOf<TvShows> {
        MetadataProviderOf {
            provider: self.clone(),
            _key: PhantomData,
        }
    }

    #[inline(always)]
    pub fn movies(&self) -> MetadataProviderOf<Movies> {
        MetadataProviderOf {
            provider: self.clone(),
            _key: PhantomData,
        }
    }

    /// insert a default [CacheValue] if the slot at a given key is not present.
    fn insert_value_if_empty(&self, key: &CacheKey) -> (CacheValue, bool) {
        // grab the entry or instert RwLock::new(None) if not present.
        let mut entry = self.cache.entry(key.clone()).or_default();

        // fast path: cache hits, no writers and the value is present.
        {
            let read_guard = entry.value();
            if let Some(value) = read_guard.as_ref() {
                return (value.clone(), false);
            }
        }

        // slow path: get a write guard, if the slot is still uninit when we acquire; initialize it.s
        let slot = entry.value_mut();

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

                        let _ = match self.cache.get_mut(key) {
                            Some(mut entry_ref) => entry_ref.replace(CacheValue::Body { text }),
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
        media_type: MediaSearchType,
    ) -> QueryResult<Vec<ExternalMedia>> {
        let title = title.to_string();
        let key = CacheKey::Search {
            title: title.clone(),
            year,
        };

        let st = self
            .coalesce_request(&key, |client| async move {
                client
                    .search(media_type, &title, year)
                    .await
                    .map(|st| st.into_boxed_str().into())
            })
            .await?;

        let mut search = serde_json::from_str::<SearchResponse>(&st).map_err(Error::other)?;

        // fill in the genre names for the search results.
        {
            let key = CacheKey::GenreList { media_type };
            let st = self
                .coalesce_request(&key, |client| async move {
                    client
                        .genre_list(media_type)
                        .await
                        .map(|st| st.into_boxed_str().into())
                })
                .await?;

            let genre_list = serde_json::from_str::<GenreList>(&st).map_err(Error::other)?;

            let mut genre_id_cache = HashMap::<u64, Genre>::with_capacity(search.results.len());
            for media in search.results.iter_mut() {
                if let Some(TMDBMediaObject {
                    genre_ids: Some(ids),
                    genres,
                    ..
                }) = media
                {
                    for genre_id in ids.clone() {
                        if let Some(genre) = genre_id_cache.get(&genre_id) {
                            genres.push(genre.name.clone());
                        } else if let Some(genre) =
                            genre_list.genres.iter().find(|x| x.id == genre_id)
                        {
                            genre_id_cache.insert(genre_id, genre.clone());
                            genres.push(genre.name.clone());
                        }
                    }
                }
            }
        }

        let media = search
            .results
            .into_iter()
            .flatten()
            .map(|media| ExternalMedia {
                external_id: media.id.to_string(),
                title: media.title,
                description: media.overview,
                release_date: media
                    .release_date
                    .and_then(|date| chrono::DateTime::from_str(&date).ok()),
                posters: media.poster_path.into_iter().collect(),
                backdrops: media.backdrop_path.into_iter().collect(),
                genres: media.genres,
                rating: media.vote_average,
                duration: media.runtime.map(|n| Duration::from_secs(n)),
            })
            .collect::<Vec<_>>();

        Ok(media)
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
pub struct MetadataProviderOf<K>
where
    K: Send + Sync + 'static,
{
    pub provider: TMDBMetadataProvider,
    _key: PhantomData<K>,
}

mod sealed {
    use super::{MediaSearchType, Movies, TvShows};

    /// Used to associate a constant [MediaType] with another type.
    pub trait AssocMediaTypeConst {
        const MEDIA_TYPE: MediaSearchType;
    }

    impl AssocMediaTypeConst for TvShows {
        const MEDIA_TYPE: MediaSearchType = MediaSearchType::Tv;
    }

    impl AssocMediaTypeConst for Movies {
        const MEDIA_TYPE: MediaSearchType = MediaSearchType::Movie;
    }
}

#[async_trait]
impl<K> ExternalQuery for MetadataProviderOf<K>
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
impl ExternalQueryShow for MetadataProviderOf<TvShows> {
    async fn seasons_for_id(&self, external_id: &str) -> QueryResult<Vec<ExternalSeason>> {
        todo!()
    }

    async fn episodes_for_season(&self, season_id: &str) -> QueryResult<Vec<ExternalEpisode>> {
        todo!()
    }
}
