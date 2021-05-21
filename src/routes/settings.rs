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

#[derive(Serialize, Deserialize)]
pub struct GlobalSettings {
    enable_ssl: bool,
    port: u32,
    priv_key: Option<String>,
    ssl_cert: Option<String>,

    cache_dir: String,
    metadata_dir: String,
    quiet_boot: bool,

    disable_auth: bool,
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
        }
    }
}

static GLOBAL_SETTINGS: SyncOnceCell<GlobalSettings> = SyncOnceCell::new();
static SETTINGS_PATH: SyncOnceCell<String> = SyncOnceCell::new();

pub fn get_global_settings() -> Option<&'static GlobalSettings> {
    GLOBAL_SETTINGS.get()
}

pub fn init_global_settings(path: Option<String>) -> Result<(), Box<dyn Error>> {
    let path = path.unwrap_or("./config.json".into());
    let _ = SETTINGS_PATH.set(path.clone());
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;

    let _ = GLOBAL_SETTINGS.set(serde_json::from_str(&content)?);

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

#[get("/settings")]
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

#[post("/settings", format = "json", data = "<new_settings>")]
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

#[get("/settings")]
pub async fn http_get_global_settings(
    user: Auth,
) -> Result<Json<Option<&'static GlobalSettings>>, errors::DimError> {
    Ok(Json(get_global_settings()))
}

#[post("/settings", format = "json", data = "<new_settings>")]
pub async fn http_set_global_settings(
    user: Auth,
    new_settings: Json<GlobalSettings>,
) -> Result<Json<Option<&'static GlobalSettings>>, errors::DimError> {
    if user.0.claims.has_role("owner") {
        let _ = set_global_settings(new_settings.into_inner());
        return Ok(Json(get_global_settings()));
    }

    Err(errors::DimError::Unauthorized)
}
