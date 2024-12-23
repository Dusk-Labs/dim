use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use dim_core::errors::DimError;
use dim_core::errors::StreamingErrors;
use dim_database::DatabaseError;
use http::StatusCode;

pub struct DimErrorWrapper(pub(crate) DimError);

impl From<DimError> for DimErrorWrapper {
    fn from(value: DimError) -> Self {
        Self(value)
    }
}

impl From<DatabaseError> for DimErrorWrapper {
    fn from(value: DatabaseError) -> Self {
        Self(DimError::DatabaseError {
            description: value.to_string(),
        })
    }
}

impl IntoResponse for DimErrorWrapper {
    fn into_response(self) -> Response {
        use DimError as E;

        let status = match self.0 {
            E::LibraryNotFound | E::NoneError | E::NotFoundError | E::ExternalSearchError(_) => {
                StatusCode::NOT_FOUND
            }
            E::StreamingError(_)
            | E::DatabaseError { .. }
            | E::UnknownError
            | E::IOError
            | E::InternalServerError
            | E::UploadFailed
            | E::ScannerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            E::Unauthenticated
            | E::Unauthorized
            | E::InvalidCredentials
            | E::CookieError(_)
            | E::NoToken
            | E::UserNotFound => StatusCode::UNAUTHORIZED,
            E::UsernameNotAvailable => StatusCode::BAD_REQUEST,
            E::UnsupportedFile | E::InvalidMediaType | E::MissingFieldInBody { .. } => {
                StatusCode::NOT_ACCEPTABLE
            }
        };

        let resp = serde_json::json!({
            "error": serde_json::json!(&self.0)["error"],
            "messsage": self.0.to_string(),
        });
        (status, serde_json::to_string(&resp).unwrap()).into_response()
    }
}

pub struct DimHtmlErrorWrapper(pub(crate) DimError);

impl From<DimError> for DimHtmlErrorWrapper {
    fn from(value: DimError) -> Self {
        Self(value)
    }
}

impl From<StreamingErrors> for DimHtmlErrorWrapper {
    fn from(value: StreamingErrors) -> Self {
        Self(DimError::StreamingError(value))
    }
}

impl From<DatabaseError> for DimHtmlErrorWrapper {
    fn from(value: DatabaseError) -> Self {
        Self(DimError::DatabaseError {
            description: value.to_string(),
        })
    }
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    message: String,
}

impl IntoResponse for DimHtmlErrorWrapper {
    fn into_response(self) -> Response {
        use DimError as E;

        let status = match self.0 {
            E::LibraryNotFound | E::NoneError | E::NotFoundError | E::ExternalSearchError(_) => {
                StatusCode::NOT_FOUND
            }
            E::StreamingError(_)
            | E::DatabaseError { .. }
            | E::UnknownError
            | E::IOError
            | E::InternalServerError
            | E::UploadFailed
            | E::ScannerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            E::Unauthenticated
            | E::Unauthorized
            | E::InvalidCredentials
            | E::CookieError(_)
            | E::NoToken
            | E::UserNotFound => StatusCode::UNAUTHORIZED,
            E::UsernameNotAvailable => StatusCode::BAD_REQUEST,
            E::UnsupportedFile | E::InvalidMediaType | E::MissingFieldInBody { .. } => {
                StatusCode::NOT_ACCEPTABLE
            }
        };

        (
            status,
            Html(
                ErrorTemplate {
                    message: self.0.to_string(),
                }
                .render()
                .unwrap(),
            )
            .into_response(),
        )
            .into_response()
    }
}
