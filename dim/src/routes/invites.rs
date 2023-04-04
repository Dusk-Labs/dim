//! This module contains various routes used to deal with invite tokens.
//!
//! # What are invite tokens?
//! Invite tokens are random UUID's that server admins can issue to other users such that they can
//! register a new account. An invite token can be created and deleted. An invite token is unique
//! per user and cannot be used twice.
use crate::core::DbConnection;
use crate::errors;
use crate::json;

use dim_database::user::Login;
use dim_database::user::User;

use http::StatusCode;
use warp::reply;

/// # GET `/api/v1/auth/invites`
/// Method will retrieve and return all invite tokens in the database.
///
/// # Authorization
/// This route requires a valid authentication token to be supplied. The token must have `owner`
/// permissions.
///
/// # Request
/// ## Example
/// ```text
/// curl -X GET http://127.0.0.1:8000/api/v1/auth/invites -H "Authorization: ...."
/// ```
///
/// # Response
/// The route will return a response with the following schema
/// ```no_compile
/// [
///   {
///     "id": String,
///     "created": i64,
///     "claimed_by": Option<String>,
///   },
///   ...
/// ]
/// ```
///
/// ## Example
/// ```no_compile
/// [
///   {
///     "id": "079a38b4-d39f-4a9e-9a18-964f225b75d3",
///     "created": 1638708402,
///     "claimed_by": "admin"
///   },
///   {
///     "id": "844caa7b-f54f-a9ea-4444-555555555555",
///     "created": 1640000000,
///   }
/// ]
/// ```
///
/// # Errors
/// * [`Unauthorized`] - Returned if the authentication token lacks `owner` permissions
///
/// [`Unauthorized`]: crate::errors::DimError::Unauthorized
pub async fn get_all_invites(
    conn: DbConnection,
    user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    if user.has_role("owner") {
        #[derive(serde::Serialize)]
        struct Row {
            id: String,
            created: i64,
            claimed_by: Option<String>,
        }

        // FIXME: LEFT JOINs cause sqlx::query! to panic, thus we must get tokens in two queries.
        // TODO: Move these into database.
        // TODO: We silently drop db errors here, we should probably change this.
        let mut row = sqlx::query_as!(
            Row,
            r#"SELECT invites.id, invites.date_added as created, NULL as "claimed_by: _"
                FROM invites
                WHERE invites.id NOT IN (SELECT users.claimed_invite FROM users)
                ORDER BY created ASC"#
        )
        .fetch_all(&mut tx)
        .await
        .unwrap_or_default();

        row.append(
            &mut sqlx::query_as!(
                Row,
                r#"SELECT invites.id, invites.date_added as created, users.username as "claimed_by: Option<String>"
            FROM  invites
            INNER JOIN users ON users.claimed_invite = invites.id"#
            )
            .fetch_all(&mut tx)
            .await
            .unwrap_or_default(),
        );

        return Ok(reply::json(&row));
    }

    Err(errors::DimError::Unauthorized)
}

/// # POST `/api/v1/auth/new_invite`
/// Method will generate and return a new invite token.
///
/// # Authorization
/// This route requires a valid authentication token to be supplied. The token must have `owner`
/// permissions.
///
/// # Request
/// ## Example
/// ```text
/// curl -X POST http://127.0.0.1:8000/api/v1/auth/new_invite -H "Authorization: ...."
/// ```
///
/// # Response
/// The route will return a response with the following schema
/// ```no_compile
/// {
///   "token": String,
/// }
/// ```
///
/// ## Example
/// ```no_compile
/// {
///   "token": "844caa7b-f54f-a9ea-4444-555555555555",
/// }
/// ```
///
/// # Errors
/// * [`Unauthorized`] - Returned if the authentication token lacks `owner` permissions
///
/// [`Unauthorized`]: crate::errors::DimError::Unauthorized
pub async fn generate_invite(
    conn: DbConnection,
    user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    if !user.has_role("owner") {
        return Err(errors::DimError::Unauthorized);
    }

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;

    let token = Login::new_invite(&mut tx).await?;

    tx.commit().await?;

    Ok(reply::json(&json!({ "token": token })))
}

/// # DELETE `/api/v1/auth/token/:token`
/// Method will revoke the supplied token.
///
/// # Authorization
/// This route requires a valid authentication token to be supplied. The token must have `owner`
/// permissions.
///
/// # Request
/// This request takes in a route parameter which is the token we want to delete.
/// ## Example
/// ```text
/// curl -X DELETE http://127.0.0.1:8000/api/v1/auth/token/844caa7b-f54f-a9ea-4444-555555555555 -H "Authorization: ...."
/// ```
///
/// # Response
/// If the token was successfully deleted, this route will return `200 0K`.
///
/// # Errors
/// * [`Unauthorized`] - Returned if the authentication token lacks `owner` permissions
///
/// [`Unauthorized`]: crate::errors::DimError::Unauthorized
pub async fn delete_invite(
    conn: DbConnection,
    user: User,
    token: String,
) -> Result<impl warp::Reply, errors::DimError> {
    if !user.has_role("owner") {
        return Err(errors::DimError::Unauthorized);
    }

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    Login::delete_token(&mut tx, token).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

#[doc(hidden)]
pub(crate) mod filters {
    use super::super::global_filters::with_auth;
    use super::super::global_filters::with_state;
    use dim_database::DbConnection;
    use warp::reject;
    use warp::Filter;

    pub fn get_all_invites(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "invites")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state(conn))
            .and_then(|user, conn| async move {
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
            .and(with_auth(conn.clone()))
            .and(with_state(conn))
            .and_then(|user, conn| async move {
                super::generate_invite(conn, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn delete_token(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "token" / String)
            .and(warp::delete())
            .and(with_auth(conn.clone()))
            .and(with_state(conn))
            .and_then(|token: String, auth, conn| async move {
                super::delete_invite(conn, auth, token)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}
