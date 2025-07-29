//! A TMDB client implementation with request coalescing and client-side rate-limiting.

use std::sync::Arc;

/// The User-Agent is generated from the package name "dim" and the version so "dim/0.x.y.z"
pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// The base url used to access TMDB;
pub const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

mod cache_control;
mod metadata_provider;
mod raw_client;

pub use metadata_provider::{MetadataProviderOf, Movies, TMDBMetadataProvider, TvShows};
use raw_client::{Cast, Genre, GenreList, SearchResponse, TMDBMediaObject, TvEpisodes, TvSeasons};

#[derive(Debug, displaydoc::Display, Clone, thiserror::Error)]
pub(self) enum TMDBClientRequestError {
    /// The body of a response was not value UTF-8.
    InvalidUTF8Body,
    /// the error comes from reqwest.
    ReqwestError(#[from] Arc<reqwest::Error>),
    /// Failed to receive result over channel: {0:?}
    RecvError(#[from] tokio::sync::broadcast::error::RecvError),
    /// Received {status:?} response code: {body:?}
    NonOkResponse {
        status: reqwest::StatusCode,
        body: String,
    },
}

impl TMDBClientRequestError {
    fn reqwest(err: reqwest::Error) -> Self {
        Self::ReqwestError(Arc::new(err))
    }
}

impl From<TMDBClientRequestError> for super::Error {
    fn from(this: TMDBClientRequestError) -> super::Error {
        use TMDBClientRequestError::*;

        match this {
            // We can generalize some errors into common ones from the upstream enum, but some have to
            // do with internal logic and are this mapped into `OtherError`. `OtherError` generally
            // indicates unexpected errors that cannot be handled. These usually mean something broke.
            //
            // FIXME: `InternalError` might be a more appropriate name.
            InvalidUTF8Body | ReqwestError(_) | RecvError(_) => {
                super::Error::OtherError(Arc::new(this))
            }

            NonOkResponse { status, body } => {
                let message = serde_json::from_str::<raw_client::TmdbError>(&body)
                    .map(|x| x.status_message)
                    .unwrap_or(body);

                super::Error::RemoteApiError {
                    code: status.as_u16(),
                    message,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::*;
    use crate::{ExternalEpisode, ExternalMedia, ExternalQuery, ExternalQueryShow, ExternalSeason};

    fn make_letterkenny() -> ExternalMedia {
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

        ExternalMedia {
            external_id: "65798".into(),
            title: "Letterkenny".into(),
            description: Some("Letterkenny follows Wayne, a good-ol’ country boy in Letterkenny, Ontario trying to protect his homegrown way of life on the farm, against a world that is constantly evolving around him. The residents of Letterkenny belong to one of three groups: Hicks, Skids, and Hockey Players. The three groups are constantly feuding with each other over seemingly trivial matters; often ending with someone getting their ass kicked.".into()),
            release_date: Some(dt),
            posters: vec!["https://image.tmdb.org/t/p/w600_and_h900_bestv2/yvQGoc9GGTfOyPty5ASShT9tPBD.jpg".into()], 
            backdrops: vec!["https://image.tmdb.org/t/p/original/wdHK7RZNIGfskbGCIusSKN3vto6.jpg".into()], 
            genres: vec!["Comedy".into()], 
            rating: Some(8.0),
            duration: None
        }
    }

    #[tokio::test]
    async fn tmdb_search() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_shows: MetadataProviderOf<TvShows> = provider.tv_shows();

        let mut metadata = provider_shows
            .search("letterkenny", None)
            .await
            .expect("search results should exist");

        let letterkenny = make_letterkenny();

        if let Some(mt) = metadata.first_mut() {
            mt.backdrops = letterkenny.backdrops.clone();
            mt.posters = letterkenny.posters.clone();
            mt.rating.replace(8.0);
        }

        assert_eq!(metadata, vec![letterkenny]);
    }

    #[tokio::test]
    async fn tmdb_get_details() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_shows: MetadataProviderOf<TvShows> = provider.tv_shows();

        let mut media = provider_shows
            .search_by_id("65798")
            .await
            .expect("search results should exist");

        let letterkenny = make_letterkenny();

        media.posters = letterkenny.posters.clone();
        media.backdrops = letterkenny.backdrops.clone();
        media.rating.replace(8.0);

        assert_eq!(letterkenny, media);
    }

    #[tokio::test]
    async fn tmdb_get_cast() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_movies: MetadataProviderOf<Movies> = provider.movies();

        let cast = provider_movies
            .cast("335984")
            .await
            .expect("cast should exist");

        assert_eq!(cast[0].external_id, "30614".to_string());
        assert_eq!(cast[0].name, "Ryan Gosling".to_string());
        assert_eq!(
            cast[0].profile_path,
            Some("https://image.tmdb.org/t/p/original/lyUyVARQKhGxaxy0FbPJCQRpiaW.jpg".to_string())
        );
        assert!(matches!(cast[0].character.as_str(), "K" | "\'K\'"));
    }

    #[tokio::test]
    async fn tmdb_get_seasons() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_shows: MetadataProviderOf<TvShows> = provider.tv_shows();

        let mut media = provider_shows
            .seasons_for_id("63639")
            .await
            .expect("search results should exist");

        assert_eq!(media.len(), 7);

        let last = media.pop().unwrap();

        assert_eq!(last.season_number, 6);

        let expected = ExternalSeason {
            external_id: "214858".into(),
            title: Some("Season 6".into()),
            description: Some("Holden and the crew of the Rocinante fight alongside the Combined Fleet of Earth and Mars to protect the Inner Planets from Marco Inaros and his Free Navy's campaign of death and destruction. Meanwhile, on a distant planet beyond the Rings, a new power rises.".into()),
            posters: vec!["https://image.tmdb.org/t/p/w600_and_h900_bestv2/smJPN02aTJcMVQ4z02CINKjg6L0.jpg".into()],
            season_number: 6
        };

        assert_eq!(last, expected);
    }

    #[tokio::test]
    async fn tmdb_get_episodes() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_shows: MetadataProviderOf<TvShows> = provider.tv_shows();

        let mut media = provider_shows
            .episodes_for_season("63639", 3)
            .await
            .expect("search results should exist");

        assert_eq!(media.len(), 13);

        let last = media.pop().unwrap();

        let expected = ExternalEpisode {
            external_id: "1503262".into(), 
            title: Some("Abaddon's Gate".into()),
            description: Some("Holden and his allies must stop Ashford and his team from destroying the Ring, and perhaps all of humanity.".into()),
            episode_number: 13,
            stills: vec!["https://image.tmdb.org/t/p/original/nE5kS7hHGmv3bTGVL1hlsVQKXo4.jpg".into()],
            duration: None
        };

        assert_eq!(last, expected);
    }

    #[tokio::test]
    async fn once_upon_get_year() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_movies: MetadataProviderOf<Movies> = provider.movies();

        let res = provider_movies
            .search_by_id("466272")
            .await
            .expect("movie should exist");

        assert_eq!(res.release_date.unwrap().year(), 2019);
    }

    #[tokio::test]
    async fn johhny_test_seasons() {
        let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");
        let provider_shows: MetadataProviderOf<TvShows> = provider.tv_shows();

        provider_shows
            .seasons_for_id("1769")
            .await
            .expect("Failed to get seasons.");
    }

    #[tokio::test]
    async fn deserialize_letterkenny() {
        let body = r#"{"id": 1234,"name": "letter kenny"}"#;
        serde_json::from_str::<TMDBMediaObject>(&body).unwrap();
    }
}
