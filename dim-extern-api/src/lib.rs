//! Library contains a common interface for extracting and obtaining filename metadata as well as
//! the implementations for various external APIs, such as TMDB.

pub mod filename;
pub mod mock;
pub mod tmdb;

use async_trait::async_trait;

use std::fmt::Debug;
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
    /// The API response could not be deserialized: {error}
    DeserializationError { body: Arc<str>, error: String },
    /// No results are found: query={query} year={year:?}
    NoResults { query: String, year: Option<i32> },
    /// No seasons found for the id supplied: {id}
    NoSeasonsFound { id: u64 },
    /// No episodes found for the id supplied: id={id} season={season}
    NoEpisodesFound { id: u64, season: u64 },
    /// Could not find genre with supplied id: {id}
    NoGenreFound { id: u64 },
    /// Other error, usually contains an error that shouldn't happen unless theres a bug.
    // This error wont be ever serialized and sent over the wire, however it should still be
    // printed in logs somewhere as its very unexpected.
    OtherError(#[serde(skip)] Arc<dyn std::error::Error + Send + Sync + 'static>),
    /// The remote API returned an error ({code}): {message}
    RemoteApiError { code: u16, message: String },
}

impl Error {
    pub fn other(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        let err = Arc::new(error);
        Self::OtherError(err)
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
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

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
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

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct ExternalEpisode {
    pub external_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub episode_number: u64,
    pub stills: Vec<String>,
    pub duration: Option<Duration>,
}

impl ExternalEpisode {
    pub fn title_or_episode(&self) -> String {
        self.title
            .clone()
            .unwrap_or_else(|| self.episode_number.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct ExternalActor {
    pub external_id: String,
    pub name: String,
    pub profile_path: Option<String>,
    pub character: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MediaSearchType {
    Movie,
    Tv,
}

impl std::fmt::Display for MediaSearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaSearchType::Movie => write!(f, "movie"),
            MediaSearchType::Tv => write!(f, "tv"),
        }
    }
}

pub trait ExternalQueryIntoShow: IntoQueryShow + ExternalQuery {}

/// Trait that must be implemented by external metadata agents which allows the scanners to query
/// for data.
#[async_trait]
pub trait ExternalQuery: Debug + Send + Sync {
    /// Search by title and year. This must return a Vec of `ExternalMedia` sorted by the search
    /// score.
    async fn search(&self, title: &str, year: Option<i32>) -> Result<Vec<ExternalMedia>>;
    /// Search by external id. This must return a singular `ExternalMedia` which has the id passed
    /// in.
    async fn search_by_id(&self, external_id: &str) -> Result<ExternalMedia>;
    /// Get all actors for a media by external id. Actors must be ordered in order of importance.
    async fn cast(&self, external_id: &str) -> Result<Vec<ExternalActor>>;
}

pub trait IntoQueryShow {
    /// Upcast `self` into `ExternalQueryShow`. It is important that providers that can query for
    /// tv shows, implements this to return `Some(self)`.
    fn as_query_show<'a>(&'a self) -> Option<&'a dyn ExternalQueryShow> {
        None
    }

    fn into_query_show(self: Arc<Self>) -> Option<Arc<dyn ExternalQueryShow>> {
        None
    }
}

/// Trait must be implemented by all external metadata agents which support querying for tv shows.
#[async_trait]
pub trait ExternalQueryShow: ExternalQuery {
    /// Get all seasons for an external id. Seasons must be ranked in order by their number.
    async fn seasons_for_id(&self, external_id: &str) -> Result<Vec<ExternalSeason>>;
    /// Get all episodes for a season ranked in order of the episode number.
    // FIXME: TMDB doesnt support fetching by season id, but rather by season number and tv show
    // id. However other backends could have the opposite situation
    // As such its ideal that we have all external ids follow a standard scheme, for instance a
    // tmdb movie id would look like this `tmdb://12345`, an imdb media id would be similar
    // `imdb://tt1234556`. Season ids would also track their parent media id, so a season id would
    // be like this `tmdb://12345?season_id=32&season=2`, similarly episodes would also track their
    // parent ids, including season id, number, tv show id, episode number and episode id. This is
    // not ideal but it should cover all of the cases.
    //
    // For now this API accepts a external id and season number but this is subject to change.
    async fn episodes_for_season(
        &self,
        external_id: &str,
        season_number: u64,
    ) -> Result<Vec<ExternalEpisode>>;
}
