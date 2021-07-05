use err_derive::Error;

use serde::Serialize;
use serde_json::json;

use std::convert::Infallible;
use std::io::Cursor;

use crate::scanners::base::ScannerError;
use nightfall::error::NightfallError;

use http::Response;
use http::StatusCode;

#[derive(Clone, Debug, Error, Serialize)]
#[serde(tag = "error")]
pub enum DimError {
    #[error(display = "A database error occured")]
    DatabaseError,
    #[error(display = "Some function returned none")]
    NoneError,
    #[error(display = "Some unknown error has occured")]
    UnknownError,
    #[error(display = "An internal server error has occured. Contact your admin.")]
    InternalServerError,
    #[error(display = "An Io error has occured")]
    IOError,
    #[error(display = "The requested resource does not exist.")]
    NotFoundError,
    #[error(display = "Authentication is required for this route.")]
    AuthRequired,
    #[error(display = "Invalid media_type supplied, options are [movie, tv].")]
    InvalidMediaType,
    #[error(display = "A error in the streaming library has occured")]
    StreamingError(#[error(source)] StreamingErrors),
    #[error(display = "You do not have permission to access this route")]
    Unauthorized,
    #[error(display = "A error has occured when matching.")]
    ScannerError(#[error(source)] ScannerError),
}

impl warp::reject::Reject for DimError {}

impl warp::Reply for DimError {
    fn into_response(self) -> warp::reply::Response {
        let status = match self {
            Self::NoneError | Self::NotFoundError => StatusCode::NOT_FOUND,
            Self::StreamingError(_)
            | Self::DatabaseError
            | Self::UnknownError
            | Self::IOError
            | Self::InternalServerError
            | Self::ScannerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthRequired | Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InvalidMediaType => StatusCode::NOT_ACCEPTABLE,
        };

        let resp = json!({
            "error": json!(&self)["error"],
            "messsage": self.to_string(),
        });

        warp::http::Response::builder()
            .status(status)
            .header("ContentType", "application/json")
            .body(serde_json::to_string(&resp).unwrap().into())
            .unwrap()
    }
}

#[derive(Clone, Debug, Error, Serialize)]
#[serde(tag = "error")]
pub enum AuthError {
    #[error(display = "Authentication failed")]
    FailedAuth,
    #[error(display = "A database error occured")]
    DatabaseError,
    #[error(display = "No invite token was supplied, when required")]
    NoTokenError,
    #[error(display = "Admin role required to access this route")]
    Unauthorized,
    #[error(display = "Wrong password")]
    WrongPassword,
    #[error(display = "Username Taken")]
    UsernameTaken,
}

impl warp::reject::Reject for AuthError {}

impl warp::Reply for AuthError {
    fn into_response(self) -> warp::reply::Response {
        let status = match self {
            Self::NoTokenError | Self::UsernameTaken => StatusCode::OK,
            Self::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::WrongPassword | Self::FailedAuth => StatusCode::FORBIDDEN,
        };

        let resp = json!({
            "error": json!(&self)["error"],
            "messsage": self.to_string(),
        });

        warp::http::Response::builder()
            .status(status)
            .header("ContentType", "application/json")
            .body(serde_json::to_string(&resp).unwrap().into())
            .unwrap()
    }
}

#[derive(Clone, Debug, Error, Serialize)]
pub enum StreamingErrors {
    #[error(display = "Failed to start process")]
    ProcFailed,
    #[error(display = "The video profile requested doesnt exist")]
    InvalidProfile,
    #[error(display = "A error with nightfall has occured")]
    OtherNightfall(#[source] NightfallError),
    #[error(display = "It appears that the file is corrupted")]
    FileIsCorrupt,
    #[error(display = "Invalid request")]
    InvalidRequest,
    #[error(display = "Requested session doesnt exist")]
    SessionDoesntExist,
    #[error(display = "InternalServerError")]
    InternalServerError,
    #[error(display = "No mediafile found")]
    NoMediaFileFound(String),
    #[error(display = "Failed to create a ffprobe context")]
    FFProbeCtxFailed,
    #[error(display = "Could not parse the gid")]
    GidParseError,
}

impl warp::reject::Reject for StreamingErrors {}

impl warp::Reply for StreamingErrors {
    fn into_response(self) -> warp::reply::Response {
        let status = match self {
            Self::OtherNightfall(NightfallError::ChunkNotDone) => StatusCode::PROCESSING,
            Self::NoMediaFileFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let resp = json!({
            "error": json!(&self)["error"],
            "messsage": self.to_string(),
        });

        warp::http::Response::builder()
            .status(status)
            .header("ContentType", "application/json")
            .body(serde_json::to_string(&resp).unwrap().into())
            .unwrap()
    }
}

impl From<std::io::Error> for StreamingErrors {
    fn from(_: std::io::Error) -> Self {
        Self::ProcFailed
    }
}

use database::DatabaseError;
impl From<DatabaseError> for DimError {
    fn from(e: DatabaseError) -> Self {
        let DatabaseError::DatabaseError(e) = e;
        Self::DatabaseError
    }
}

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

impl From<DatabaseError> for AuthError {
    fn from(e: DatabaseError) -> Self {
        let DatabaseError::DatabaseError(e) = e;
        Self::DatabaseError
    }
}
