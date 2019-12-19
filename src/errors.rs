use diesel::result::Error as DieselError;
use err_derive::Error;
use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
};
use serde::Serialize;
use std::io::Cursor;

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
}

#[derive(Debug, Error, Serialize)]
pub enum StreamingErrors {
    #[error(display = "Failed to start process")]
    ProcFailed,
    #[error(display = "The video profile requested doesnt exist")]
    InvalidProfile,
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
            _ => Self::DatabaseError,
        }
    }
}

impl Responder<'static> for DimError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
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
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}

impl Responder<'static> for AuthError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        let status = match self {
            Self::FailedAuth | Self::NoTokenError => Status::Ok,
            Self::DatabaseError => Status::InternalServerError,
            Self::Unauthorized => Status::Unauthorized,
        };

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}
