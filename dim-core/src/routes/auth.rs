//! This module contains all docs and APIs related to authentication and user creation.
//!
//! # Request Authentication and Authorization
//! Most API endpoints require a valid JWT authentication token. If no such token is supplied, the
//! API will return [`Unauthenticated`]. Authentication tokens can be obtained by logging in with
//! the [`login`] method. Authentication tokens must be passed to the server through a
//! `Authroization` header.
//!
//! ## Example of an authenticated call
//! ```text
//! curl -X POST http://127.0.0.1:8000/api/v1/auth/whoami -H "Content-type: application/json" -H
//! "Authorization: eyJhb....."
//! ```
//!
//! # Token expiration
//! By default tokens expire after exactly two weeks, once the tokens expire the client must renew
//! them. At the moment renewing the token is only possible by logging in again.
//!
//! [`Unauthenticated`]: crate::errors::DimError::Unauthenticated
//! [`login`]: fn@login
use crate::core::DbConnection;
use crate::errors;

use dim_database::user::verify;
use dim_database::user::InsertableUser;
use dim_database::user::Login;
use dim_database::user::User;

use serde_json::json;

use warp::reply;

pub mod filters {
    use crate::core::DbConnection;

    use warp::reject;
    use warp::Filter;

    use dim_database::user::Login;

    use super::super::global_filters::with_db;

    pub fn login(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "login")
            .and(warp::post())
            .and(warp::body::json::<Login>())
            .and(with_db(conn))
            .and_then(|new_login: Login, conn: DbConnection| async move {
                super::login(new_login, conn)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn register(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "register")
            .and(warp::post())
            .and(warp::body::json::<Login>())
            .and(with_db(conn))
            .and_then(|new_login: Login, conn: DbConnection| async move {
                super::register(new_login, conn)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

/// # POST `/api/v1/auth/login`
/// Method will log a user in and return a authentication token that can be used to authenticate other
/// requests.
///
/// # Request
/// This method accepts a JSON body that deserializes into [`Login`].
///
/// ## Example
/// ```text
/// curl -X POST http://127.0.0.1:8000/api/v1/auth/login -H "Content-type: application/json" -d
/// '{"username": "testuser", "password": "testpassword"}'
/// ```
///
/// # Response
/// If authentication is successful, this method will return status `200 0K` as well as a
/// authentication token.
/// ```no_compile
/// {
///   "token": "...."
/// }
/// ```
///
/// # Errors
/// * [`InvalidCredentials`] - The provided username or password is incorrect.
///
/// [`InvalidCredentials`]: crate::errors::DimError::InvalidCredentials
/// [`Login`]: dim_database::user::Login
pub async fn login(
    new_login: Login,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    let user = User::get(&mut tx, &new_login.username)
        .await
        .map_err(|_| errors::DimError::InvalidCredentials)?;
    let pass = user.get_pass(&mut tx).await?;
    if verify(user.username, pass, new_login.password) {
        let token = dim_database::user::Login::create_cookie(user.id);

        return Ok(reply::json(&json!({
            "token": token,
        })));
    }

    Err(errors::DimError::InvalidCredentials)
}

pub async fn admin_exists(conn: DbConnection) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    Ok(reply::json(&json!({
        "exists": !User::get_all(&mut tx).await?.is_empty()
    })))
}

/// # POST `/api/v1/auth/register`
/// Method will create a new user and return it a authentication token if a user has been
/// successfuly created.
///
/// # Request
/// This method accepts a JSON body that deserializes into [`Login`]. If there are no other users
/// in the database, this route will give the new user `owner` permissions. Additionally this route
/// will not require an invite token.
///
/// If there is a user in the database, this request will require an invite token and the user will
/// be given only `user` permissions.
///
/// ## Example
/// ```text
/// curl -X POST http://127.0.0.1:8000/api/v1/auth/login -H "Content-type: application/json" -d
/// '{"username": "testuser", "password": "testpassword", "invite_token":
/// "72390330-b8af-4413-8305-5f8cae1c8f88"}'
/// ```
///
/// # Response
/// If a user is successfully created, this method will return status `200 0K` as well as the
/// create user's username.
/// ```no_compile
/// {
///   "username": "...."
/// }
/// ```
///
/// # Errors
/// * [`NoToken`] - Either the request doesnt contain an invite token, or the invite token is
/// invalid.
///
/// [`NoToken`]: crate::errors::DimError::NoToken
/// [`Login`]: dim_database::user::Login
pub async fn register(
    new_user: Login,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::DimError> {
    // FIXME: Return INTERNAL SERVER ERROR maybe with a traceback?
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    // NOTE: I doubt this method can faily all the time, we should map server error here too.
    let users_empty = User::get_all(&mut tx).await?.is_empty();

    if !users_empty
        && (new_user.invite_token.is_none() || !new_user.invite_token_valid(&mut tx).await?)
    {
        return Err(errors::DimError::NoToken);
    }

    let roles = dim_database::user::Roles(if !users_empty {
        vec!["user".to_string()]
    } else {
        vec!["owner".to_string()]
    });

    let claimed_invite = if users_empty {
        // NOTE: Double check what we are returning here.
        Login::new_invite(&mut tx).await?
    } else {
        new_user.invite_token.ok_or(errors::DimError::NoToken)?
    };

    let res = InsertableUser {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        roles,
        claimed_invite,
        prefs: Default::default(),
    }
    .insert(&mut tx)
    .await?;

    // FIXME: Return internal server error.
    tx.commit().await?;

    Ok(reply::json(&json!({ "username": res.username })))
}
