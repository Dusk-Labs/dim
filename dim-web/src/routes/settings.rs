use axum::response::{Json, IntoResponse};
use axum::{extract, Extension};

use dim_core::routes::settings::{GlobalSettings, get_global_settings, set_global_settings};
use dim_database::DatabaseError;
use dim_database::DbConnection;
use dim_database::user::UpdateableUser;
use dim_database::user::User;
use dim_database::user::UserSettings;

use super::auth::AuthError;


pub async fn get_user_settings(
    Extension(user): Extension<User>,
    extract::State(conn): extract::State<DbConnection>,
) -> Result<axum::response::Response, AuthError> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
    Ok(Json(&User::get_by_id(&mut tx, user.id).await?.prefs).into_response())
}

pub async fn post_user_settings(
    Extension(user): Extension<User>,
    extract::State(conn): extract::State<DbConnection>,
    extract::Json(new_settings): extract::Json<UserSettings>,
) -> Result<axum::response::Response, AuthError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(DatabaseError::from)?;
    let update_user = UpdateableUser {
        prefs: Some(new_settings.clone()),
    };

    update_user.update(&mut tx, user.id).await?;

    tx.commit().await.map_err(DatabaseError::from)?;
    drop(lock);

    Ok(Json(&new_settings).into_response())
}

// TODO: Hide secret key.
pub async fn http_get_global_settings() -> Result<axum::response::Response, AuthError> {
    Ok(Json(&get_global_settings()).into_response())
}

// TODO: Disallow setting secret key over http.
pub async fn http_set_global_settings(
    Extension(user): Extension<User>,
    extract::Json(new_settings): extract::Json<GlobalSettings>,
) -> Result<axum::response::Response, AuthError> {
    if user.has_role("owner") {
        set_global_settings(new_settings).unwrap();
        return Ok(Json(&get_global_settings()).into_response());
    }

    Err(AuthError::InvalidCredentials)
}
