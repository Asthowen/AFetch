use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub logo: Logo,
    #[serde(default = "default_text_color")]
    pub text_color: Vec<u8>,
    #[serde(default = "default_text_color_header")]
    pub text_color_header: Option<Vec<u8>>,
    #[serde_as(as = "serde_with::EnumMap")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Logo {
    #[serde(default = "default_status")]
    pub status: bool,
    #[serde(default = "default_char_type")]
    pub char_type: String,
    #[serde(default = "default_picture_path")]
    pub picture_path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Entry {
    Cpu(Cpu),
    Memory,
    Battery,
    OS,
    Host,
    Kernel,
    Uptime,
    Packages,
    Shell(Shell),
    Resolution,
    #[serde(rename = "desktop-environment")]
    DesktopEnvironment(DesktopEnvironment),
    #[serde(rename = "window-manager")]
    WindowManager,
    Terminal,
    #[serde(rename = "terminal-font")]
    TerminalFont,
    GPUS,
    Network,
    Disks(Disks),
    Disk(Disk),
    #[serde(rename = "public-ip")]
    PublicIP,
    #[serde(rename = "color-blocks")]
    ColorBlocks,
    #[serde(rename = "empty-line")]
    EmptyLine,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Cpu {
    #[serde(default = "default_false")]
    pub percentage: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct DesktopEnvironment {
    #[serde(default = "default_true")]
    pub version: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Shell {
    #[serde(default = "default_true")]
    pub version: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Disks {
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Disk {
    pub hide: Option<Vec<String>>,
}

// Default value functions
fn default_language() -> String {
    "auto".to_owned()
}

const fn default_status() -> bool {
    true
}

fn default_char_type() -> String {
    "braille".to_owned()
}

fn default_picture_path() -> String {
    "none".to_owned()
}

fn default_text_color() -> Vec<u8> {
    vec![255, 255, 255]
}

const fn default_text_color_header() -> Option<Vec<u8>> {
    None
}

const fn default_false() -> bool {
    false
}

const fn default_true() -> bool {
    true
}
