use axum::response::IntoResponse;
use axum::response::Response;
use dim_database::DatabaseError;
use displaydoc::Display;
use thiserror::Error;

use serde::Serialize;

use nightfall::error::NightfallError;

use http::StatusCode;

pub trait ErrorStatusCode {
    fn status_code(&self) -> StatusCode;
}

// FIXME: A lot of these errors need to fucking go man.
#[derive(Clone, Display, Debug, Error, Serialize)]
#[serde(tag = "error")]
pub enum DimError {
    /// A database error occured: {description}.
    DatabaseError { description: String },
    /// Some function returned none.
    NoneError,
    /// Some unknown error has occured.
    UnknownError,
    /// Internal server error.
    InternalServerError,
    /// Io Error
    IOError,
    /// The requested resource does not exist.
    NotFoundError,
    /// Authentication is required for this route.
    Unauthenticated,
    /// Invalid Media type supplied.
    InvalidMediaType,
    /// An error in the streaming module has occured
    #[error(transparent)]
    StreamingError(#[from] StreamingErrors),
    /// User has no permission to access this route.
    Unauthorized,
    /// Error has occured when matching: {0:?}
    ScannerError(#[from] crate::scanner::Error),
    /// Upload failed.
    UploadFailed,
    /// Failed to deserialize request body: {description:?}.
    MissingFieldInBody { description: String },
    /// Unsupported file type.
    UnsupportedFile,
    /// Library does not exist.
    LibraryNotFound,
    /// Invite token required.
    NoToken,
    /// Invalid credentials.
    InvalidCredentials,
    /// Requested username is not available.
    UsernameNotAvailable,
    /// An error has occured while parsing cookies: {0:?}
    CookieError(#[source] dim_auth::AuthError),
    /// User does not exist
    UserNotFound,
    /// Couldn't find the tmdb id provided.
    ExternalSearchError(dim_extern_api::Error),
}

impl From<sqlx::Error> for DimError {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError {
            description: format!("{:?}", e),
        }
    }
}

impl From<DatabaseError> for DimError {
    fn from(e: DatabaseError) -> Self {
        Self::DatabaseError {
            description: format!("{:?}", e),
        }
    }
}

// TODO: Clean this up.
impl From<()> for DimError {
    fn from(_: ()) -> Self {
        Self::UnknownError
    }
}

impl From<std::io::Error> for DimError {
    fn from(_: std::io::Error) -> Self {
        Self::IOError
    }
}

impl IntoResponse for DimError {
    fn into_response(self) -> Response {
        match self {
            Self::LibraryNotFound
            | Self::NoneError
            | Self::NotFoundError
            | Self::ExternalSearchError(_) => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            },
            Self::StreamingError(_)
            | Self::DatabaseError { .. }
            | Self::UnknownError
            | Self::IOError
            | Self::InternalServerError
            | Self::UploadFailed
            | Self::ScannerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            },
            Self::Unauthenticated
            | Self::Unauthorized
            | Self::InvalidCredentials
            | Self::CookieError(_)
            | Self::NoToken
            | Self::UserNotFound => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            },
            Self::UsernameNotAvailable => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            },
            Self::UnsupportedFile | Self::InvalidMediaType | Self::MissingFieldInBody { .. } => {
                (StatusCode::NOT_ACCEPTABLE, self.to_string()).into_response()
            }
        }
    }
}

#[derive(Clone, Display, Debug, Error, Serialize)]
#[serde(tag = "error")]
pub enum StreamingErrors {
    /// A database error occured: {0}
    DatabaseError(String),
    /// Failed to start process
    ProcFailed,
    /// The video profile requested doesnt exist
    InvalidProfile,
    /// A error with nightfall has occured
    OtherNightfall(NightfallError),
    /// It appears that the file is corrupted
    FileIsCorrupt,
    /// Invalid request
    InvalidRequest,
    /// Requested session doesnt exist
    SessionDoesntExist,
    /// InternalServerError"
    InternalServerError,
    /// No mediafile found: {0}
    NoMediaFileFound(String),
    /// Failed to create a ffprobe context
    FFProbeCtxFailed,
    /// Could not parse the gid
    GidParseError,
    /// The requested file does not exist on disk.
    FileDoesNotExist,
}

impl From<sqlx::Error> for StreamingErrors {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError(format!("{:?}", e))
    }
}

impl From<NightfallError> for StreamingErrors {
    fn from(e: NightfallError) -> Self {
        Self::OtherNightfall(e)
    }
}

impl From<std::io::Error> for StreamingErrors {
    fn from(_: std::io::Error) -> Self {
        Self::ProcFailed
    }
}

impl IntoResponse for StreamingErrors {
    fn into_response(self) -> Response {
        match self {
            Self::OtherNightfall(NightfallError::ChunkNotDone) => {
                (StatusCode::PROCESSING, self.to_string()).into_response()
            }
            Self::NoMediaFileFound(_) | Self::FileDoesNotExist => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
