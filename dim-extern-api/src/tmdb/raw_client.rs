use retry_block::async_retry;
use retry_block::delay::Fixed;
use retry_block::OperationResult;

use serde::Deserialize;
use std::future::Future;
use std::time::Duration;

use crate::{ExternalActor, ExternalEpisode, ExternalMedia, ExternalSeason, MediaSearchType};

use super::{TMDBClientRequestError, TMDBMetadataProvider, TMDB_BASE_URL};

// -- TMDB API Data Models

#[derive(Deserialize, Clone, Debug)]
pub struct SearchResponse {
    pub results: Vec<Option<TMDBMediaObject>>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct TMDBMediaObject {
    pub id: u64,
    #[serde(alias = "name")]
    pub title: String,
    #[serde(alias = "first_air_date")]
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub vote_average: Option<f64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<u64>>,
    pub genres: Option<Vec<Genre>>,
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
            posters: media
                .poster_path
                .into_iter()
                .map(|x| format!("https://image.tmdb.org/t/p/w600_and_h900_bestv2{x}"))
                .collect(),
            backdrops: media
                .backdrop_path
                .into_iter()
                .map(|x| format!("https://image.tmdb.org/t/p/original{x}"))
                .collect(),
            genres: media
                .genres
                .unwrap_or_default()
                .into_iter()
                .map(|genre| genre.name)
                .collect(),
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
pub struct CastActor {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub character: String,
    pub cast_id: u64,
    pub gender: u64,
    pub adult: bool,
    pub profile_path: Option<String>,
    pub order: u64,
}

impl From<CastActor> for ExternalActor {
    fn from(actor: CastActor) -> Self {
        let CastActor {
            id,
            name,
            profile_path,
            character,
            ..
        } = actor;

        ExternalActor {
            name,
            character,
            external_id: id.to_string(),
            profile_path: profile_path.map(|x| format!("https://image.tmdb.org/t/p/original{x}")),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Cast {
    pub id: u64,
    pub cast: Vec<CastActor>,
}

#[derive(Deserialize, Debug)]
pub struct TmdbError {
    pub status_message: String,
    pub status_code: u64,
}

#[derive(Deserialize, Debug)]
pub struct TvSeasons {
    pub seasons: Vec<TvSeason>,
}

impl From<TvSeasons> for Vec<ExternalSeason> {
    fn from(seasons: TvSeasons) -> Self {
        let TvSeasons { seasons } = seasons;
        seasons.into_iter().map(Into::into).collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct TvSeason {
    pub id: u64,
    pub air_date: Option<String>,
    pub episode_count: u64,
    pub name: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: u64,
}

impl From<TvSeason> for ExternalSeason {
    fn from(season: TvSeason) -> Self {
        let TvSeason {
            id,
            name,
            overview,
            poster_path,
            season_number,
            ..
        } = season;

        Self {
            external_id: id.to_string(),
            title: Some(name),
            description: overview,
            posters: poster_path
                .map(|x| {
                    vec![format!(
                        "https://image.tmdb.org/t/p/w600_and_h900_bestv2{x}"
                    )]
                })
                .unwrap_or_default(),
            season_number,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TvEpisodes {
    pub episodes: Vec<TvEpisode>,
}

impl From<TvEpisodes> for Vec<ExternalEpisode> {
    fn from(episodes: TvEpisodes) -> Self {
        let TvEpisodes { episodes } = episodes;

        episodes.into_iter().map(Into::into).collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct TvEpisode {
    pub id: u64,
    pub name: Option<String>,
    pub episode_number: u64,
    pub overview: Option<String>,
    pub still_path: Option<String>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<u64>,
}

impl From<TvEpisode> for ExternalEpisode {
    fn from(episode: TvEpisode) -> Self {
        let TvEpisode {
            id,
            name,
            episode_number,
            overview,
            still_path,
            ..
        } = episode;

        Self {
            external_id: id.to_string(),
            title: name,
            description: overview,
            stills: still_path
                .map(|x| vec![format!("https://image.tmdb.org/t/p/original{x}")])
                .unwrap_or_default(),
            episode_number,
            duration: None,
        }
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

        let provider = self.provider.http_client.clone();

        async move {
            let result = async_retry!(Fixed::new(Duration::from_millis(50)).take(24), {
                let request = provider.get(url.clone()).query(&args);
                let response = match request
                    .send()
                    .await
                    .map_err(TMDBClientRequestError::reqwest)
                {
                    Ok(x) => x,
                    Err(err) => return Err(err).into(),
                };

                let status = response.status();

                let body = match response
                    .bytes()
                    .await
                    .map_err(TMDBClientRequestError::reqwest)
                {
                    Ok(x) => x,
                    Err(err) => return Err(err).into(),
                };

                let body = std::str::from_utf8(&body)
                    .map_err(|_| TMDBClientRequestError::InvalidUTF8Body)
                    .map(|st| st.to_string());

                if status != reqwest::StatusCode::OK {
                    return Err(TMDBClientRequestError::NonOkResponse {
                        body: body.unwrap_or_default(),
                        status,
                    })
                    .into();
                }

                match body {
                    Ok(x) => OperationResult::Ok(x),
                    Err(err) => OperationResult::Err(err),
                }
            });

            result
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

    pub async fn get_actor(
        &self,
        media_type: MediaSearchType,
        id: &str,
    ) -> Result<String, TMDBClientRequestError> {
        let args = vec![
            ("api_key", self.provider.api_key.as_ref()),
            ("language", "en-US"),
        ];

        self.make_request(args, format!("/{media_type}/{id}/credits"))
            .await
    }

    pub async fn get_episodes(
        &self,
        id: &str,
        season_number: u64,
    ) -> Result<String, TMDBClientRequestError> {
        let args = vec![
            ("api_key", self.provider.api_key.as_ref()),
            ("language", "en-US"),
        ];

        self.make_request(args, format!("/tv/{id}/season/{season_number}"))
            .await
    }
}
