use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;

use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;
use err_derive::Error;

use serde::Serialize;
use std::io::Cursor;

use nightfall::error::NightfallError;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "error")]
pub enum DimError {
    #[error(display = "A database error occured")]
    DatabaseError,
    #[error(display = "Some function returned none")]
    NoneError,
    #[error(display = "Some unknown error has occured")]
    UnknownError,
    #[error(display = "An Io error has occured")]
    IOError,
    #[error(display = "Database failed to fetch such item")]
    NotFoundError,
    #[error(display = "Authentication is required for this route")]
    AuthRequired,
    #[error(display = "Invalid media_type supplied, options are [movie, tv]")]
    InvalidMediaType,
    #[error(display = "A error in the streaming library has occured")]
    StreamingError(#[error(source)] StreamingErrors),
}

#[derive(Debug, Error, Serialize)]
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

#[derive(Debug, Error, Serialize)]
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
}

impl From<std::io::Error> for StreamingErrors {
    fn from(_: std::io::Error) -> Self {
        Self::ProcFailed
    }
}

impl From<DieselError> for DimError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => Self::NotFoundError,
            _ => Self::DatabaseError,
        }
    }
}

use database::DatabaseError;
impl From<DatabaseError> for DimError {
    fn from(e: DatabaseError) -> Self {
        let DatabaseError::AsyncError(e) = e;
        Self::from(e)
    }
}

impl From<tokio_diesel::AsyncError> for DimError {
    fn from(e: tokio_diesel::AsyncError) -> Self {
        match e {
            tokio_diesel::AsyncError::Error(e) => Self::from(e),
            _ => Self::UnknownError,
        }
    }
}

impl From<std::option::NoneError> for DimError {
    fn from(_: std::option::NoneError) -> Self {
        Self::NoneError
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

impl From<DieselError> for AuthError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => Self::FailedAuth,
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                Self::UsernameTaken
            }
            _ => Self::DatabaseError,
        }
    }
}

impl From<DatabaseError> for AuthError {
    fn from(e: DatabaseError) -> Self {
        let DatabaseError::AsyncError(e) = e;
        Self::from(e)
    }
}

impl From<tokio_diesel::AsyncError> for AuthError {
    fn from(e: tokio_diesel::AsyncError) -> Self {
        match e {
            tokio_diesel::AsyncError::Error(e) => Self::from(e),
            _ => Self::DatabaseError,
        }
    }
}

impl<'r> Responder<'r, 'static> for DimError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let status = match self {
            Self::NoneError | Self::NotFoundError => Status::NotFound,
            Self::StreamingError(_) | Self::DatabaseError | Self::UnknownError | Self::IOError => {
                Status::InternalServerError
            }
            Self::AuthRequired => Status::Unauthorized,
            Self::InvalidMediaType => Status::NotModified,
        };

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .streamed_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}

impl<'r> Responder<'r, 'static> for AuthError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let status = match self {
            Self::NoTokenError => Status::Ok,
            Self::UsernameTaken => Status::Ok,
            Self::DatabaseError => Status::InternalServerError,
            Self::Unauthorized => Status::Unauthorized,
            Self::WrongPassword | Self::FailedAuth => Status::Forbidden,
        };

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .streamed_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}

impl<'r> Responder<'r, 'static> for StreamingErrors {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let status = match self {
            Self::OtherNightfall(NightfallError::ChunkNotDone) => Status::Processing,
            Self::NoMediaFileFound(_) => Status::NotFound,
            _ => Status::InternalServerError,
        };

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .streamed_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}
