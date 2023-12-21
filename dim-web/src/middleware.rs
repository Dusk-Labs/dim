use crate::AppState;
use axum::body::Body;
use axum::http::Request;
use axum::extract::State;
use axum::response::Redirect;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_flash::Flash;
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
    flash: Flash,
    mut req: axum::http::Request<Body>,
    next: axum::middleware::Next<Body>,
) -> Result<axum::response::Response, DimErrorWrapper> {
    let is_html_request: bool = if let Some(accept_str) = req.headers().get(axum::http::header::ACCEPT) {
        accept_str.to_str().unwrap().contains("text/html")
    } else {
        false
    };
    let id: UserID;
    if let Some(token) = get_cookie_token_value(&req) {
        id = match dim_database::user::Login::verify_cookie(token) {
            Ok(id) => id,
            Err(e) => {
                let error = DimError::CookieError(e);
                if is_html_request {
                    return Ok(
                        (
                            flash.error(error.to_string()),
                            Redirect::to("/login").into_response()
                        ).into_response()
                    );
                }
                return Err(DimErrorWrapper(error));
            }
        };
    } else if let Some(token) = req.headers().get(axum::http::header::AUTHORIZATION) {
        id = match dim_database::user::Login::verify_cookie(token.to_str().unwrap().to_string()) {
            Ok(id) => id,
            Err(e) => {
                let error = DimError::CookieError(e);
                if is_html_request {
                    return Ok(
                        (
                            flash.error(error.to_string()),
                            Redirect::to("/login").into_response()
                        ).into_response()
                    );
                }
                return Err(DimErrorWrapper(error));
            }
        };
    } else {
        let error = DimError::NoToken;
        if is_html_request {
            return Ok(
                (
                    flash.error(error.to_string()),
                    Redirect::to("/login").into_response()
                ).into_response()
            );
        }
        return Err(DimErrorWrapper(error));
    }

    let mut tx = match conn.read().begin().await {
        Ok(tx) => tx,
        Err(_) => {
            let error = DimError::DatabaseError {
                description: String::from("Failed to start transaction"),
            };
            if is_html_request {
                return Ok(
                    (
                        flash.error(error.to_string()),
                        Redirect::to("/login").into_response()
                    ).into_response()
                );
            }
            return Err(DimErrorWrapper(error))
        }
    };
    let current_user = match dim_database::user::User::get_by_id(&mut tx, id).await {
        Ok(current_user) => current_user,
        Err(_) => {
            let error = DimError::UserNotFound;
            if is_html_request {
                return Ok(
                    (
                        flash.error(error.to_string()),
                        Redirect::to("/login").into_response()
                    ).into_response()
                );
            }
            return Err(DimErrorWrapper(error));
        }
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
