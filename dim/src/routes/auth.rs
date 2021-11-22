use crate::core::DbConnection;
use crate::errors;
use auth::{jwt_generate, Wrapper as Auth};
use bytes::BufMut;

use database::asset::Asset;
use database::asset::InsertableAsset;
use database::progress::Progress;
use database::user::verify;
use database::user::InsertableUser;
use database::user::Login;
use database::user::User;

use serde_json::json;

use warp::reply;

use http::StatusCode;

use futures::TryStreamExt;
use uuid::Uuid;

pub mod filters {
    use crate::core::DbConnection;
    use serde::Deserialize;

    use warp::reject;
    use warp::Filter;

    use database::user::Login;

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
            .and_then(|auth: auth::Wrapper, conn: DbConnection| async move {
                super::whoami(auth, conn)
                    .await
                    .map_err(|e| reject::custom(e))
            })
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

    pub fn user_change_password(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        pub struct Params {
            old_password: String,
            new_password: String,
        }

        warp::path!("api" / "v1" / "auth" / "password")
            .and(warp::patch())
            .and(auth::with_auth())
            .and(warp::body::json::<Params>())
            .and(with_db(conn))
            .and_then(
                |user: auth::Wrapper,
                 Params {
                     old_password,
                     new_password,
                 }: Params,
                 conn: DbConnection| async move {
                    super::user_change_password(conn, user, old_password, new_password)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn admin_delete_token(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "token" / String)
            .and(warp::delete())
            .and(auth::with_auth())
            .and(with_db(conn))
            .and_then(
                |token: String, auth: auth::Wrapper, conn: DbConnection| async move {
                    super::delete_invite(conn, auth, token)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn user_delete_self(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        pub struct Params {
            password: String,
        }

        warp::path!("api" / "v1" / "user" / "delete")
            .and(warp::delete())
            .and(auth::with_auth())
            .and(warp::body::json::<Params>())
            .and(with_db(conn))
            .and_then(
                |auth: auth::Wrapper, Params { password }: Params, conn: DbConnection| async move {
                    super::user_delete_self(conn, auth, password)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn user_change_username(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        pub struct Params {
            new_username: String,
        }
        warp::path!("api" / "v1" / "auth" / "username")
            .and(warp::patch())
            .and(auth::with_auth())
            .and(warp::body::json::<Params>())
            .and(with_db(conn))
            .and_then(|user: auth::Wrapper,
                Params {
                    new_username,
                }: Params,
                conn: DbConnection| async move {
                    super::user_change_username(conn, user, new_username)
                        .await
                        .map_err(|e| reject::custom(e))
                })
    }

    pub fn user_upload_avatar(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "user" / "avatar")
            .and(warp::post())
            .and(auth::with_auth())
            .and(warp::multipart::form().max_length(5_000_000))
            .and(with_db(conn))
            .and_then(|user, form, conn| async move {
                super::user_upload_avatar(conn, user, form)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

pub async fn login(
    new_login: Login,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::AuthError> {
    let mut tx = conn.read().begin().await?;
    let user = User::get(&mut tx, &new_login.username)
        .await
        .map_err(|_| errors::AuthError::UserDoesntExist)?;

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

pub async fn whoami(user: Auth, conn: DbConnection) -> Result<impl warp::Reply, errors::DimError> {
    let username = user.0.claims.get_user();
    let mut tx = conn.read().begin().await?;

    Ok(reply::json(&json!({
        "picture": Asset::get_of_user(&mut tx, &username).await.ok().map(|x| format!("/images/{}", x.local_path)),
        "spentWatching": Progress::get_total_time_spent_watching(&mut tx, username.clone())
            .await
            .unwrap_or(0) / 3600,
        "username": username,
        "roles": user.0.claims.clone_roles()
    })))
}

pub async fn admin_exists(conn: DbConnection) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    Ok(reply::json(&json!({
        "exists": !User::get_all(&mut tx).await?.is_empty()
    })))
}

pub async fn register(
    new_user: Login,
    conn: DbConnection,
) -> Result<impl warp::Reply, errors::AuthError> {
    let mut tx = conn.write().begin().await?;
    let users_empty = User::get_all(&mut tx).await?.is_empty();

    if !users_empty
        && (new_user.invite_token.is_none()
            || !new_user.invite_token_valid(&mut tx).await.unwrap_or(false))
    {
        return Err(errors::AuthError::NoTokenError);
    }

    let roles = if !users_empty {
        vec!["user".to_string()]
    } else {
        vec!["owner".to_string()]
    };

    let claimed_invite = if users_empty {
        Login::new_invite(&mut tx).await?
    } else {
        new_user
            .invite_token
            .ok_or(errors::AuthError::NoTokenError)?
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

    tx.commit().await?;

    Ok(reply::json(&json!({ "username": res })))
}

pub async fn get_all_invites(
    conn: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::AuthError> {
    let mut tx = conn.read().begin().await?;
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

    Err(errors::AuthError::Unauthorized)
}

pub async fn generate_invite(
    conn: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::AuthError> {
    if !user.0.claims.has_role("owner") {
        return Err(errors::AuthError::Unauthorized);
    }

    let mut tx = conn.write().begin().await?;

    let token = Login::new_invite(&mut tx).await?;

    tx.commit().await?;

    Ok(reply::json(&json!({ "token": token })))
}

pub async fn delete_invite(
    conn: DbConnection,
    user: Auth,
    token: String,
) -> Result<impl warp::Reply, errors::AuthError> {
    if !user.0.claims.has_role("owner") {
        return Err(errors::AuthError::Unauthorized);
    }

    let mut tx = conn.write().begin().await?;
    Login::delete_token(&mut tx, token).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn user_change_password(
    conn: DbConnection,
    user: Auth,
    old_password: String,
    new_password: String,
) -> Result<impl warp::Reply, errors::AuthError> {
    let mut tx = conn.write().begin().await?;
    let user = User::get_one(&mut tx, user.0.claims.get_user(), old_password)
        .await
        .map_err(|_| errors::AuthError::WrongPassword)?;
    user.set_password(&mut tx, new_password).await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn user_delete_self(
    conn: DbConnection,
    user: Auth,
    password: String,
) -> Result<impl warp::Reply, errors::AuthError> {
    let mut tx = conn.write().begin().await?;
    let _ = User::get_one(&mut tx, user.0.claims.get_user(), password)
        .await
        .map_err(|_| errors::AuthError::WrongPassword)?;

    User::delete(&mut tx, user.0.claims.get_user()).await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn user_change_username(
    conn: DbConnection,
    user: Auth,
    new_username: String,
) -> Result<impl warp::Reply, errors::AuthError> {
    let mut tx = conn.write().begin().await?;
    if User::get(&mut tx, &new_username).await.is_ok() {
        return Err(errors::AuthError::UsernameTaken);
    }

    User::set_username(&mut tx, user.0.claims.get_user(), new_username).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn user_upload_avatar(
    conn: DbConnection,
    user: Auth,
    form: warp::multipart::FormData,
) -> Result<impl warp::Reply, errors::DimError> {
    let parts: Vec<warp::multipart::Part> = form
        .try_collect()
        .await
        .map_err(|_e| errors::DimError::UploadFailed)?;

    let mut tx = conn.write().begin().await?;
    let asset = if let Some(p) = parts.into_iter().filter(|x| x.name() == "file").next() {
        process_part(&mut tx, p).await
    } else {
        Err(errors::DimError::UploadFailed)
    };

    User::set_picture(&mut tx, user.0.claims.get_user(), asset?.id).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn process_part(
    conn: &mut database::Transaction<'_>,
    p: warp::multipart::Part,
) -> Result<Asset, errors::DimError> {
    if p.name() != "file" {
        return Err(errors::DimError::UploadFailed);
    }

    let file_ext = match dbg!(p.content_type()) {
        Some("image/jpeg" | "image/jpg") => "jpg",
        Some("image/png") => "png",
        _ => return Err(errors::DimError::UnsupportedFile),
    };

    let contents = p
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
        .map_err(|_| errors::DimError::UploadFailed)?;

    let local_file = format!("{}.{}", Uuid::new_v4().to_string(), file_ext);
    let local_path = format!(
        "{}/{}",
        crate::core::METADATA_PATH.get().unwrap(),
        &local_file
    );

    tokio::fs::write(&local_path, contents)
        .await
        .map_err(|_| errors::DimError::UploadFailed)?;

    Ok(InsertableAsset {
        local_path: local_file,
        file_ext: file_ext.into(),
        ..Default::default()
    }
    .insert(conn)
    .await?)
}
