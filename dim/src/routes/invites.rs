use crate::core::DbConnection;
use crate::errors;
use crate::json;

use auth::Wrapper as Auth;
use database::user::Login;
use warp::reply;
use http::StatusCode;

pub async fn get_all_invites(
    conn: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    if user.0.claims.has_role("owner") {
        #[derive(serde::Serialize)]
        struct Row {
            id: String,
            created: i64,
            claimed_by: Option<String>,
        }

        // FIXME: LEFT JOINs cause sqlx::query! to panic, thus we must get tokens in two queries.
        // TODO: Move these into database.
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
                r#"SELECT invites.id, invites.date_added as created, users.username as claimed_by
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

pub async fn generate_invite(
    conn: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    if !user.0.claims.has_role("owner") {
        return Err(errors::DimError::Unauthorized);
    }

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = database::write_tx(&mut lock).await?;

    let token = Login::new_invite(&mut tx).await?;

    tx.commit().await?;

    Ok(reply::json(&json!({ "token": token })))
}

pub async fn delete_invite(
    conn: DbConnection,
    user: Auth,
    token: String,
) -> Result<impl warp::Reply, errors::DimError> {
    if !user.0.claims.has_role("owner") {
        return Err(errors::DimError::Unauthorized);
    }

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = database::write_tx(&mut lock).await?;
    Login::delete_token(&mut tx, token).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

#[doc(hidden)]
pub(crate) mod filters {
    use database::DbConnection;
    use warp::Filter;
    use warp::reject;
    use super::super::global_filters::with_state;

    pub fn get_all_invites(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "invites")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state(conn))
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
            .and(with_state(conn))
            .and_then(|user: auth::Wrapper, conn: DbConnection| async move {
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
            .and(auth::with_auth())
            .and(with_state(conn))
            .and_then(
                |token: String, auth: auth::Wrapper, conn: DbConnection| async move {
                    super::delete_invite(conn, auth, token)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }
}
