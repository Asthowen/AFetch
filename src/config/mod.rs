#![allow(clippy::ref_option_ref)]

pub mod deserialize;

use crate::{
    config::deserialize::ColorWrapper,
    error::FetchInfoError,
    system::{InfoField, InfoKind},
    translations::get_language,
};
use bitcode::{Decode, Encode};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

const FALLBACK_COLOR: Option<ColorWrapper> = Some(ColorWrapper::Rgb {
    r: 255,
    g: 255,
    b: 255,
});
const DEFAULT_IPV4_DOMAIN: &str = "ipinfo.io";
const DEFAULT_IPV4_PORT: u16 = 80;
const DEFAULT_IPV4_PATH: &str = "/ip";
const DEFAULT_IPV6_DOMAIN: &str = "v6.ipinfo.io";
const DEFAULT_IPV6_PORT: u16 = 80;
const DEFAULT_IPV6_PATH: &str = "/ip";

#[derive(Debug, Decode, Encode)]
pub struct Config {
    pub info: HashMap<InfoKind, Vec<InfoField>>,
    pub language: &'static str,
    pub entries: Vec<Entry<'static>>,
    pub colors: ColorOption,
    pub logo: LogoStyle<'static>,
    pub parameters: InfoConfig<'static>,
}

#[derive(Debug, Default, Decode, Encode)]
pub struct InfoConfig<'a> {
    pub disks: DisksInfoConfig<'a>,
    pub networks: NetworksInfoConfig<'a>,
    pub public_ip: PublicIpInfoConfig<'a>,
}

#[derive(Debug, Decode, Encode)]
pub struct DisksInfoConfig<'a> {
    pub exclude: Vec<&'a str>,
    pub include: Option<Vec<&'a str>>,
}

#[derive(Debug, Decode, Encode)]
pub struct NetworksInfoConfig<'a> {
    pub exclude: Vec<&'a str>,
    pub include: Option<Vec<&'a str>>,
    pub private_only: bool,
    pub assigned_only: bool,
    pub ignore_loopback: bool,
}

#[derive(Debug, Decode, Encode)]
pub struct PublicIpInfoConfig<'a> {
    pub ipv4_domain: &'a str,
    pub ipv4_port: u16,
    pub ipv4_path: &'a str,
    pub ipv6_domain: &'a str,
    pub ipv6_port: u16,
    pub ipv6_path: &'a str,
}

impl<'a> Default for DisksInfoConfig<'a> {
    fn default() -> Self {
        Self {
            include: None,
            exclude: vec!["/boot", "/etc", "/snapd", "/docker"],
        }
    }
}

impl<'a> Default for NetworksInfoConfig<'a> {
    fn default() -> Self {
        Self {
            include: None,
            exclude: vec![
                "br-", "docker", "veth", "tun", "tap", "wg", "virbr", "vmnet",
            ],
            private_only: true,
            assigned_only: true,
            ignore_loopback: true,
        }
    }
}

impl<'a> Default for PublicIpInfoConfig<'a> {
    fn default() -> Self {
        Self {
            ipv4_domain: DEFAULT_IPV4_DOMAIN,
            ipv4_port: DEFAULT_IPV4_PORT,
            ipv4_path: DEFAULT_IPV4_PATH,
            ipv6_domain: DEFAULT_IPV6_DOMAIN,
            ipv6_port: DEFAULT_IPV6_PORT,
            ipv6_path: DEFAULT_IPV6_PATH,
        }
    }
}

#[derive(Debug, Decode, Encode)]
pub struct ColorOption {
    pub header: Option<ColorWrapper>,
    pub header_separator: Option<ColorWrapper>,
    pub info: Option<ColorWrapper>,
    pub separator: Option<ColorWrapper>,
}

impl Default for ColorOption {
    fn default() -> Self {
        Self {
            header: None,
            header_separator: FALLBACK_COLOR,
            info: FALLBACK_COLOR,
            separator: FALLBACK_COLOR,
        }
    }
}

#[derive(Debug, PartialEq, Decode, Encode)]
pub enum ColorBlockDisplay {
    Normal,
    Bright,
    Both,
}

impl ColorBlockDisplay {
    pub fn show_normal(&self) -> bool {
        self == &Self::Normal || self == &Self::Both
    }

    pub fn show_bright(&self) -> bool {
        self == &Self::Bright || self == &Self::Both
    }
}

#[derive(Debug, Decode, Encode)]
pub enum Entry<'a> {
    Info {
        kind: InfoKind,
        header: &'a str,
        format: &'a str,
        separator: &'a str,
        fields: Vec<InfoField>,
    },
    Separator {
        content: String,
        sizing: SeparatorSizing,
    },
    ColorBlocks {
        content: &'a str,
        display: ColorBlockDisplay,
    },
}

impl<'a> Entry<'a> {
    pub fn from_info(
        kind: InfoKind,
        language_func: fn(&str) -> &'a str,
        header: Option<&'a str>,
        separator: Option<&'a str>,
    ) -> Self {
        let format = kind.default_format();
        let header = header.unwrap_or_else(|| language_func(kind.default_header()));
        Self::Info {
            kind,
            header,
            format,
            separator: separator.unwrap_or_else(|| language_func("_colon_")),
            fields: kind
                .get_fields()
                .iter()
                .filter(|field| {
                    let field_str = field.as_str();
                    format.contains(field_str) || header.contains(field_str)
                })
                .copied()
                .collect(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Decode, Encode)]
#[serde(rename_all = "lowercase")]
pub enum SeparatorSizing {
    #[default]
    Dynamic,
    #[serde(untagged)]
    Fixed(usize),
}

#[derive(Debug, Deserialize, Decode, Encode)]
#[serde(tag = "style")]
#[serde(rename_all = "lowercase")]
pub enum LogoStyle<'a> {
    Disabled,
    Braille {
        logo: Option<&'a str>,
    },
    File {
        location: &'a str,
    },
    #[cfg(feature = "image")]
    Image {
        location: &'a str,
    },
}

impl<'a> Default for LogoStyle<'a> {
    fn default() -> Self {
        Self::Braille { logo: None }
    }
}

#[derive(Debug, Default, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Locale {
    Fr,
    En,
    #[default]
    #[serde(other)]
    Auto,
}

impl From<Locale> for &str {
    fn from(loc: Locale) -> Self {
        match loc {
            Locale::Auto => "auto",
            Locale::En => "en",
            Locale::Fr => "fr",
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let entries = default_entries(Locale::default());
        Self {
            info: group_fields_by_kind(&entries),
            language: Locale::default().into(),
            entries,
            colors: ColorOption::default(),
            logo: LogoStyle::default(),
            parameters: InfoConfig::default(),
        }
    }
}

pub fn load_config() -> Config {
    let cache_path = dirs::cache_dir()
        .map(|p| p.join("afetch.bin"))
        .ok_or_else(|| {
            FetchInfoError::error_exit(
                "An error occurred while retrieving the cache folder, \
                please open an issue at: https://github.com/Asthowen/AFetch/issues/new \
                so that we can solve your issue.",
            )
        })
        .unwrap();
    std::fs::read(&cache_path)
        .ok()
        .and_then(|buf| {
            let buf = Box::leak(buf.into_boxed_slice());
            bitcode::decode(buf).ok()
        })
        .or_else(|| {
            let config_path = dirs::config_dir()
                .map(|p| p.join("afetch").join("config.json"))
                .ok_or_else(|| {
                    FetchInfoError::error_exit(
                        "An error occurred while retrieving the config folder, \
                        please open an issue at: https://github.com/Asthowen/AFetch/issues/new \
                        so that we can solve your issue.",
                    )
                })
                .unwrap();

            std::fs::read(config_path)
                .ok()
                .and_then(|buf| {
                    let buf = Box::leak(buf.into_boxed_slice());
                    serde_json::from_slice(buf)
                        .inspect_err(|error| {
                            eprintln!(
                                "Warning: Your configuration is malformed ({error}). \
                            Falling back to the default one.",
                            );
                        })
                        .ok()
                })
                .inspect(|config| {
                    std::fs::create_dir_all(cache_path.parent().unwrap())
                        .and_then(|()| std::fs::write(cache_path, bitcode::encode(config)))
                        .ok();
                })
        })
        .unwrap_or_default()
}

fn default_entries(locale: Locale) -> Vec<Entry<'static>> {
    let language_func = get_language(locale.into());
    vec![
        Entry::from_info(InfoKind::Host, language_func, Some(""), Some("")),
        Entry::Separator {
            content: "─".to_owned(),
            sizing: SeparatorSizing::Dynamic,
        },
        Entry::from_info(InfoKind::Product, language_func, None, None),
        Entry::from_info(InfoKind::Cpu, language_func, None, None),
        Entry::from_info(InfoKind::Kernel, language_func, None, None),
        Entry::from_info(InfoKind::Uptime, language_func, None, None),
        Entry::from_info(InfoKind::Memory, language_func, None, None),
        #[cfg(not(target_os = "windows"))]
        Entry::from_info(InfoKind::Loadavg, language_func, None, None),
        Entry::Separator {
            content: String::default(),
            sizing: SeparatorSizing::Fixed(0),
        },
        Entry::ColorBlocks {
            content: "● ",
            display: ColorBlockDisplay::Both,
        },
    ]
}

fn group_fields_by_kind(entries: &[Entry]) -> HashMap<InfoKind, Vec<InfoField>> {
    let mut info = HashMap::new();

    for entry in entries {
        if let Entry::Info { kind, fields, .. } = entry {
            info.entry(kind).or_insert_with(HashSet::new).extend(fields);
        }
    }

    info.into_iter()
        .map(|(kind, fields)| (*kind, fields.into_iter().collect()))
        .collect()
}
