use crate::api::APIExec;
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

type CacheType = Arc<Mutex<HashMap<(String, Option<i32>, bool), SearchResult>>>;

lazy_static! {
    static ref CACHE: CacheType = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone, Debug)]
pub struct TMDbSearch<'a> {
    api_key: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    total_results: u64,
    results: VecDeque<Option<QueryResult>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct QueryResult {
    pub id: u64,

    title: Option<String>,
    name: Option<String>,

    release_date: Option<String>,
    first_air_date: Option<String>,

    pub overview: Option<String>,
    pub vote_average: Option<f64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genres: Option<Vec<Genre>>,
}

impl QueryResult {
    pub fn get_release_date(&self) -> Option<String> {
        if self.release_date.is_none() {
            return self.first_air_date.clone();
        }
        self.release_date.clone()
    }

    pub fn get_title(&self) -> Option<String> {
        if self.title.is_none() {
            return self.name.clone();
        }
        self.title.clone()
    }
}

impl<'a> APIExec<'a> for TMDbSearch<'a> {
    fn new(api_key: &'a str) -> Self {
        Self { api_key }
    }

    fn search(&mut self, title: String, year: Option<i32>, tv: bool) -> Option<QueryResult> {
        if let Ok(mut resp) = self.paginated_search(&title, year, tv) {
            return resp.get_one();
        }
        None
    }
}

impl<'a> TMDbSearch<'a> {
    fn paginated_search(
        &mut self,
        title: &str,
        year: Option<i32>,
        tv: bool,
    ) -> Result<SearchResult, reqwest::Error> {
        {
            let cache = CACHE.lock().unwrap();
            let key = (title.to_string(), year, tv);
            if cache.contains_key(&key) {
                return Ok(cache.get(&key).unwrap().clone());
            }
        }

        let sub_point = if tv { "tv" } else { "movie" };

        let mut resp: SearchResult = match year {
            Some(y) => {
                let mut res = reqwest::get(format!("https://api.themoviedb.org/3/search/{}?api_key={}&language=en-US&query={}&page=1&include_adult=false&year={}", sub_point, self.api_key, title, y).as_str())?;
                if res.status().as_u16() == 429u16 {
                    retry_after(res);
                    return self.paginated_search(title, year, tv);
                }

                res.json()?
            }
            None => {
                let mut res = reqwest::get(format!("https://api.themoviedb.org/3/search/{}?api_key={}&language=en-US&query={}&page=1&include_adult=false", sub_point, self.api_key, title).as_str())?;
                if res.status().as_u16() == 429u16 {
                    retry_after(res);
                    return self.paginated_search(title, year, tv);
                }

                res.json()?
            }
        };

        if let Some(x) = resp.get_one() {
            let result: QueryResult = {
                let mut res = reqwest::get(
                    format!(
                        "https://api.themoviedb.org/3/{}/{}?api_key={}&language=en-US",
                        sub_point, x.id, self.api_key
                    )
                    .as_str(),
                )?;
                if res.status().as_u16() == 429u16 {
                    retry_after(res);
                    return self.paginated_search(title, year, tv);
                }

                res.json()?
            };

            resp.put_one(result);
        }

        {
            let mut cache = CACHE.lock().unwrap();
            let key = (title.to_string(), year, tv);

            cache.insert(key, resp.clone());
        }

        Ok(resp)
    }
}

impl SearchResult {
    fn get_one(&mut self) -> Option<QueryResult> {
        match self.results.pop_front() {
            Some(x) => x,
            None => None,
        }
    }

    fn put_one(&mut self, item: QueryResult) {
        self.results.push_front(Some(item));
    }
}

fn retry_after(res: reqwest::Response) {
    // For some reason retry-after is always 0??
    // *might be because im an idiot???
    let _ = res.headers().get("retry-after").unwrap();
}
