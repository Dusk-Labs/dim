use super::APIExec;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

type CacheType = Arc<Mutex<HashMap<(String, Option<i32>, MediaType), SearchResult>>>;
type GenreCache = Arc<Mutex<HashMap<MediaType, GenreWrapper>>>;

lazy_static::lazy_static! {
    static ref CACHE: CacheType = Arc::new(Mutex::new(HashMap::new()));
    static ref GENRE_CACHE: GenreCache = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum MediaType {
    Movie,
    Tv,
}

#[derive(Clone, Debug)]
pub struct TMDbSearch {
    api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    total_results: u64,
    results: VecDeque<Option<Media>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: u64,
    pub title: Option<String>,
    pub name: Option<String>,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub overview: Option<String>,
    pub vote_average: Option<f64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<u64>>,
    pub genres: Option<Vec<Genre>>,
    pub seasons: Option<Vec<Seasons>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Seasons {
    pub id: u64,
    pub air_date: Option<String>,
    pub episode_count: Option<u64>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: Option<u64>,
    pub episodes: Option<Vec<Episode>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wrapper {
    pub seasons: Option<Vec<Seasons>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenreWrapper {
    pub genres: Vec<Genre>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Episode {
    pub id: u64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub episode_number: Option<u64>,
    pub still_path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

impl MediaType {
    pub fn to_string(self) -> String {
        match self {
            MediaType::Movie => "movie".to_string(),
            MediaType::Tv => "tv".to_string(),
        }
    }
}

impl TMDbSearch {
    fn internal_search_by_id(
        &self,
        id: i32,
        media_type: MediaType,
    ) -> Result<SearchResult, reqwest::Error> {
        let mut res = reqwest::get(
            format!(
                "https://api.themoviedb.org/3/{}/{}?api_key={}",
                media_type.to_string(),
                id,
                self.api_key
            )
            .as_str(),
        )?;

        if res.status().as_u16() == 429u16 {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            return self.internal_search_by_id(id, media_type);
        }

        let result = SearchResult {
            total_results: 1,
            results: {
                let mut deque = VecDeque::new();
                deque.push_back(Some(res.json()?));
                deque
            },
        };

        let resp = match media_type {
            MediaType::Tv => self.fill_details(result),
            _ => result,
        };

        Ok(resp)
    }

    fn internal_search(
        &self,
        title: String,
        year: Option<i32>,
        media_type: MediaType,
    ) -> Result<SearchResult, reqwest::Error> {
        {
            let cache = CACHE.lock().unwrap();
            let key = (title.clone(), year, media_type);
            if cache.contains_key(&key) {
                return Ok(cache.get(&key).unwrap().clone());
            }
        }

        let year_query = year.map_or_else(|| "".to_string(), |x| format!("&year={}", x));

        let mut res = reqwest::get(
            format!(
                "https://api.themoviedb.org/3/search/{}?api_key={}&language=en-US&query={}&page=1&include_adult=false{}", 
                media_type.to_string(),
                self.api_key,
                title, 
                year_query).as_str())?;
        if res.status().as_u16() == 429u16 {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            return self.internal_search(title, year, media_type);
        }

        let result = res.json()?;
        let resp = match media_type {
            MediaType::Tv => self.fill_details(result),
            _ => result,
        };

        let resp = self.fetch_genres(resp, media_type);

        {
            let mut cache = CACHE.lock().unwrap();
            let key = (title, year, media_type);

            cache.insert(key, resp.clone());
        }

        Ok(resp)
    }

    fn fill_details(&self, mut result: SearchResult) -> SearchResult {
        if let Some(b) = result.results.front_mut() {
            if let Some(bs) = b.as_mut() {
                bs.seasons = self.fetch_seasons(bs.id);
            }
        }

        if let Some(b) = result.results.front_mut() {
            if let Some(bs) = b.as_mut() {
                if let Some(cs) = bs.seasons.as_mut() {
                    for c in cs.iter_mut() {
                        c.episodes(bs.id, self.api_key.clone());
                    }
                }
            }
        }
        result
    }

    fn fetch_seasons(&self, id: u64) -> Option<Vec<Seasons>> {
        let req = reqwest::get(
            format!(
                "https://api.themoviedb.org/3/tv/{}?api_key={}",
                id, self.api_key
            )
            .as_str(),
        );

        if let Ok(mut d) = req {
            match d.json::<Wrapper>() {
                Ok(x) => {
                    return x.seasons;
                }
                Err(_) => return None,
            };
        }

        None
    }

    fn fetch_genres(&self, mut result: SearchResult, media_type: MediaType) -> SearchResult {
        if let Some(b) = result.results.front_mut() {
            if let Some(bs) = b.as_mut() {
                bs.genres = self.inner_fetch_genres(bs.genre_ids.clone(), media_type);
            }
        }
        result
    }

    fn inner_fetch_genres(
        &self,
        genre_ids: Option<Vec<u64>>,
        media_type: MediaType,
    ) -> Option<Vec<Genre>> {
        let genre_ids = match genre_ids {
            Some(x) => x,
            None => return None,
        };

        {
            let genre_cache = GENRE_CACHE.lock().unwrap();
            if genre_cache.contains_key(&media_type) {
                let genres = genre_cache.get(&media_type).unwrap();
                return Some(
                    genres
                        .genres
                        .iter()
                        .filter(|y| genre_ids.contains(&y.id))
                        .cloned()
                        .collect::<Vec<Genre>>(),
                );
            }
        }

        let path = media_type.to_string();
        let req = reqwest::get(
            format!(
                "https://api.themoviedb.org/3/genre/{}/list?api_key={}",
                path, self.api_key
            )
            .as_str(),
        );

        if let Ok(mut d) = req {
            match d.json::<GenreWrapper>() {
                Ok(x) => {
                    {
                        let mut genre_cache = GENRE_CACHE.lock().unwrap();
                        genre_cache.insert(media_type, x.clone());
                    }

                    return Some(
                        x.genres
                            .iter()
                            .filter(|y| genre_ids.contains(&y.id))
                            .cloned()
                            .collect::<Vec<Genre>>(),
                    );
                }
                Err(err) => {
                    println!("{:?}", err);
                    return None;
                }
            }
        }
        None
    }
}

impl Seasons {
    pub fn episodes(&mut self, id: u64, api_key: String) {
        let req = reqwest::get(
            format!(
                "https://api.themoviedb.org/3/tv/{}/season/{}?api_key={}",
                id,
                self.season_number.unwrap_or(0),
                api_key
            )
            .as_str(),
        );

        if let Ok(mut d) = req {
            if let Ok(x) = d.json::<Seasons>() {
                self.episodes = x.episodes;
            }
        }
    }
}

impl<'a> APIExec<'a> for TMDbSearch {
    fn new(api_key: &'a str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    fn search(&mut self, title: String, year: Option<i32>, media_type: MediaType) -> Option<Media> {
        self.internal_search(title, year, media_type)
            .map_or_else(|_| None, |mut x| x.pop_front())
    }

    fn search_many(
        &mut self,
        title: String,
        year: Option<i32>,
        media_type: MediaType,
        result_num: usize,
    ) -> Vec<Media> {
        self.internal_search(title, year, media_type)
            .map_or_else(|_| Vec::new(), |x| x.pop_many(result_num))
    }

    fn search_by_id(&mut self, id: i32, media_type: MediaType) -> Option<Media> {
        self.internal_search_by_id(id, media_type)
            .map_or_else(|_| None, |mut x| x.pop_front())
    }
}

impl Media {
    pub fn get_season(&self, num: i32) -> Seasons {
        if let Some(seasons) = self.seasons.as_ref() {
            if let Some(x) = seasons
                .iter()
                .find(|x| x.season_number.eq(&Some(num as u64)))
            {
                return x.clone();
            }
        }
        Seasons::default()
    }

    pub fn get_title(&self) -> Option<String> {
        if self.title.is_none() {
            return self.name.clone();
        }
        self.title.clone()
    }

    pub fn get_release_date(&self) -> Option<String> {
        if self.release_date.is_none() {
            return self.first_air_date.clone();
        }
        self.release_date.clone()
    }
}

impl Seasons {
    pub fn get_episode(&self, num: i32) -> Episode {
        if let Some(episodes) = self.episodes.as_ref() {
            if let Some(x) = episodes
                .iter()
                .find(|x| x.episode_number.eq(&Some(num as u64)))
            {
                return x.clone();
            }
        }
        Episode::default()
    }
}

impl SearchResult {
    fn pop_front(&mut self) -> Option<Media> {
        self.results.pop_front().map_or_else(|| None, |x| x)
    }

    fn pop_many(&self, num: usize) -> Vec<Media> {
        self.results
            .clone()
            .into_iter()
            .take(num)
            .filter_map(|x| x)
            .collect::<Vec<Media>>()
    }
}
