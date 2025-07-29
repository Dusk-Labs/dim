use crate::AppState;
use axum::body::Body;
use axum::http::Request;
use axum::extract::State;
use axum_extra::extract::cookie::Cookie;
use dim_core::errors::DimError;
use dim_database::user::UserID;

use crate::DimErrorWrapper;

pub fn get_cookie_token_value(
    request: &Request<Body>,
) -> Option<String> {
    request
        .headers()
        .get_all("Cookie")
        .iter()
        .filter_map(|cookie| {
            cookie
                .to_str()
                .ok()
                .and_then(|cookie| cookie.parse::<Cookie>().ok())
        })
        .find_map(|cookie| {
            (cookie.name() == "token").then(move || cookie.value().to_owned())
        })
}

pub async fn verify_token(
    State(AppState { conn, .. }): State<AppState>,
    mut req: axum::http::Request<Body>,
    next: axum::middleware::Next<Body>,
) -> Result<axum::response::Response, DimErrorWrapper> {
    let id: UserID;
    if let Some(token) = get_cookie_token_value(&req) {
        id = dim_database::user::Login::verify_cookie(token)
            .map_err(|e| DimError::CookieError(e))
            .map_err(|e| DimErrorWrapper(e))?;
    } else if let Some(token) = req.headers().get(axum::http::header::AUTHORIZATION) {
        id = dim_database::user::Login::verify_cookie(token.to_str().unwrap().to_string())
            .map_err(|e| DimError::CookieError(e))
            .map_err(|e| DimErrorWrapper(e))?;
    } else {
        return Err(DimErrorWrapper(DimError::NoToken));
    }

    let mut tx = match conn.read().begin().await {
        Ok(tx) => tx,
        Err(_) => {
            return Err(DimErrorWrapper(DimError::DatabaseError {
                description: String::from("Failed to start transaction"),
            }))
        }
    };
    let current_user = dim_database::user::User::get_by_id(&mut tx, id)
        .await
        .map_err(|_| DimError::UserNotFound)
        .map_err(|e| DimErrorWrapper(e))?;

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
