pub(crate) use crate::scanners::ApiMediaType as MediaType;
use crate::scanners::MetadataAgent;

use serde::Deserialize;
use serde::Serialize;

use std::collections::VecDeque;
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, unimplemented};

use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
use reqwest::StatusCode;

use err_derive::Error;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Error, Serialize)]
pub enum TmdbError {
    #[error(display = "The request timeouted")]
    Timeout,
    #[error(display = "Max retry count reached")]
    ReachedMaxTries,
    #[error(display = "Internal error with reqwest")]
    ReqwestError,
    #[error(display = "The json returned could not be deserialized")]
    DeserializationError,
    #[error(display = "No results are found")]
    NoResults,
}

pub struct Tmdb {
    api_key: String,
    client: Client,
    base: String,
    media_type: MediaType,
}

impl Tmdb {
    pub fn new(api_key: String, media_type: MediaType) -> Self {
        let client = ClientBuilder::new().user_agent(APP_USER_AGENT);

        Self {
            api_key,
            client: client.build().unwrap(),
            base: "https://api.themoviedb.org/3".into(),
            media_type,
        }
    }

    pub fn search_by_name(
        &mut self,
        title: String,
        year: Option<i32>,
        max_tries: Option<usize>,
    ) -> Result<Vec<Media>, TmdbError> {
        type CacheKey = (String, Option<i32>, MediaType);
        type CacheStore = Arc<RwLock<HashMap<CacheKey, Vec<Media>>>>;

        lazy_static::lazy_static! {
            static ref __CACHE: CacheStore = Arc::new(RwLock::new(HashMap::new()));
        }

        {
            let lock = (*__CACHE).read().unwrap();
            let key = (title.clone(), year, self.media_type);

            if let Some(x) = lock.get(&key) {
                return Ok(x.to_vec());
            }
        }

        let max_tries = max_tries.unwrap_or(10);

        if max_tries <= 0 {
            return Err(TmdbError::ReachedMaxTries);
        }

        let mut args: Vec<(String, String)> = Vec::new();
        args.push(("api_key".into(), self.api_key.clone()));
        args.push(("language".into(), "en-US".into()));
        args.push(("query".into(), title.clone()));
        args.push(("page".into(), "1".into()));
        args.push(("include_adult".into(), "false".into()));

        if let Some(year) = year {
            args.push(("year".into(), year.to_string()));
        }

        let url = format!("{}/search/{}", self.base, self.media_type.to_string(),);

        let req = self
            .client
            .get(url)
            .query(&args)
            .send()
            .map_err(|_| TmdbError::ReqwestError)?;

        if matches!(req.status(), StatusCode::TOO_MANY_REQUESTS) {
            thread::sleep(Duration::from_millis(1000));
            return self.search_by_name(title, year, Some(max_tries - 1));
        }

        let result: Vec<Media> = req
            .json::<SearchResult>()
            .map_err(|_| TmdbError::DeserializationError)?
            .results
            .into_iter()
            .flatten()
            .collect();

        {
            let mut lock = (*__CACHE).write().unwrap();
            let key = (title.clone(), year, self.media_type);
            lock.insert(key, result.clone());
        }

        Ok(result)
    }

    pub fn get_seasons_for(&mut self, media: &Media) -> Result<Vec<Season>, TmdbError> {
        let mut args: Vec<(String, String)> = Vec::new();
        args.push(("api_key".into(), self.api_key.clone()));

        let req = self
            .client
            .get(format!("{}/tv/{}", self.base, media.id))
            .query(&args)
            .send()
            .map_err(|_| TmdbError::ReqwestError)?;

        #[derive(Deserialize)]
        struct Wrapper {
            seasons: Option<Vec<Season>>,
        }

        req.json::<Wrapper>()
            .map_err(|_| TmdbError::DeserializationError)?
            .seasons
            .ok_or(TmdbError::NoResults)
    }

    pub fn get_episodes_for(
        &mut self,
        media: &Media,
        season: u64,
    ) -> Result<Vec<Episode>, TmdbError> {
        let mut args: Vec<(String, String)> = Vec::new();
        args.push(("api_key".into(), self.api_key.clone()));

        let req = self
            .client
            .get(format!("{}/tv/{}/season/{}", self.base, media.id, season))
            .query(&args)
            .send()
            .map_err(|_| TmdbError::ReqwestError)?;

        #[derive(Deserialize)]
        struct Wrapper {
            episodes: Option<Vec<Episode>>,
        }

        req.json::<Wrapper>()
            .map_err(|_| TmdbError::DeserializationError)?
            .episodes
            .ok_or(TmdbError::NoResults)
    }

    pub fn get_genre_detail(&mut self, genre_id: u64) -> Result<Genre, TmdbError> {
        lazy_static::lazy_static! {
            static ref __CACHE: Arc<RwLock<HashMap<MediaType, Vec<Genre>>>> = Arc::new(RwLock::new(HashMap::new()));
        }

        {
            let lock = (*__CACHE).read().unwrap();
            if let Some(x) = lock.get(&self.media_type) {
                if let Some(x) = x.iter().find(|x| x.id == genre_id) {
                    return Ok(x.clone());
                }
            }
        }

        let mut args: Vec<(String, String)> = Vec::new();
        args.push(("api_key".into(), self.api_key.clone()));

        let url = format!("{}/genre/{}/list", self.base.clone(), self.media_type);
        let req = self
            .client
            .get(url)
            .query(&args)
            .send()
            .map_err(|_| TmdbError::ReqwestError)?;

        #[derive(Deserialize)]
        struct Wrapper {
            genres: Vec<Genre>,
        }

        let genres = req
            .json::<Wrapper>()
            .map_err(|_| TmdbError::DeserializationError)?
            .genres;

        {
            let mut lock = (*__CACHE).write().unwrap();
            lock.insert(self.media_type, genres.clone());
        }

        genres
            .iter()
            .find(|x| x.id == genre_id)
            .cloned()
            .ok_or(TmdbError::NoResults)
    }
}

impl MetadataAgent for Tmdb {
    type Error = TmdbError;

    fn search(&mut self, title: String, year: Option<i32>) -> Result<super::ApiMedia, Self::Error> {
        self.fetch(title, year)?
            .next()
            .ok_or(TmdbError::NoResults)?
    }

    fn fetch(
        &mut self,
        title: String,
        year: Option<i32>,
    ) -> Result<Box<dyn Iterator<Item = Result<super::ApiMedia, Self::Error>>>, Self::Error> {
        let mut results = self.search_by_name(title, year, None)?.into_iter().fuse();

        let client = ClientBuilder::new().user_agent(APP_USER_AGENT);
        let mut this = Tmdb {
            api_key: self.api_key.clone(),
            client: client.build().unwrap(),
            base: self.base.clone(),
            media_type: self.media_type.clone(),
        };

        let it = std::iter::from_fn(move || {
            let media = results.next()?;

            // TODO: Add From-impls for: Season -> ApiSeason, Media -> ApiMedia.

            let seasons = match this.media_type {
                MediaType::Movie => Vec::new(),
                MediaType::Tv => {
                    let seasons = match this.get_seasons_for(&media) {
                        Ok(s) => s,
                        Err(e) => return Some(Err(e)),
                    };

                    seasons
                        .iter()
                        .map(|x| super::ApiSeason {
                            id: x.id,
                            name: x.name.clone(),
                            poster_path: x.poster_path.clone().map(|s| {
                                format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", s)
                            }),
                            poster_file: x.poster_path.clone(),
                            season_number: x.season_number.unwrap(),
                            episodes: this
                                .get_episodes_for(&media, x.season_number.unwrap_or(0))
                                .unwrap()
                                .iter()
                                .map(|x| super::ApiEpisode {
                                    id: x.id,
                                    name: x.name.clone(),
                                    overview: x.overview.clone(),
                                    episode: x.episode_number.clone(),
                                    still: x.still_path.clone().map(|s| {
                                        format!("https://image.tmdb.org/t/p/original/{}", s)
                                    }),
                                    still_file: x.still_path.clone(),
                                })
                                .collect(),
                        })
                        .collect()
                }
            };

            let result = super::ApiMedia {
                id: media.id,
                title: media.title.clone(),
                release_date: media.release_date.clone(),
                overview: media.overview.clone(),
                rating: media.vote_average.map(|x| x as i32),
                poster_path: media
                    .poster_path
                    .clone()
                    .map(|s| format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", s)),
                poster_file: media.poster_path.clone(),
                backdrop_path: media
                    .backdrop_path
                    .clone()
                    .map(|s| format!("https://image.tmdb.org/t/p/original/{}", s)),
                backdrop_file: media.backdrop_path.clone(),
                genres: media
                    .genre_ids
                    .clone()
                    .map(|g| {
                        g.iter()
                            .map(|x| this.get_genre_detail(*x))
                            .filter_map(|x| x.ok())
                            .map(|x| x.name)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                media_type: match this.media_type {
                    MediaType::Tv => super::ApiMediaType::Tv,
                    _ => super::ApiMediaType::Movie,
                },
                seasons,
            };

            Some(Ok(result))
        });

        Ok(Box::new(it))
    }

    fn search_many(
        &mut self,
        title: String,
        year: Option<i32>,
        n: usize,
    ) -> Result<Vec<super::ApiMedia>, Self::Error> {
        let it = self.fetch(title, year)?;
        let mut results = Vec::with_capacity(n);

        for result in it.take(n) {
            results.push(result?);
        }

        Ok(results)
    }
}

#[derive(Deserialize)]
struct SearchResult {
    results: Vec<Option<Media>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Media {
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
}

#[derive(Deserialize, Clone, Debug)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Season {
    pub id: u64,
    pub air_date: Option<String>,
    pub episode_count: Option<u64>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: Option<u64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Episode {
    pub id: u64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub episode_number: Option<u64>,
    pub still_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const API_KEY: &str = "38c372f5bc572c8aadde7a802638534e";

    #[test]
    fn test_search_by_name() {
        let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Movie);
        let result = tmdb
            .search_by_name("Blade Runner 2049".into(), None, None)
            .unwrap();

        let result = result.first().unwrap();
        assert_eq!(result.title, "Blade Runner 2049");
        assert_eq!(result.release_date, Some("2017-10-04".into()));

        let result = tmdb
            .search_by_name("Blade Runner 2049".into(), Some(2017), None)
            .unwrap();

        let result = result.first().unwrap();
        assert_eq!(result.title, "Blade Runner 2049");
        assert_eq!(result.release_date, Some("2017-10-04".into()));
        assert!(result.overview.is_some());

        let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
        let result = tmdb
            .search_by_name("The expanse".into(), None, None)
            .unwrap();

        let result = result.first().unwrap();
        assert_eq!(result.title, "The Expanse");
        assert_eq!(result.release_date, Some("2015-12-14".into()));
        assert!(result.overview.is_some());
        assert!(result.poster_path.is_some());
    }

    #[test]
    fn test_get_seasons_for() {
        let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
        let result = tmdb
            .search_by_name("The expanse".into(), None, None)
            .unwrap();

        let result = result.first().unwrap();
        let seasons = tmdb.get_seasons_for(&result).unwrap();

        assert_eq!(seasons.len(), 6);

        let season1 = &seasons[1];
        assert_eq!(season1.air_date, Some("2015-12-14".into()));
        assert_eq!(season1.episode_count, Some(10));
        assert_eq!(season1.season_number, Some(1));
        assert_eq!(season1.name, Some("Season 1".into()));
        assert!(season1.overview.is_some());
    }

    #[test]
    fn test_get_episodes_for() {
        let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
        let result = tmdb
            .search_by_name("The expanse".into(), None, None)
            .unwrap();

        let result = result.first().unwrap();
        let seasons = tmdb.get_seasons_for(&result).unwrap();

        assert_eq!(seasons.len(), 6);

        let season1 = &seasons[1];

        let result = tmdb
            .get_episodes_for(&result, season1.season_number.unwrap())
            .unwrap();
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_get_genre_detail() {
        let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
        let result = tmdb
            .search_by_name("The expanse".into(), None, None)
            .unwrap();

        let genres = result.first().unwrap().genre_ids.as_ref().unwrap();

        let result = tmdb.get_genre_detail(genres[0]).unwrap();
        assert_eq!(result.name, "Drama".to_string());
    }
}
