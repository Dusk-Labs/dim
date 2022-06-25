//! A TMDB client implementation with request coalescing and client-side rate-limiting.
//!

use std::sync::Arc;

/// The User-Agent is generated from the package name "dim" and the version so "dim/0.x.y.z"
pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// The base url used to access TMDB;
pub const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

mod cache_control;
mod metadata_provider;
mod raw_client;

pub use metadata_provider::{MetadataProviderOf, Movies, TMDBMetadataProvider, TvShows};
use raw_client::{Genre, GenreList, SearchResponse, TMDBMediaObject};

#[derive(Debug, displaydoc::Display, Clone, thiserror::Error)]
pub(self) enum TMDBClientRequestError {
    /// The body of a response was not value UTF-8.
    InvalidUTF8Body,
    /// the error comes from reqwest.
    ReqwestError(#[from] Arc<reqwest::Error>),
}

impl TMDBClientRequestError {
    fn reqwest(err: reqwest::Error) -> Self {
        Self::ReqwestError(Arc::new(err))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::*;
    use crate::external::{ExternalMedia, ExternalQuery};

    #[tokio::test]
    async fn sanity_check_tmdb_works() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_shows: MetadataProviderOf<TvShows> = provider.tv_shows();

        let metadata = provider_shows
            .search("letterkenny", None)
            .await
            .expect("search results should exist");

        let dt = chrono::Utc::now()
            .with_day(7)
            .unwrap()
            .with_month(2)
            .unwrap()
            .with_year(2016)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .with_hour(0)
            .unwrap();

        let letterkenny = ExternalMedia {
            external_id: "65798".into(),
            title: "Letterkenny".into(),
            description: Some("Letterkenny follows Wayne, a good-ol’ country boy in Letterkenny, Ontario trying to protect his homegrown way of life on the farm, against a world that is constantly evolving around him. The residents of Letterkenny belong to one of three groups: Hicks, Skids, and Hockey Players. The three groups are constantly feuding with each other over seemingly trivial matters; often ending with someone getting their ass kicked.".into()),
            release_date: Some(dt), 
            posters: vec!["/yvQGoc9GGTfOyPty5ASShT9tPBD.jpg".into()], 
            backdrops: vec!["/wdHK7RZNIGfskbGCIusSKN3vto6.jpg".into()], 
            genres: vec!["Comedy".into()], 
            rating: Some(8.5), 
            duration: None
        };

        assert_eq!(metadata, vec![letterkenny]);
    }
}
