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
}

#[derive(Debug, Error, Serialize)]
#[serde(tag = "error")]
pub enum AuthError {
    #[error(display = "Authentication failed")]
    FailedAuth,
    #[error(display = "A database error occured")]
    DatabaseError,
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
        Response::build()
            .header(ContentType::JSON)
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}

impl Responder<'static> for AuthError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::JSON)
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}
