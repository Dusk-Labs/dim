//! This module contains all docs and APIs related to users and user metadata.
use crate::AppState;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::extract::Json;
use axum::extract::Multipart;
use axum::extract::multipart::Field;
use axum::extract::State;
use axum::Extension;

use dim_database::DatabaseError;
use dim_database::asset::Asset;
use dim_database::asset::InsertableAsset;
use dim_database::user::User;

use http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum AuthError {
    /// Username not available.
    UsernameNotAvailable,
    /// Upload failed.
    UploadFailed,
    /// Unsupported file.
    UnsupportedFile,
    /// Not logged in.
    InvalidCredentials,
    /// database: {0}
    Database(#[from] DatabaseError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::UsernameNotAvailable => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::UnsupportedFile => {
                (StatusCode::NOT_ACCEPTABLE, self.to_string()).into_response()
            }
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            Self::UploadFailed | Self::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}


#[derive(Deserialize)]
pub struct ChangePasswordParams {
    old_password: String,
    new_password: String,
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
/// [`InvalidCredentials`]: AuthError::InvalidCredentials
pub async fn change_password(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
    Json(params): Json<ChangePasswordParams>,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(DatabaseError::from)?;

    let user = User::authenticate(&mut tx, user.username, params.old_password)
        .await
        .map_err(|_| AuthError::InvalidCredentials)?;

    user.set_password(&mut tx, params.new_password).await?;

    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct DeleteParams {
    password: String,
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
/// [`InvalidCredentials`]: AuthError::InvalidCredentials
pub async fn delete(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
    Json(params): Json<DeleteParams>,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(DatabaseError::from)?;

    let user = User::authenticate(&mut tx, user.username, params.password)
        .await
        .map_err(|_| AuthError::InvalidCredentials)?;

    User::delete(&mut tx, user.id).await?;

    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct ChangeUsernameParams {
    new_username: String,
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
/// [`UsernameNotAvailable`]: AuthError::UsernameNotAvailable
pub async fn change_username(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
    Json(params): Json<ChangeUsernameParams>,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(DatabaseError::from)?;
    if User::get(&mut tx, &params.new_username).await.is_ok() {
        return Err(AuthError::UsernameNotAvailable);
    }

    User::set_username(&mut tx, user.username.clone(), params.new_username).await?;
    tx.commit().await.map_err(DatabaseError::from)?;

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
/// [`UploadFailed`]: AuthError::UploadFailed
/// [`UnsupportedFile`]: AuthError::UnsupportedFile
pub async fn upload_avatar(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(DatabaseError::from)?;

    let mut asset: Option<Asset> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap().to_string();
        if name == "file" {
            asset = Some(process_part(&mut tx, field).await?)
        }
    }

    match asset {
        Some(asset) => {
            User::set_picture(&mut tx, user.id, asset.id).await?;
            tx.commit().await.map_err(DatabaseError::from)?;

            Ok(StatusCode::OK)
        }
        None => Err(AuthError::UploadFailed)
    }
}

#[doc(hidden)]
pub async fn process_part(
    conn: &mut dim_database::Transaction<'_>,
    p: Field<'_>,
) -> Result<Asset, AuthError> {
    if p.name().unwrap() != "file" {
        return Err(AuthError::UploadFailed);
    }

    let file_ext = match p.content_type() {
        Some("image/jpeg" | "image/jpg") => "jpg",
        Some("image/png") => "png",
        _ => return Err(AuthError::UnsupportedFile),
    };

    let contents = p
        .bytes()
        .await
        .map_err(|_| AuthError::UploadFailed)?;

    let local_file = format!("{}.{}", Uuid::new_v4().to_string(), file_ext);
    let local_path = format!(
        "{}/{}",
        dim_core::core::METADATA_PATH.get().unwrap(),
        &local_file
    );

    tokio::fs::write(&local_path, contents)
        .await
        .map_err(|_| AuthError::UploadFailed)?;

    Ok(InsertableAsset {
        local_path: local_file,
        file_ext: file_ext.into(),
        ..Default::default()
    }
    .insert_local_asset(conn)
    .await?)
}
