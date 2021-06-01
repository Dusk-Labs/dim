use crate::core::DbConnection;
use crate::errors;
use auth::{jwt_generate, Wrapper as Auth};

use database::progress::Progress;
use database::user::verify;
use database::user::InsertableUser;
use database::user::Login;
use database::user::User;

use diesel::prelude::*;
use tokio_diesel::*;

use serde_json::json;

use warp::reject;
use warp::reply;
use warp::reply::Json;
use warp::Filter;

use std::convert::Infallible;

pub fn auth_routes(
    conn: DbConnection,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::login(conn.clone())
        .or(filters::whoami(conn.clone()))
        .or(filters::admin_exists(conn.clone()))
        .or(filters::register(conn.clone()))
        .or(filters::get_all_invites(conn.clone()))
        .or(filters::generate_invite(conn.clone()))
        .recover(filters::handle_rejection)
}

mod filters {
    use crate::core::DbConnection;
    use serde::Deserialize;

    use warp::reject;
    use warp::Filter;

    use database::user::Login;

    use std::convert::Infallible;

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

    pub fn whoami(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "whoami")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_db(conn))
            .and_then(super::whoami)
    }

    pub fn admin_exists(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "admin_exists")
            .and(warp::get())
            .and(with_db(conn))
            .and_then(|conn: DbConnection| async move {
                super::admin_exists(conn)
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

    pub fn get_all_invites(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "invites")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_db(conn))
            .and_then(|user: auth::Wrapper, conn: DbConnection| async move {
                super::get_all_invites(conn, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn generate_invite(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "new_invite")
            .and(warp::post())
            .and(auth::with_auth())
            .and(with_db(conn))
            .and_then(|user: auth::Wrapper, conn: DbConnection| async move {
                super::generate_invite(conn, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    fn with_db(
        conn: DbConnection,
    ) -> impl Filter<Extract = (DbConnection,), Error = Infallible> + Clone {
        warp::any().map(move || conn.clone())
    }

    pub async fn handle_rejection(
        err: warp::reject::Rejection,
    ) -> Result<impl warp::Reply, warp::reject::Rejection> {
        if let Some(e) = err.find::<crate::errors::AuthError>() {
            return Ok(e.clone());
        }

        Err(err)
    }
}

pub async fn login(
    new_login: Login,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::AuthError> {
    use database::schema::users::dsl::*;
    let uname = new_login.username.clone();
    let user: (String, String, String) =
        users.filter(username.eq(uname)).first_async(&conn).await?;

    if verify(user.0.clone(), user.1.clone(), new_login.password.clone()) {
        let token = jwt_generate(user.0, user.2.split(",").map(|x| x.to_string()).collect());

        return Ok(reply::json(&json!({
            "token": token,
        })));
    }

    Err(errors::AuthError::WrongPassword)
}

pub async fn whoami(user: Auth, conn: DbConnection) -> Result<impl warp::Reply, Infallible> {
    Ok(reply::json(&json!({
        "username": user.0.claims.get_user(),
        "picture": "https://i.redd.it/3n1if40vxxv31.png",
        "spentWatching": Progress::get_total_time_spent_watching(&conn, user.0.claims.get_user()).await.unwrap_or(0) / 3600
    })))
}

pub async fn admin_exists(conn: DbConnection) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(&json!({
        "exists": !User::get_all(&conn).await?.is_empty()
    })))
}

pub async fn register(
    new_user: Login,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::AuthError> {
    let users_empty = User::get_all(&conn).await?.is_empty();

    if !users_empty
        && (new_user.invite_token.is_none()
            || !new_user.invite_token_valid(&conn).await.unwrap_or(false))
    {
        return Err(errors::AuthError::NoTokenError);
    }

    let roles = if !users_empty {
        vec!["user".to_string()]
    } else {
        vec!["owner".to_string()]
    };

    let res = InsertableUser {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        roles,
    }
    .insert(&conn)
    .await?;

    if users_empty {
        new_user.invalidate_token(&conn).await?;
    }

    Ok(reply::json(&json!({ "username": res })))
}

pub async fn get_all_invites(
    conn: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::AuthError> {
    if user.0.claims.has_role("owner") {
        return Ok(reply::json(&json!({
            "invites": Login::get_all_invites(&conn).await?
        })));
    }

    Err(errors::AuthError::Unauthorized)
}

pub async fn generate_invite(
    conn: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::AuthError> {
    if !user.0.claims.has_role("owner") {
        return Err(errors::AuthError::Unauthorized);
    }

    Ok(reply::json(&json!({
        "token": Login::new_invite(&conn).await?
    })))
}
