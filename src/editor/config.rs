use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] toml::de::Error),
    #[error("Cannot determine HOME directory for configuration")]
    MissingHomeDirectory,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub editor: EditorConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct EditorConfig {
    pub theme: Option<String>,
}

impl Config {
    /// Loads the config file.
    pub fn load(path: Option<PathBuf>) -> Result<Self, Error> {
        let config = if let Some(path) = path {
            Self::load_from_file(&path)?
        } else {
            Self::load_from_config_dir()?
        };
        Ok(config)
    }

    /// Returns the path to the configuratoin file.
    pub fn get_config_path() -> Result<PathBuf, Error> {
        // TODO: Add Windows compatibility for config path (e.g., %APPDATA%)
        let config_dir = if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME") {
            PathBuf::from(config_home)
        } else {
            let home_dir = std::env::var_os("HOME").ok_or(Error::MissingHomeDirectory)?;
            PathBuf::from(home_dir).join(".config")
        };

        let app_name = env!("CARGO_PKG_NAME");
        let app_config_dir = config_dir.join(app_name);

        fs::create_dir_all(&app_config_dir)?;
        Ok(app_config_dir.join("config.toml"))
    }

    /// Loads the configuration from the default path.
    pub fn load_from_config_dir() -> Result<Self, Error> {
        let config_path = Self::get_config_path()?;
        let config_str = match fs::read_to_string(&config_path) {
            Ok(str) => str,
            Err(_) => return Ok(Self::default()),
        };
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }

    /// Loads a configuration from a given file.
    pub fn load_from_file(path: &PathBuf) -> Result<Self, Error> {
        let config_str = fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
