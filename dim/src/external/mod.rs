use async_trait::async_trait;

use std::sync::Arc;
use std::time::Duration;

use displaydoc::Display;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Clone, Display, Debug, Error, Serialize)]
pub enum Error {
    /// The request timeouted
    Timeout,
    /// Max retry count reached
    ReachedMaxTries,
    /// The API response could not be deserialized: {0:?}
    DeserializationError(String),
    /// No results are found: query={query} year={year:?}
    NoResults { query: String, year: Option<i32> },
    /// No seasons found for the id supplied: {id}
    NoSeasonsFound { id: u64 },
    /// No episodes found for the id supplied: id={id} season={season}
    NoEpisodesFound { id: u64, season: u64 },
    /// Could not find genre with supplied id: {id}
    NoGenreFound { id: u64 },
    /// Other error
    OtherError(#[serde(skip)] Arc<dyn std::error::Error>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ExternalMedia {
    /// String representation of the id for this media object.
    pub external_id: String,
    /// The title of this media object.
    pub title: String,
    /// The description or overview of this media object.
    pub description: Option<String>,
    /// The release date or first air date of this media object.
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub release_date: Option<chrono::DateTime<chrono::Utc>>,
    /// A list of posters for this media object.
    pub posters: Vec<String>,
    /// A list of backdrops for this media object.
    pub backdrops: Vec<String>,
    /// A list of genres for this media object.
    pub genres: Vec<String>,
    /// The rating for this media object normalized in the range 0 to 1.
    pub rating: Option<f64>,
    /// The duration for this media object.
    pub duration: Option<Duration>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct ExternalSeason {
    /// String representation of the id for this season object.
    pub external_id: String,
    /// The title of this season.
    pub title: Option<String>,
    /// The description of this season.
    pub description: Option<String>,
    /// A list of posters for this season object.
    pub posters: Vec<String>,
    /// The season number for this season.
    pub season_number: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct ExternalEpisode {
    pub external_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub episode_number: u64,
    pub stills: Vec<String>,
    pub duration: Option<Duration>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct ExternalActor {
    pub external_id: String,
    pub name: String,
    pub character: String,
}

/// Trait that must be implemented by external metadata agents which allows the scanners to query
/// for data.
#[async_trait]
pub trait ExternalQuery {
    /// Search by title and year. This must return a Vec of `ExternalMedia` sorted by the search
    /// score.
    async fn search(&self, title: &str, year: Option<i32>) -> Result<Vec<ExternalMedia>>;
    /// Search by external id. This must return a singular `ExternalMedia` which has the id passed
    /// in.
    async fn search_by_id(&self, external_id: &str) -> Result<ExternalMedia>;
    /// Get all actors for an external id. Actors must be ordered in order of importance.
    async fn actors(&self, external_id: &str) -> Result<Vec<ExternalActor>>;
}

/// Trait must be implemented by all external metadata agents which support querying for tv shows.
#[async_trait]
pub trait ExternalQueryShow: ExternalQuery {
    /// Get all seasons for an external id. Seasons must be ranked in order by their number.
    async fn seasons_for_id(&self, external_id: &str) -> Result<Vec<ExternalSeason>>;
    /// Get all episodes for a season ranked in order of the episode number.
    async fn episodes_for_season(&self, season_id: &str) -> Result<Vec<ExternalEpisode>>;
}
