extern crate reqwest;

use std::collections::HashMap;
use torrent_name_parser::Metadata;
use serde::Serialize;
use chrono::prelude::Utc;
use chrono::NaiveDate;
use chrono::Datelike;

use crate::tmdb::{TMDbSearch, MovieResult};

#[derive(Serialize)]
struct InsertableMedia {
    library_id: i32,
    name: String,
    added: String,
    media_type: String,
}

#[derive(Serialize)]
struct UpdateMedia {
    pub name: String,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub year: Option<i32>,
    pub added: Option<String>,
    pub poster_path: Option<String>,
}

pub struct MovieObject<'a> {
    raw_metadata: Metadata,
    path: String,
    api: TMDbSearch<'a>,
    lib_id: i32,
    media_id: Option<i32>,
}

impl<'a> MovieObject<'a> {
    pub fn new(path: String, lib_id: i32) -> Self {
        let metadata = Metadata::from(path.as_str());
        Self {
            raw_metadata: metadata,
            path: path,
            api: TMDbSearch::new("38c372f5bc572c8aadde7a802638534e"),
            lib_id: lib_id,
            media_id: None
        }
    }

    pub fn metadata_scan(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let body = InsertableMedia {
            library_id: self.lib_id,
            name: self.raw_metadata.title().to_owned(),
            added: Utc::now().to_string(),
            media_type: String::from("movie"),
        };
        let client = reqwest::Client::new();

        let result: HashMap<String, i32> = client.post("http://127.0.0.1:8000/api/v1/media")
            .json(&body)
            .send()?
            .json()?;

        if let Some(id) = result.get("id") {
            self.media_id = Some(*id);
        }
        Ok(())
    }

    pub fn media_fetch(&mut self) -> Result<(), ()> {
        let year = match self.raw_metadata.year() {
            Some(x) => Some(x as u16),
            None => None,
        };

        let res = self.api.search(self.raw_metadata.title().to_owned(), year);
        match res {
            Some(x) => { self.update(x).unwrap(); },
            None => { },
        };
        Ok(())
    }

    fn update(&mut self, data: MovieResult) -> Result<(), Box<dyn std::error::Error>> {
        if self.media_id.is_none() {
            return Ok(())
        }
        
        let year: Option<i32> = match data.release_date {
            Some(x) => Some(NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d").unwrap().year() as i32),
            None => None,
        };

        let body = UpdateMedia {
            name: data.title,
            description: data.overview,
            rating: match data.vote_average {
                Some(x) => Some(x as i32),
                None => None,
            },
            year: year,
            added: None,
            poster_path: match data.poster_path {
                Some(path) => Some(format!("http://image.tmdb.org/t/p/original/{}", path)),
                None => None,
            },
        };

        let client = reqwest::Client::new();

        let _ = client.patch(format!("http://127.0.0.1:8000/api/v1/media/{}", self.media_id.unwrap()).as_str())
            .json(&body)
            .send()?;

        Ok(())
    }
}

pub trait APIExec<'a> {
    fn new(api_key: &'a str) -> Self;
    fn search(&mut self, title: String, year: Option<u16>) -> Option<MovieResult>;
}
