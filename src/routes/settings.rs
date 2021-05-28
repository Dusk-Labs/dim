use crate::core::DbConnection;
use crate::errors;

use database::user::InsertableUser;
use database::user::UpdateableUser;
use database::user::User;
use database::user::UserSettings;

use auth::Wrapper as Auth;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use rocket::State;
use rocket_contrib::json::Json;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::lazy::SyncOnceCell;

#[derive(Serialize, Deserialize, Clone)]
pub struct GlobalSettings {
    pub enable_ssl: bool,
    pub port: u32,
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

#[get("/user/settings")]
pub async fn get_user_settings(
    db: State<'_, DbConnection>,
    user: Auth,
) -> Result<Json<UserSettings>, errors::DimError> {
    Ok(Json(
        User::get_one_unchecked(&*db, user.0.claims.get_user())
            .await?
            .settings,
    ))
}

#[post("/user/settings", format = "json", data = "<new_settings>")]
pub async fn post_user_settings(
    db: State<'_, DbConnection>,
    user: Auth,
    new_settings: Json<UserSettings>,
) -> Result<Json<UserSettings>, errors::DimError> {
    let update_user = UpdateableUser {
        settings: Some(new_settings.clone()),
        ..Default::default()
    };

    update_user.update(&*db, user.0.claims.get_user()).await?;

    Ok(new_settings)
}

#[get("/host/settings")]
pub async fn http_get_global_settings(
    user: Auth,
) -> Result<Json<&'static GlobalSettings>, errors::DimError> {
    Ok(Json(get_global_settings()))
}

#[post("/host/settings", format = "json", data = "<new_settings>")]
pub async fn http_set_global_settings(
    user: Auth,
    new_settings: Json<GlobalSettings>,
) -> Result<Json<&'static GlobalSettings>, errors::DimError> {
    if user.0.claims.has_role("owner") {
        let _ = set_global_settings(new_settings.into_inner());
        return Ok(Json(get_global_settings()));
    }

    Err(errors::DimError::Unauthorized)
}
