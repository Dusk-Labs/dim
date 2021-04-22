use crate::errors::DimError;

use rocket_contrib::json::JsonValue;
use rocket_contrib::json::json;

#[catch(404)]
pub async fn not_found() -> DimError {
    DimError::NotFoundError
}

#[catch(500)]
pub async fn internal_server_error() -> DimError {
     DimError::InternalServerError
}
