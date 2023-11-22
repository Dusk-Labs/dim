use crate::utils::ffpath;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

use serde::Deserialize;
use serde::Serialize;


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
    pub version: String,
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
            version: String::new(),
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