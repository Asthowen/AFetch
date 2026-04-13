#![allow(clippy::ref_option_ref)]

pub mod deserialize;

mod constants;

use std::collections::{HashMap, HashSet};

use bitcode::{Decode, Encode};
use serde::Deserialize;
use strum::IntoStaticStr;

use crate::config::deserialize::ColorWrapper;
use crate::error::FetchInfoError;
use crate::system::{InfoField, InfoKind};
use crate::translations::get_language;

#[derive(Debug, Decode, Encode)]
pub struct Config<'a> {
    pub info: HashMap<InfoKind, Vec<InfoField>>,
    pub language: &'a str,
    pub entries: Vec<Entry<'a>>,
    pub colors: ColorOption,
    pub logo: LogoStyle<'a>,
    pub parameters: InfoConfig<'a>,
}

impl Config<'_> {
    pub fn load(buffer: &mut Vec<u8>) -> Config<'_> {
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

        if let Ok(content) = std::fs::read(&cache_path) {
            *buffer = content;
            return bitcode::decode(buffer).ok().unwrap_or_default();
        }

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

        if let Ok(content) = std::fs::read(&config_path) {
            *buffer = content;

            let config: Config = serde_json::from_slice(buffer)
                .inspect_err(|error| {
                    eprintln!(
                        "Warning: Your configuration is malformed ({error}). \
                            Falling back to the default one.",
                    );
                })
                .ok()
                .unwrap_or_default();

            std::fs::create_dir_all(cache_path.parent().unwrap())
                .and_then(|()| std::fs::write(cache_path, bitcode::encode(&config)))
                .ok();

            return config;
        }

        Config::default()
    }
}

impl Default for Config<'_> {
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

impl Default for DisksInfoConfig<'_> {
    fn default() -> Self {
        Self {
            include: None,
            exclude: vec!["/boot", "/etc", "/snapd", "/docker"],
        }
    }
}

#[derive(Debug, Decode, Encode)]
pub struct NetworksInfoConfig<'a> {
    pub exclude: Vec<&'a str>,
    pub include: Option<Vec<&'a str>>,
    pub private_only: bool,
    pub assigned_only: bool,
    pub ignore_loopback: bool,
}

impl Default for NetworksInfoConfig<'_> {
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

#[derive(Debug, Decode, Encode)]
pub struct PublicIpInfoConfig<'a> {
    pub ipv4_domain: &'a str,
    pub ipv4_port: u16,
    pub ipv4_path: &'a str,
    pub ipv6_domain: &'a str,
    pub ipv6_port: u16,
    pub ipv6_path: &'a str,
}

impl Default for PublicIpInfoConfig<'_> {
    fn default() -> Self {
        Self {
            ipv4_domain: constants::DEFAULT_IPV4_DOMAIN,
            ipv4_port: constants::DEFAULT_IPV4_PORT,
            ipv4_path: constants::DEFAULT_IPV4_PATH,
            ipv6_domain: constants::DEFAULT_IPV6_DOMAIN,
            ipv6_port: constants::DEFAULT_IPV6_PORT,
            ipv6_path: constants::DEFAULT_IPV6_PATH,
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
            header_separator: constants::DEFAULT_COLOR_OPTION,
            info: constants::DEFAULT_COLOR_OPTION,
            separator: constants::DEFAULT_COLOR_OPTION,
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
                .fields()
                .iter()
                .filter(|field| {
                    let field_str: &'static str = (*field).into();
                    format.contains(field_str) || header.contains(field_str)
                })
                .copied()
                .collect(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Decode, Deserialize, Encode)]
#[serde(rename_all = "lowercase")]
pub enum SeparatorSizing {
    #[default]
    Dynamic,
    #[serde(untagged)]
    Fixed(usize),
}

#[derive(Debug, Decode, Deserialize, Encode)]
#[serde(rename_all = "lowercase", tag = "style")]
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

impl Default for LogoStyle<'_> {
    fn default() -> Self {
        Self::Braille { logo: None }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, IntoStaticStr)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
enum Locale {
    Fr,
    En,
    #[default]
    #[serde(other)]
    Auto,
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
