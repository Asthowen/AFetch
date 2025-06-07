use crate::error::FetchInfosError;
use crate::util::colored::ColorWrapper;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_language")]
    pub language: String,
    pub logo: Logo,
    pub colors: Colors,
    pub entries: Vec<Infos>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogoStyle {
    #[default]
    #[serde(rename = "braille")]
    Braille,
    #[serde(rename = "picture")]
    Picture,
    #[serde(rename = "file")]
    File,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Logo {
    #[serde(default = "default_status")]
    pub status: bool,
    pub style: LogoStyle,
    pub picture_path: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Colors {
    pub headers: Option<ColorWrapper>,
    pub separator: Option<ColorWrapper>,
    pub infos: Option<ColorWrapper>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infos {
    pub entry: String,
    pub header: Option<String>,
    pub separator: Option<String>,
    pub value: String,
}

fn default_language() -> String {
    "auto".to_owned()
}

const fn default_status() -> bool {
    true
}

pub fn load_config() -> Result<Config, FetchInfosError> {
    let parent_dir: PathBuf = dirs::config_dir()
        .ok_or_else(||
            FetchInfosError::error_exit(
                "An error occurred while retrieving the configuration files folder, please open an issue at: https://github.com/Asthowen/AFetch/issues/new so that we can solve your issue.".to_owned()
            )
        )?.join("afetch");
    let config_path = parent_dir.join("config.json");

    if !parent_dir.exists() {
        std::fs::create_dir_all(&parent_dir).map_err(|error| {
            FetchInfosError::error_exit(format!(
                "An error occurred while creating the configuration files: {error}",
            ))
        })?;
    }

    let config_content = if config_path.exists() {
        std::fs::read_to_string(&config_path).map_err(|error| {
            FetchInfosError::error(format!("Error when reading configuration file: {error}",))
        })?
    } else {
        let config = serde_json::to_string_pretty(&Config::default()).map_err(|error| {
            FetchInfosError::error(format!("Error when parsing default configuration: {error}",))
        })?;

        std::fs::write(&config_path, &config).map_err(|fallback_error| {
            FetchInfosError::error(format!(
                "Error when reading configuration fileFailed to create default configuration file: {fallback_error}",
            ))
        })?;

        config
    };

    match serde_json::from_str(&config_content) {
        Ok(config) => Ok(config),
        Err(parse_error) => {
            eprintln!(
                "Warning: Your configuration is malformed ({parse_error}). Falling back to the default configuration.",
            );
            Ok(Config::default())
        }
    }
}
