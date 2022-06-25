use std::future::Future;
use std::time::Duration;

use serde::{de::DeserializeOwned, Deserialize};

use crate::external::{ExternalMedia, MediaSearchType};

use super::{TMDBClientRequestError, TMDBMetadataProvider, TMDB_BASE_URL};

// -- TMDB API Data Models

#[derive(Deserialize, Clone, Debug)]
pub struct SearchResponse {
    pub results: Vec<Option<TMDBMediaObject>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TMDBMediaObject {
    pub id: u64,
    #[serde(rename(serialize = "title", deserialize = "name"))]
    pub title: String,
    #[serde(rename(serialize = "release_date", deserialize = "first_air_date"))]
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

impl From<TMDBMediaObject> for ExternalMedia {
    fn from(media: TMDBMediaObject) -> ExternalMedia {
        ExternalMedia {
            external_id: media.id.to_string(),
            title: media.title,
            description: media.overview,
            release_date: media.release_date.and_then(|date| {
                let s = format!("{date} 00:00:00 +0000");
                chrono::DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S %z")
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
            }),
            posters: media.poster_path.into_iter().collect(),
            backdrops: media.backdrop_path.into_iter().collect(),
            genres: media.genres,
            rating: media.vote_average,
            duration: media.runtime.map(|n| Duration::from_secs(n)),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GenreList {
    pub genres: Vec<Genre>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MovieDetails {
    #[serde(flatten)]
    pub media_object: TMDBMediaObject,
}

impl From<MovieDetails> for ExternalMedia {
    fn from(details: MovieDetails) -> Self {
        let MovieDetails { media_object } = details;

        media_object.into()
    }
}

#[derive(Deserialize, Debug)]
pub struct TvDetails {
    #[serde(flatten)]
    pub media_object: TMDBMediaObject,
    pub genres: Option<Vec<Genre>>,
}

impl From<TvDetails> for ExternalMedia {
    fn from(details: TvDetails) -> Self {
        let TvDetails {
            media_object,
            genres,
        } = details;

        let mut media: ExternalMedia = media_object.into();

        media.genres = genres
            .unwrap_or_default()
            .into_iter()
            .map(|genre| genre.name)
            .collect();

        media
    }
}

// -- TMDBClient

/// Internal TMDB client type used for building and making requests.
pub(super) struct TMDBClient {
    pub provider: TMDBMetadataProvider,
}

impl TMDBClient {
    fn make_request<A, T>(
        &self,
        args: A,
        path: String,
    ) -> impl Future<Output = Result<String, TMDBClientRequestError>>
    where
        A: IntoIterator<Item = (T, T)>,
        T: ToString,
    {
        let url = format!("{TMDB_BASE_URL}{path}");
        let args: Vec<_> = args
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let request = self.provider.http_client.get(url).query(&args);

        async move {
            let response = request
                .send()
                .await
                .map_err(TMDBClientRequestError::reqwest)?;

            let body = response
                .bytes()
                .await
                .map_err(TMDBClientRequestError::reqwest)?;

            std::str::from_utf8(&body)
                .map_err(|_| TMDBClientRequestError::InvalidUTF8Body)
                .map(|st| st.to_string())
        }
    }

    pub async fn genre_list(
        &self,
        media_type: MediaSearchType,
    ) -> Result<String, TMDBClientRequestError> {
        self.make_request(
            vec![("api_key", self.provider.api_key.as_ref())],
            format!("/genre/{media_type}/list"),
        )
        .await
    }

    pub async fn search(
        &self,
        media_type: MediaSearchType,
        title: &str,
        year: Option<i32>,
    ) -> Result<String, TMDBClientRequestError> {
        let args = vec![
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
        );

        self.make_request(args, format!("/search/{media_type}"))
            .await
    }

    pub async fn get_details(
        &self,
        media_type: MediaSearchType,
        id: &str,
    ) -> Result<String, TMDBClientRequestError> {
        let args = vec![
            ("api_key", self.provider.api_key.as_ref()),
            ("language", "en-US"),
            ("query", id),
        ];

        self.make_request(args, format!("/{media_type}/{id}")).await
    }
}
