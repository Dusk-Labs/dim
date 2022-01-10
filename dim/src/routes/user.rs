use crate::core::DbConnection;
use crate::errors;
use auth::Wrapper as Auth;
use bytes::BufMut;

use database::asset::Asset;
use database::asset::InsertableAsset;
use database::progress::Progress;
use database::user::User;

use serde_json::json;

use warp::reply;

use http::StatusCode;

use futures::TryStreamExt;
use uuid::Uuid;

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

pub async fn change_password(
    conn: DbConnection,
    user: Auth,
    old_password: String,
    new_password: String,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = database::write_tx(&mut lock).await?;
    let user = User::get_one(&mut tx, user.0.claims.get_user(), old_password)
        .await
        .map_err(|_| errors::DimError::InvalidCredentials)?;
    user.set_password(&mut tx, new_password).await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete(
    conn: DbConnection,
    user: Auth,
    password: String,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = database::write_tx(&mut lock).await?;
    let _ = User::get_one(&mut tx, user.0.claims.get_user(), password)
        .await
        .map_err(|_| errors::DimError::InvalidCredentials)?;

    User::delete(&mut tx, user.0.claims.get_user()).await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn change_username(
    conn: DbConnection,
    user: Auth,
    new_username: String,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = database::write_tx(&mut lock).await?;
    if User::get(&mut tx, &new_username).await.is_ok() {
        return Err(errors::DimError::UsernameNotAvailable);
    }

    User::set_username(&mut tx, user.0.claims.get_user(), new_username).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn upload_avatar(
    conn: DbConnection,
    user: Auth,
    form: warp::multipart::FormData,
) -> Result<impl warp::Reply, errors::DimError> {
    let parts: Vec<warp::multipart::Part> = form
        .try_collect()
        .await
        .map_err(|_e| errors::DimError::UploadFailed)?;

    let mut lock = conn.writer().lock_owned().await;
    let mut tx = database::write_tx(&mut lock).await?;
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

#[doc(hidden)]
pub(crate) mod filters {
    use crate::core::DbConnection;
    use serde::Deserialize;

    use warp::reject;
    use warp::Filter;

    use super::super::global_filters::with_state;

    pub fn whoami(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "auth" / "whoami")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state(conn))
            .and_then(|auth: auth::Wrapper, conn: DbConnection| async move {
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
            .and(auth::with_auth())
            .and(warp::body::json::<Params>())
            .and(with_state(conn))
            .and_then(
                |user: auth::Wrapper,
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
            .and(auth::with_auth())
            .and(warp::body::json::<Params>())
            .and(with_state(conn))
            .and_then(
                |auth: auth::Wrapper, Params { password }: Params, conn: DbConnection| async move {
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
            .and(warp::patch())
            .and(auth::with_auth())
            .and(warp::body::json::<Params>())
            .and(with_state(conn))
            .and_then(|user: auth::Wrapper,
                Params {
                    new_username,
                }: Params,
                conn: DbConnection| async move {
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
            .and(auth::with_auth())
            .and(warp::multipart::form().max_length(5_000_000))
            .and(with_state(conn))
            .and_then(|user, form, conn| async move {
                super::upload_avatar(conn, user, form)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}
