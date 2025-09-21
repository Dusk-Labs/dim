use axum::response::IntoResponse;
use axum::response::Response;
use dim_core::errors::DimError;
use dim_database::DatabaseError;
use http::StatusCode;

/// Wrapper for DimError that implements IntoResponse for Axum compatibility
#[derive(Debug)]
pub struct DimErrorWrapper(pub DimError);

impl From<DimError> for DimErrorWrapper {
    fn from(error: DimError) -> Self {
        Self(error)
    }
}

impl From<DatabaseError> for DimErrorWrapper {
    fn from(error: DatabaseError) -> Self {
        Self(DimError::DatabaseError {
            description: error.to_string(),
        })
    }
}

impl From<sqlx::Error> for DimErrorWrapper {
    fn from(error: sqlx::Error) -> Self {
        Self(DimError::DatabaseError {
            description: error.to_string(),
        })
    }
}

impl From<dim_core::errors::StreamingErrors> for DimErrorWrapper {
    fn from(error: dim_core::errors::StreamingErrors) -> Self {
        Self(DimError::StreamingError(error))
    }
}

impl From<nightfall::error::NightfallError> for DimErrorWrapper {
    fn from(error: nightfall::error::NightfallError) -> Self {
        Self(DimError::StreamingError(
            dim_core::errors::StreamingErrors::OtherNightfall(error),
        ))
    }
}

impl IntoResponse for DimErrorWrapper {
    fn into_response(self) -> Response {
        let status_code = match &self.0 {
            DimError::NotFoundError => StatusCode::NOT_FOUND,
            DimError::Unauthenticated => StatusCode::UNAUTHORIZED,
            DimError::Unauthorized => StatusCode::FORBIDDEN,
            DimError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            DimError::InvalidMediaType => StatusCode::BAD_REQUEST,
            DimError::MissingFieldInBody { .. } => StatusCode::BAD_REQUEST,
            DimError::UnsupportedFile => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            DimError::LibraryNotFound => StatusCode::NOT_FOUND,
            DimError::NoToken => StatusCode::BAD_REQUEST,
            DimError::UsernameNotAvailable => StatusCode::CONFLICT,
            DimError::UploadFailed => StatusCode::INTERNAL_SERVER_ERROR,
            DimError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            DimError::StreamingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DimError::ScannerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DimError::CookieError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = self.0.to_string();
        (status_code, error_message).into_response()
    }
}
