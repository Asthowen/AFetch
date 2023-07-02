use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct LogoConfig {
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default = "default_char_type")]
    pub char_type: String,
    #[serde(default = "default_picture_path")]
    pub picture_path: String,
}

fn default_status() -> String {
    "enable".to_owned()
}

fn default_char_type() -> String {
    "braille".to_owned()
}

fn default_picture_path() -> String {
    "none".to_owned()
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub logo: LogoConfig,
    #[serde(default = "default_text_color")]
    pub text_color: Vec<u8>,
    #[serde(default = "default_text_color_header")]
    pub text_color_header: Option<Vec<u8>>,
    #[serde(default = "default_disabled_entries")]
    pub disabled_entries: Vec<String>,
}

fn default_language() -> String {
    "auto".to_owned()
}

fn default_text_color() -> Vec<u8> {
    vec![255, 255, 255]
}

fn default_text_color_header() -> Option<Vec<u8>> {
    None
}

fn default_disabled_entries() -> Vec<String> {
    vec![
        "battery".to_owned(),
        "public-ip".to_owned(),
        "cpu-usage".to_owned(),
        "network".to_owned(),
        "wm".to_owned(),
    ]
}
