use crate::core::DbConnection;
use crate::errors;
use crate::utils::ffpath;

use dim_database::user::UpdateableUser;
use dim_database::user::User;
use dim_database::user::UserSettings;

use serde::Deserialize;
use serde::Serialize;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

use warp::reply;

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
    pub secret_key: Option<[u8; 32]>,
    pub enable_hwaccel: bool,
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
            metadata_dir: ffpath("config/metadata"),
            quiet_boot: false,
            disable_auth: false,
            verbose: false,
            secret_key: None,
            enable_hwaccel: true,
        }
    }
}

static GLOBAL_SETTINGS: Lazy<Mutex<GlobalSettings>> = Lazy::new(|| Default::default());
static SETTINGS_PATH: OnceCell<String> = OnceCell::new();

pub fn get_global_settings() -> GlobalSettings {
    let lock = GLOBAL_SETTINGS.lock().unwrap();
    lock.clone()
}

pub fn init_global_settings(path: Option<String>) -> Result<(), Box<dyn Error>> {
    let path = path.unwrap_or(ffpath("config/config.toml"));
    let _ = SETTINGS_PATH.set(path.clone());
    let mut content = String::new();

    OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(path)?
        .read_to_string(&mut content)?;

    {
        let mut lock = GLOBAL_SETTINGS.lock().unwrap();
        *lock = toml::from_str(&content).unwrap_or_default();
    }

    let _ = set_global_settings(get_global_settings());

    Ok(())
}

pub fn set_global_settings(settings: GlobalSettings) -> Result<(), Box<dyn Error>> {
    let path = SETTINGS_PATH
        .get()
        .cloned()
        .unwrap_or(ffpath("config/config.toml"));

    {
        let mut lock = GLOBAL_SETTINGS.lock().unwrap();
        *lock = settings;
    }

    let settings = get_global_settings();
    File::create(path)?
        .write(toml::to_string_pretty(&settings).unwrap().as_ref())
        .unwrap();

    Ok(())
}

pub mod filters {
    use dim_database::user::User;
    use dim_database::user::UserSettings;
    use dim_database::DbConnection;

    use warp::reject;
    use warp::Filter;
    use warp::Rejection;

    use super::super::global_filters::with_auth;
    use super::super::global_filters::with_state;

    pub fn get_user_settings(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "user" / "settings")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|auth: User, conn: DbConnection| async move {
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
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(
                |settings: UserSettings, auth: User, conn: DbConnection| async move {
                    println!("saving user settings");
                    super::post_user_settings(conn, auth, settings)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn get_global_settings(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "host" / "settings")
            .and(warp::get())
            .and(with_auth(conn))
            .and_then(|auth: User| async move {
                super::http_get_global_settings(auth)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn set_global_settings(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("api" / "v1" / "host" / "settings")
            .and(warp::post())
            .and(warp::body::json::<super::GlobalSettings>())
            .and(with_auth(conn))
            .and_then(|settings: super::GlobalSettings, auth: User| async move {
                super::http_set_global_settings(auth, settings)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

pub async fn get_user_settings(
    db: DbConnection,
    user: User,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = db.read().begin().await?;
    Ok(reply::json(&User::get_by_id(&mut tx, user.id).await?.prefs))
}

pub async fn post_user_settings(
    db: DbConnection,
    user: User,
    new_settings: UserSettings,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut lock = db.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await?;
    let update_user = UpdateableUser {
        prefs: Some(new_settings.clone()),
    };

    update_user.update(&mut tx, user.id).await?;

    tx.commit().await?;
    drop(lock);

    Ok(reply::json(&new_settings))
}

// TODO: Hide secret key.
pub async fn http_get_global_settings(_user: User) -> Result<impl warp::Reply, errors::DimError> {
    Ok(reply::json(&get_global_settings()))
}

// TODO: Disallow setting secret key over http.
pub async fn http_set_global_settings(
    user: User,
    new_settings: GlobalSettings,
) -> Result<impl warp::Reply, errors::DimError> {
    if user.has_role("owner") {
        set_global_settings(new_settings).unwrap();
        return Ok(reply::json(&get_global_settings()));
    }

    Err(errors::DimError::Unauthorized)
}
