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
use axum::response::Redirect;
use axum::response::Response;
use axum_flash::Flash;
use axum_flash::IncomingFlashes;
use crate::error::DimHtmlErrorWrapper;
use dim_core::errors::DimError;
use dim_database::user::InsertableUser;
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
pub struct LoginTemplate {
    flashes: Vec<(String, String)>,
}

pub async fn login(
    State(AppState { conn, .. }): State<AppState>,
    flashes: IncomingFlashes,
    request: Request<Body>,
) -> (IncomingFlashes, impl IntoResponse) {
    let mut flashes_for_template = Vec::new();
    for (level, text) in flashes.clone().iter() {
        let level_string = format!("{:?}", level);
        flashes_for_template.push((level_string, text.to_owned()));
    }
    match get_cookie_token_value(&request) {
        Some(_) => {
            // If there is a token cookie, redirect to dashboard
            return (
                flashes,
                Redirect::to("/").into_response()
            )
        },
        _ => {}
    }
    if auth::is_admin_exists(conn).await.unwrap_or(false) {
        (
            flashes,
            Html(LoginTemplate {
                flashes: flashes_for_template
            }.render().unwrap()).into_response()
        )
    } else {
        (
            flashes,
            Redirect::to("/register").into_response()
        )
    }
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn handle_login(
    State(AppState { conn, .. }): State<AppState>,
    flash: Flash,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, DimHtmlErrorWrapper> {
    let mut tx = conn.read().begin().await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let user = User::get(&mut tx, &form.username).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let pass = user.get_pass(&mut tx).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    if verify(user.username, pass, form.password) {
        let token = Login::create_cookie(user.id);

        return Ok(
            Response::builder()
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
                .unwrap()
        );
    }

    let message = flash.error("The provided username or password is incorrect.");
    Ok(
        (
            message,
            Redirect::to("/login").into_response()
        ).into_response()
    )
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    admin_exists: bool,
    flashes: Vec<(String, String)>,
}

pub async fn register(
    State(AppState { conn, .. }): State<AppState>,
    flashes: IncomingFlashes,
) -> impl IntoResponse {
    let mut flashes_for_template = Vec::new();
    for (level, text) in flashes.clone().iter() {
        let level_string = format!("{:?}", level);
        flashes_for_template.push((level_string, text.to_owned()));
    }
    (
        flashes,
        Html(RegisterTemplate {
            admin_exists: auth::is_admin_exists(conn).await.unwrap_or(false),
            flashes: flashes_for_template,
        }.render().unwrap()).into_response()
    )
}

pub async fn handle_register(
    State(AppState { conn, .. }): State<AppState>,
    flash: Flash,
    Form(new_user): Form<Login>,
) -> Result<impl IntoResponse, DimHtmlErrorWrapper> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let users_empty = User::get_all(&mut tx).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?.is_empty();

    if !users_empty && (new_user.invite_token.is_none() || !new_user.invite_token_valid(&mut tx).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?) {
        return Ok(
            (
                flash.error(DimError::NoToken.to_string()),
                Redirect::to("/register").into_response()
            ).into_response()
        );
    }

    let roles = dim_database::user::Roles(if !users_empty {
        vec!["user".to_string()]
    } else {
        vec!["owner".to_string()]
    });

    let claimed_invite = if users_empty {
        Login::new_invite(&mut tx).await.map_err(|err| {
            DimHtmlErrorWrapper(DimError::DatabaseError {
                description: err.to_string(),
            })
        })?
    } else {
        match new_user.invite_token {
            Some(token) => token,
            None => {
                return Ok(
                    (
                        flash.error(DimError::NoToken.to_string()),
                        Redirect::to("/register").into_response()
                    ).into_response()
                );
            }
        }
    };

    let res = InsertableUser {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        roles,
        claimed_invite,
        prefs: Default::default(),
    }
    .insert(&mut tx)
    .await
    .map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;

    tx.commit().await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;

    Ok(
        (
            flash.info(format!("Please login with your newly created user: {}.", res.username)),
            Redirect::to("/login").into_response()
        ).into_response()
    )
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
