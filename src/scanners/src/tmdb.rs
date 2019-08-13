extern crate reqwest;

use std::collections::VecDeque;
use serde::{Deserialize};
use crate::api::{APIExec};

#[derive(Clone, Debug)]
pub struct TMDbSearch<'a> {
    api_key: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    total_results: u64,
    results: VecDeque<Option<MovieResult>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MovieResult {
    pub id: u64,
    pub title: String,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub vote_average: Option<f64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

impl<'a> APIExec<'a> for TMDbSearch<'a> {
    fn new(api_key: &'a str) -> Self {
        Self { api_key: api_key }
    }

    fn search(&mut self, title: String, year: Option<u16>) -> Option<MovieResult> {
        let mut resp = self.paginated_search(&title, &year).unwrap();
        resp.get_one()
    }
}

impl<'a> TMDbSearch<'a> {
    fn paginated_search(
        &mut self,
        title: &String,
        year: &Option<u16>,
    ) -> Result<SearchResult, Box<dyn std::error::Error>> {
        let resp: SearchResult = match year {
            Some(y) => reqwest::get(
                format!("https://api.themoviedb.org/3/search/movie?api_key={}&language=en-US&query={}&page=1&include_adult=false&year={}", self.api_key, title, y).as_str()
                )?.json()?,
            None => reqwest::get(
                format!("https://api.themoviedb.org/3/search/movie?api_key={}&language=en-US&query={}&page=1&include_adult=false", self.api_key, title).as_str()
                )?.json()?,
        };

        Ok(resp)
    }
}

impl SearchResult {
    fn get_one(&mut self) -> Option<MovieResult> {
        match self.results.pop_front() {
            Some(x) => x,
            None => None,
        }
    }
}
