use crate::AppState;
use crate::routes::auth;
use askama::Template;
use axum::body;
use axum::body::Empty;
use axum::body::Body;
use axum::http::Request;
use axum::extract::Form;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use dim_database::user::User;
use dim_database::user::Login;
use dim_database::user::verify;
use crate::middleware::get_cookie_token_value;
use serde::Deserialize;
use http::StatusCode;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

pub async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

pub async fn login(
    State(AppState { conn, .. }): State<AppState>,
    request: Request<Body>,
) -> impl IntoResponse {
    match get_cookie_token_value(&request) {
        Some(_) => {
            // If there is a token cookie, redirect to dashboard
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header("Location", "/")
                .body(body::boxed(Empty::new()))
                .unwrap()
        },
        _ => {}
    }
    if auth::is_admin_exists(conn).await.unwrap_or(false) {
        Html(LoginTemplate {}.render().unwrap()).into_response()
    } else {
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", "/register")
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn handle_login(
    State(AppState { conn, .. }): State<AppState>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    match conn.read().begin().await {
        Ok(mut tx) => {
            match User::get(&mut tx, &form.username).await {
                Ok(user) => {
                    match user.get_pass(&mut tx).await {
                        Ok(pass) => {
                            if verify(user.username, pass, form.password) {
                                let token = Login::create_cookie(user.id);

                                return Response::builder()
                                    .status(StatusCode::SEE_OTHER)
                                    .header("Location", "/")
                                    // Set token cookie max age to 1 year
                                    .header(
                                        "Set-Cookie",
                                        format!(
                                            "token={}; Path=/; Max-Age={}; SameSite=Strict; HttpOnly",
                                            token,
                                            31536000)
                                    )
                                    .body(body::boxed(Empty::new()))
                                    .unwrap();
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    };

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/login")
        .body(body::boxed(Empty::new()))
        .unwrap()
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    admin_exists: bool,
}

pub async fn register(
    State(AppState { conn, .. }): State<AppState>,
) -> impl IntoResponse {
    RegisterTemplate {
        admin_exists: auth::is_admin_exists(conn).await.unwrap_or(false)
    }
}

pub async fn handle_logout() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/login")
        .header(
            "Set-Cookie",
            "token=TO_BE_DELETED; Path=/; Max-Age=-1; SameSite=Strict; HttpOnly"
        )
        .body(body::boxed(Empty::new()))
        .unwrap()
}
