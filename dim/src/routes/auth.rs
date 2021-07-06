use crate::core::DbConnection;
use crate::errors;
use auth::{jwt_generate, Wrapper as Auth};

use database::progress::Progress;
use database::user::verify;
use database::user::InsertableUser;
use database::user::Login;
use database::user::User;

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
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use crate::core::DbConnection;
    use serde::Deserialize;

    use warp::reject;
    use warp::Filter;

    use database::user::Login;

    use std::convert::Infallible;

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
}

pub async fn login(
    new_login: Login,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::AuthError> {
    let user = User::get(&conn, &new_login.username).await?;

    if verify(
        user.username.clone(),
        user.password.clone(),
        new_login.password.clone(),
    ) {
        let token = jwt_generate(user.username, user.roles.clone());

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

    let claimed_invite = if users_empty {
        Login::new_invite(&conn).await?
    } else {
        new_user
            .invite_token
            .ok_or(errors::AuthError::NoTokenError)?
    };

    let res = InsertableUser {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        roles,
        claimed_invite: claimed_invite,
        prefs: Default::default(),
    }
    .insert(&conn)
    .await?;

    Ok(reply::json(&json!({ "username": res })))
}

pub async fn get_all_invites(
    conn: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::AuthError> {
    if user.0.claims.has_role("owner") {
        #[derive(serde::Serialize)]
        struct Row {
            id: String,
            created: i64,
            claimed_by: Option<String>,
        }

        // FIXME: LEFT JOINs cause sqlx::query! to panic, thus we must get tokens in two queries.
        let mut row = sqlx::query_as!(
            Row,
            r#"SELECT invites.id, invites.date_added as created, NULL as "claimed_by: _"
                FROM invites
                WHERE invites.id NOT IN (SELECT users.claimed_invite FROM users)
                ORDER BY created ASC"#
        )
        .fetch_all(&conn)
        .await
        .unwrap_or_default();

        row.append(
            &mut sqlx::query_as!(
                Row,
                r#"SELECT invites.id, invites.date_added as created, users.username as claimed_by
            FROM  invites
            INNER JOIN users ON users.claimed_invite = invites.id"#
            )
            .fetch_all(&conn)
            .await
            .unwrap_or_default(),
        );

        return Ok(reply::json(&row));
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
