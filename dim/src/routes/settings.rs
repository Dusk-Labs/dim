use crate::core::DbConnection;
use crate::errors;

use database::user::InsertableUser;
use database::user::UpdateableUser;
use database::user::User;
use database::user::UserSettings;

use auth::Wrapper as Auth;
use serde::Deserialize;
use serde::Serialize;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::lazy::SyncOnceCell;

use warp::reply;
use warp::Filter;
use warp::Rejection;

#[derive(Serialize, Deserialize, Clone)]
pub struct GlobalSettings {
    pub enable_ssl: bool,
    pub port: u16,
    pub priv_key: Option<String>,
    pub ssl_cert: Option<String>,

    pub cache_dir: String,
    pub metadata_dir: String,
    pub quiet_boot: bool,

    pub disable_auth: bool,

    pub verbose: bool,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            enable_ssl: false,
            port: 8000,
            priv_key: None,
            ssl_cert: None,
            cache_dir: {
                cfg_if::cfg_if! {
                    if #[cfg(target_family = "unix")] {
                        "/tmp/streaming_cache".into()
                    } else {
                        "./streaming_cache".into()
                    }
                }
            },
            metadata_dir: "./metadata".into(),
            quiet_boot: false,
            disable_auth: false,
            verbose: false,
        }
    }
}

static GLOBAL_SETTINGS: SyncOnceCell<GlobalSettings> = SyncOnceCell::new();
static SETTINGS_PATH: SyncOnceCell<String> = SyncOnceCell::new();

pub fn get_global_settings() -> &'static GlobalSettings {
    if let Some(x) = GLOBAL_SETTINGS.get() {
        return x;
    }

    unreachable!("Global settings not initialized.");
}

pub fn init_global_settings(path: Option<String>) -> Result<(), Box<dyn Error>> {
    let path = path.unwrap_or("./config.json".into());
    let _ = SETTINGS_PATH.set(path.clone());
    let mut content = String::new();
    File::with_options()
        .write(true)
        .create(true)
        .read(true)
        .open(path)?
        .read_to_string(&mut content)?;

    let _ = GLOBAL_SETTINGS.set(serde_json::from_str(&content).unwrap_or_default());
    set_global_settings(get_global_settings().clone());

    Ok(())
}

pub fn set_global_settings(settings: GlobalSettings) -> Result<(), Box<dyn Error>> {
    let path = SETTINGS_PATH
        .get()
        .cloned()
        .unwrap_or("./config.json".into());
    let _ = GLOBAL_SETTINGS.set(settings);
    let settings = GLOBAL_SETTINGS.get().unwrap();
    serde_json::to_writer(File::create(path)?, settings)?;

    Ok(())
}

pub fn settings_router(
    conn: DbConnection,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    filters::get_user_settings(conn.clone())
        .or(filters::get_global_settings())
        .or(filters::post_user_settings(conn))
        .or(filters::set_global_settings())
}

mod filters {
    use database::user::UserSettings;
    use database::DbConnection;

    use auth::Wrapper as Auth;

    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use super::super::global_filters::with_state;

    pub fn get_user_settings(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "user" / "settings")
            .and(warp::get())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(|auth: Auth, conn: DbConnection| async move {
                super::get_user_settings(conn, auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn post_user_settings(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "user" / "settings")
            .and(warp::post())
            .and(warp::body::json::<UserSettings>())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |settings: UserSettings, auth: Auth, conn: DbConnection| async move {
                    super::post_user_settings(conn, auth, settings)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn get_global_settings(
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "host" / "settings")
            .and(warp::get())
            .and(auth::with_auth())
            .and_then(|auth: Auth| async move {
                super::http_get_global_settings(auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn set_global_settings(
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "host" / "settings")
            .and(warp::post())
            .and(warp::body::json::<super::GlobalSettings>())
            .and(auth::with_auth())
            .and_then(|settings: super::GlobalSettings, auth: Auth| async move {
                super::http_set_global_settings(auth, settings)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

pub async fn get_user_settings(
    db: DbConnection,
    user: Auth,
) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(
        &User::get(&db, &user.0.claims.get_user()).await?.prefs,
    ))
}

pub async fn post_user_settings(
    db: DbConnection,
    user: Auth,
    new_settings: UserSettings,
) -> Result<impl warp::Reply, errors::DimError> {
    let update_user = UpdateableUser {
        prefs: Some(new_settings.clone()),
    };

    update_user.update(&db, &user.0.claims.get_user()).await?;

    Ok(reply::json(&new_settings))
}

pub async fn http_get_global_settings(user: Auth) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(get_global_settings()))
}

pub async fn http_set_global_settings(
    user: Auth,
    new_settings: GlobalSettings,
) -> Result<impl warp::Reply, errors::DimError> {
    if user.0.claims.has_role("owner") {
        let _ = set_global_settings(new_settings);
        return Ok(reply::json(get_global_settings()));
    }

    Err(errors::DimError::Unauthorized)
}
