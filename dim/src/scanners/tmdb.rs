pub(crate) use database::library::MediaType;
use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::StatusCode;

use err_derive::Error;
use futures::stream;
use futures::StreamExt;
use tokio::sync::RwLock;

use async_recursion::async_recursion;

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

#[derive(Clone)]
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

    pub async fn search(
        &mut self,
        title: String,
        year: Option<i32>,
    ) -> Result<super::ApiMedia, TmdbError> {
        self.search_by_name(title, year, None)
            .await?
            .first()
            .cloned()
            .map(Into::into)
            .ok_or(TmdbError::NoResults)
    }

    pub async fn search_by_id(&mut self, id: i32) -> Result<Media, TmdbError> {
        let mut args: Vec<(String, String)> = Vec::new();
        args.push(("api_key".into(), self.api_key.clone()));
        args.push(("language".into(), "en-US".into()));

        let url = format!("{}/{}/{}", self.base, self.media_type.to_string(), id);
        let req = self
            .client
            .get(url)
            .query(&args)
            .send()
            .await
            .map_err(|_| TmdbError::ReqwestError)?;

        #[derive(Deserialize, Clone, Debug)]
        struct WMedia {
            pub id: u64,
            #[serde(rename(deserialize = "original_title", deserialize = "original_name"))]
            pub title: String,
            #[serde(rename(deserialize = "release_date", deserialize = "first_air_date"))]
            pub release_date: Option<String>,
            pub overview: Option<String>,
            pub vote_average: Option<f64>,
            pub poster_path: Option<String>,
            pub backdrop_path: Option<String>,
            pub genres: Vec<GenrePair>,
        }

        #[derive(Deserialize, Clone, Debug)]
        struct GenrePair {
            pub id: u64,
            pub name: String,
        }

        let result: WMedia = req
            .json::<WMedia>()
            .await
            .map_err(|_| TmdbError::DeserializationError)?;

        Ok(Media {
            id: result.id,
            title: result.title,
            release_date: result.release_date,
            overview: result.overview,
            vote_average: result.vote_average,
            poster_path: result.poster_path,
            backdrop_path: result.backdrop_path,
            genre_ids: None,
            genres: result
                .genres
                .clone()
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<String>>(),
        })
    }

    #[async_recursion]
    pub async fn search_by_name(
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
            let lock = (*__CACHE).read().await;
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
            .await
            .map_err(|_| TmdbError::ReqwestError)?;

        if matches!(req.status(), StatusCode::TOO_MANY_REQUESTS) {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            return self.search_by_name(title, year, Some(max_tries - 1)).await;
        }

        let mut result: Vec<Media> = req
            .json::<SearchResult>()
            .await
            .map_err(|_| TmdbError::DeserializationError)?
            .results
            .into_iter()
            .flatten()
            .collect();

        for media in result.iter_mut() {
            let ids = media.genre_ids.clone().unwrap_or_default();
            media.genres = stream::iter(ids)
                .filter_map(|x| {
                    let client = ClientBuilder::new().user_agent(APP_USER_AGENT);
                    let mut this = Tmdb {
                        api_key: self.api_key.clone(),
                        client: client.build().unwrap(),
                        base: self.base.clone(),
                        media_type: self.media_type.clone(),
                    };

                    async move { this.get_genre_detail(x).await.ok().map(|x| x.name.clone()) }
                })
                .collect::<Vec<String>>()
                .await;
        }

        {
            let mut lock = (*__CACHE).write().await;
            let key = (title.clone(), year, self.media_type);
            lock.insert(key, result.clone());
        }

        Ok(result)
    }

    pub async fn get_seasons_for(&mut self, id: u64) -> Result<Vec<Season>, TmdbError> {
        let mut args: Vec<(String, String)> = Vec::new();
        args.push(("api_key".into(), self.api_key.clone()));

        let req = self
            .client
            .get(format!("{}/tv/{}", self.base, id))
            .query(&args)
            .send()
            .await
            .map_err(|_| TmdbError::ReqwestError)?;

        #[derive(Deserialize)]
        struct Wrapper {
            seasons: Option<Vec<Season>>,
        }

        req.json::<Wrapper>()
            .await
            .map_err(|_| TmdbError::DeserializationError)?
            .seasons
            .ok_or(TmdbError::NoResults)
    }

    pub async fn get_episodes_for(
        &mut self,
        id: u64,
        season: u64,
    ) -> Result<Vec<Episode>, TmdbError> {
        let mut args: Vec<(String, String)> = Vec::new();
        args.push(("api_key".into(), self.api_key.clone()));

        let req = self
            .client
            .get(format!("{}/tv/{}/season/{}", self.base, id, season))
            .query(&args)
            .send()
            .await
            .map_err(|_| TmdbError::ReqwestError)?;

        #[derive(Deserialize)]
        struct Wrapper {
            episodes: Option<Vec<Episode>>,
        }

        req.json::<Wrapper>()
            .await
            .map_err(|_| TmdbError::DeserializationError)?
            .episodes
            .ok_or(TmdbError::NoResults)
    }

    pub async fn get_genre_detail(&mut self, genre_id: u64) -> Result<Genre, TmdbError> {
        lazy_static::lazy_static! {
            static ref __CACHE: Arc<RwLock<HashMap<MediaType, Vec<Genre>>>> = Arc::new(RwLock::new(HashMap::new()));
        }

        {
            let lock = (*__CACHE).read().await;
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
            .await
            .map_err(|_| TmdbError::ReqwestError)?;

        #[derive(Deserialize)]
        struct Wrapper {
            genres: Vec<Genre>,
        }

        let genres = req
            .json::<Wrapper>()
            .await
            .map_err(|_| TmdbError::DeserializationError)?
            .genres;

        {
            let mut lock = (*__CACHE).write().await;
            lock.insert(self.media_type.clone(), genres.clone());
        }

        genres
            .iter()
            .find(|x| x.id == genre_id)
            .cloned()
            .ok_or(TmdbError::NoResults)
    }
}
/*

 {
  "page": 1,
  "results": [
    {
      "adult": false,
      "backdrop_path": "/1stUIsjawROZxjiCMtqqXqgfZWC.jpg",
      "genre_ids": [
        12,
        14
      ],
      "id": 672,
      "original_language": "en",
      "original_title": "Harry Potter and the Chamber of Secrets",
      "overview": "Cars fly, trees fight back, and a mysterious house-elf comes to warn Harry Potter at the start of his second year at Hogwarts. Adventure and danger await when bloody writing on a wall announces: The Chamber Of Secrets Has Been Opened. To save Hogwarts will require all of Harry, Ron and Hermione’s magical abilities and courage.",
      "popularity": 118.243,
      "poster_path": "/sdEOH0992YZ0QSxgXNIGLq1ToUi.jpg",
      "release_date": "2002-11-13",
      "title": "Harry Potter and the Chamber of Secrets",
      "video": false,
      "vote_average": 7.7,
      "vote_count": 16310
    }
  ],
  "total_pages": 1,
  "total_results": 1
}
*/

#[derive(Deserialize, Clone)]
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
    #[serde(skip_deserializing)]
    pub genres: Vec<String>,
}

impl Into<super::ApiMedia> for Media {
    fn into(self) -> super::ApiMedia {
        super::ApiMedia {
            id: self.id,
            title: self.title,
            release_date: self.release_date,
            overview: self.overview,
            poster_path: self
                .poster_path
                .clone()
                .map(|s| format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", s)),
            poster_file: self.poster_path,
            backdrop_path: self
                .backdrop_path
                .clone()
                .map(|s| format!("https://image.tmdb.org/t/p/original/{}", s)),
            backdrop_file: self.backdrop_path,
            genres: self.genres,
            rating: self.vote_average.map(|x| x as i32),
            seasons: Vec::new(),
        }
    }
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

impl Into<super::ApiSeason> for Season {
    fn into(self) -> super::ApiSeason {
        super::ApiSeason {
            id: self.id,
            name: self.name,
            poster_path: self
                .poster_path
                .clone()
                .map(|s| format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", s)),
            poster_file: self.poster_path.clone(),
            season_number: self.season_number.unwrap_or(1),
            episodes: Vec::new(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Episode {
    pub id: u64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub episode_number: Option<u64>,
    pub still_path: Option<String>,
}

impl Into<super::ApiEpisode> for Episode {
    fn into(self) -> super::ApiEpisode {
        super::ApiEpisode {
            id: self.id,
            name: self.name,
            overview: self.overview,
            episode: self.episode_number,
            still: self
                .still_path
                .clone()
                .map(|s| format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{}", s)),
            still_file: self.still_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const API_KEY: &str = "38c372f5bc572c8aadde7a802638534e";

    // #[test]
    // fn test_search_by_name() {
    //     let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Movie);
    //     let result = tmdb
    //         .search_by_name("Blade Runner 2049".into(), None, None)
    //         .unwrap();

    //     let result = result.first().unwrap();
    //     assert_eq!(result.title, "Blade Runner 2049");
    //     assert_eq!(result.release_date, Some("2017-10-04".into()));

    //     let result = tmdb
    //         .search_by_name("Blade Runner 2049".into(), Some(2017), None)
    //         .unwrap();

    //     let result = result.first().unwrap();
    //     assert_eq!(result.title, "Blade Runner 2049");
    //     assert_eq!(result.release_date, Some("2017-10-04".into()));
    //     assert!(result.overview.is_some());

    //     let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
    //     let result = tmdb
    //         .search_by_name("The expanse".into(), None, None)
    //         .unwrap();

    //     let result = result.first().unwrap();
    //     assert_eq!(result.title, "The Expanse");
    //     assert_eq!(result.release_date, Some("2015-12-14".into()));
    //     assert!(result.overview.is_some());
    //     assert!(result.poster_path.is_some());
    // }

    // #[test]
    // fn test_get_seasons_for() {
    //     let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
    //     let result = tmdb
    //         .search_by_name("The expanse".into(), None, None)
    //         .unwrap();

    //     let result = result.first().unwrap();
    //     let seasons = tmdb.get_seasons_for(&result).unwrap();

    //     assert_eq!(seasons.len(), 6);

    //     let season1 = &seasons[1];
    //     assert_eq!(season1.air_date, Some("2015-12-14".into()));
    //     assert_eq!(season1.episode_count, Some(10));
    //     assert_eq!(season1.season_number, Some(1));
    //     assert_eq!(season1.name, Some("Season 1".into()));
    //     assert!(season1.overview.is_some());
    // }

    // #[test]
    // fn test_get_episodes_for() {
    //     let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
    //     let result = tmdb
    //         .search_by_name("The expanse".into(), None, None)
    //         .unwrap();

    //     let result = result.first().unwrap();
    //     let seasons = tmdb.get_seasons_for(&result).unwrap();

    //     assert_eq!(seasons.len(), 6);

    //     let season1 = &seasons[1];

    //     let result = tmdb
    //         .get_episodes_for(&result, season1.season_number.unwrap())
    //         .unwrap();
    //     assert_eq!(result.len(), 10);
    // }

    // #[test]
    // fn test_get_genre_detail() {
    //     let mut tmdb = Tmdb::new(API_KEY.to_string(), MediaType::Tv);
    //     let result = tmdb
    //         .search_by_name("The expanse".into(), None, None)
    //         .unwrap();

    //     let genres = result.first().unwrap().genre_ids.as_ref().unwrap();

    //     let result = tmdb.get_genre_detail(genres[0]).unwrap();
    //     assert_eq!(result.name, "Drama".to_string());
    // }
}
