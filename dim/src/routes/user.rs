//! This module contains all docs and APIs related to users and user metadata.
use crate::core::DbConnection;
use crate::errors;
use bytes::BufMut;

use dim_database::asset::Asset;
use dim_database::asset::InsertableAsset;
use dim_database::progress::Progress;
use dim_database::user::User;

use serde_json::json;

use warp::reply;

use http::StatusCode;

use futures::TryStreamExt;
use uuid::Uuid;

/// # GET `/api/v1/user`
/// Method returns metadata about the currently logged in user.
///
/// # Request
/// This method takes in no additional parameters or data.
///
/// ## Authorization
/// This method requires a valid authentication token.
///
/// ## Example
/// ```text
/// curl -X GET http://127.0.0.1:8000/api/v1/user -H "Authorization: ..."
/// ```
///
/// # Response
/// This method will return a JSON payload with the following schema:
/// ```no_compile
/// {
///   "picture": Option<String>,
///   "spentWatching": i64,
///   "username": String,
///   "roles": [String]
/// }
/// ```
///
/// ## Example
/// ```no_compile
/// {
///   "picture": "/images/avatar.jpg",
///   "spentWatching": 12,
///   "username": "admin",
///   "roles": ["owner"],
/// }
/// ```
pub async fn whoami(user: User, conn: DbConnection) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;

    Ok(reply::json(&json!({
        "picture": Asset::get_of_user(&mut tx, user.id).await.ok().map(|x| format!("/images/{}", x.local_path)),
        "spentWatching": Progress::get_total_time_spent_watching(&mut tx, user.id)
            .await
            .unwrap_or(0) / 3600,
        "username": user.username,
        "roles": user.roles()
    })))
}

/// # POST `/api/v1/user/password`
/// Method changes the password for a logged in account.
///
/// # Request
/// This method accepts a JSON body with the following schema:
/// ```no_compile
/// {
///   "old_password": String,
///   "new_password": String,
/// }
/// ```
/// The `old_password` field in the JSON payload must be the currently registered password for this
/// user. The `new_password` field is the new password that we want to set.
///
/// ## Example
/// ```text
/// curl -X POST http://127.0.0.1:8000/api/v1/user/password -H "Content-type: application/json"
/// -H "Authroization: ..." -d '{"old_password": "testPass", "new_password": "newTestPass"}'
/// ```
///
/// # Response
/// If the password is successfully changed, the method will simply return `200 0K`.
///
/// # Errors
/// * [`InvalidCredentials`] - The provided `old_password` is incorrect or the authentication token
/// is invalid.
///
/// [`InvalidCredentials`]: crate::errors::DimError::InvalidCredentials
pub async fn change_password(
    conn: DbConnection,
    user: User,
    old_password: String,
    new_password: String,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;

    let user = User::authenticate(&mut tx, user.username, old_password)
        .await
        .map_err(|_| errors::DimError::InvalidCredentials)?;

    user.set_password(&mut tx, new_password).await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}

/// # DELETE `/api/v1/user`
/// Method deletes the currently logged in account.
///
/// # Request
/// This method accepts a JSON body with the following schema:
/// ```no_compile
/// {
///   "password": String,
/// }
/// ```
/// The `password` field in the JSON payload must be the currently registered password for this
/// user. This is required as a safety mechanism to avoid accidental account deletion.
///
/// ## Example
/// ```text
/// curl -X DELETE http://127.0.0.1:8000/api/v1/user -H "Content-type: application/json" -H "Authroization: ..."
/// -d '{"password": "testPass"}'
/// ```
///
/// # Response
/// If the account is successfully deleted, the method will simply return `200 0K`.
///
/// # SAFETY and caveats
/// Because Dim uses JWTs for authorization, deleting an account doesnt mean the authorization
/// token is also revoked as JWTs are stateless by design. Because of this, users must ensure that
/// the token is cleared from memory and is not *EVER* reused.
///
/// # Errors
/// * [`InvalidCredentials`] - The provided `old_password` is incorrect or the authentication token
/// is invalid.
///
/// [`InvalidCredentials`]: crate::errors::DimError::InvalidCredentials
pub async fn delete(
    conn: DbConnection,
    user: User,
    password: String,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;

    let user = User::authenticate(&mut tx, user.username, password)
        .await
        .map_err(|_| errors::DimError::InvalidCredentials)?;

    User::delete(&mut tx, user.id).await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}

/// # POST `/api/v1/user/username`
/// Method changes the username of the current account.
///
/// # Request
/// This method accepts a JSON payload with the following schema:
/// ```no_compile
/// {
///   "new_username": String
/// }
/// ```
///
/// ## Example
/// ```text
/// curl -X POST http://127.0.0.1:8000/api/v1/user/username -H "Content-type: application/json" -H
/// "Authorization: ..." -d '{"new_username": "testUsername"}'
/// ```
///
/// # Response
/// If the username is successfully changed this method will simply return `200 OK`.
///
/// # Errors
/// * [`UsernameNotAvailable`] - THe provided username has already been claimed by another user.
///
/// [`UsernameNotAvailable`]: crate::errors::DimError::UsernameNotAvailable
pub async fn change_username(
    conn: DbConnection,
    user: User,
    new_username: String,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    if User::get(&mut tx, &new_username).await.is_ok() {
        return Err(errors::DimError::UsernameNotAvailable);
    }

    User::set_username(&mut tx, user.username.clone(), new_username).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

/// # POST `/api/v1/user/avatar`
/// This method can be used to set a new avatar for a user.
///
/// # Request
/// This method accepts a multipart file upload. Only `jpg` and `png` files are supported.
///
/// ## Example
/// ```text
/// curl -X POST http://127.0.0.1:8000/api/v1/user/avatar -H "Authorization: ..." --form
/// file='@newAvatar.png'
/// ```
///
/// # Response
/// If the avatar is successfully uploaded, this route will return `200 OK`.
///
/// # Errors
/// * [`UploadFailed`] - No file has been uploaded correctly or the `file` form field has not been
/// * [`UnsupportedFile`] - The file uploaded is not supported.
/// found.
///
/// [`UploadFailed`]: crate::errors::DimError::UploadFailed
/// [`UnsupportedFile`]: crate::errors::DimError::UnsupportedFile
pub async fn upload_avatar(
    conn: DbConnection,
    user: User,
    form: warp::multipart::FormData,
) -> Result<impl warp::Reply, errors::DimError> {
    let parts: Vec<warp::multipart::Part> = form
        .try_collect()
        .await
        .map_err(|_e| errors::DimError::UploadFailed)?;

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    let asset = if let Some(p) = parts.into_iter().filter(|x| x.name() == "file").next() {
        process_part(&mut tx, p).await
    } else {
        Err(errors::DimError::UploadFailed)
    };

    User::set_picture(&mut tx, user.id, asset?.id).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

#[doc(hidden)]
pub async fn process_part(
    conn: &mut dim_database::Transaction<'_>,
    p: warp::multipart::Part,
) -> Result<Asset, errors::DimError> {
    if p.name() != "file" {
        return Err(errors::DimError::UploadFailed);
    }

    let file_ext = match p.content_type() {
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
    .insert_local_asset(conn)
    .await?)
}

#[doc(hidden)]
pub(crate) mod filters {
    use crate::core::DbConnection;
    use serde::Deserialize;

    use dim_database::user::User;

    use warp::reject;
    use warp::Filter;

    use super::super::global_filters::with_auth;
    use super::super::global_filters::with_state;

    pub fn whoami(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let path =
            warp::path!("api" / "v1" / "user").or(warp::path!("api" / "v1" / "auth" / "whoami"));

        path.unify()
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state(conn))
            .and_then(|auth: User, conn: DbConnection| async move {
                super::whoami(auth, conn)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn change_password(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        pub struct Params {
            old_password: String,
            new_password: String,
        }

        warp::path!("api" / "v1" / "user" / "password")
            .and(warp::patch())
            .and(with_auth(conn.clone()))
            .and(warp::body::json::<Params>())
            .and(with_state(conn))
            .and_then(
                |user: User,
                 Params {
                     old_password,
                     new_password,
                 }: Params,
                 conn: DbConnection| async move {
                    super::change_password(conn, user, old_password, new_password)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn delete(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        pub struct Params {
            password: String,
        }

        warp::path!("api" / "v1" / "user" / "delete")
            .and(warp::delete())
            .and(with_auth(conn.clone()))
            .and(warp::body::json::<Params>())
            .and(with_state(conn))
            .and_then(
                |auth: User, Params { password }: Params, conn: DbConnection| async move {
                    super::delete(conn, auth, password)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn change_username(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        pub struct Params {
            new_username: String,
        }
        warp::path!("api" / "v1" / "user" / "username")
            .or(warp::path!("api" / "v1" / "auth" / "username"))
            .unify()
            .and(warp::patch())
            .and(with_auth(conn.clone()))
            .and(warp::body::json::<Params>())
            .and(with_state(conn))
            .and_then(|user, Params { new_username }: Params, conn| async move {
                super::change_username(conn, user, new_username)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn upload_avatar(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "user" / "avatar")
            .and(warp::post())
            .and(with_auth(conn.clone()))
            .and(warp::multipart::form().max_length(5_000_000))
            .and(with_state(conn))
            .and_then(|user, form, conn| async move {
                super::upload_avatar(conn, user, form)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}
