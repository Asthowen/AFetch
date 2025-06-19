#![allow(clippy::ref_option_ref)]

pub mod deserialize;

use crate::{
    error::FetchInfosError,
    system::{InfoField, InfoKind},
    translations::get_language,
    util::colored::ColorWrapper,
};
use bitcode::{Decode, Encode};
use serde::Deserialize;

const FALLBACK_COLOR: Option<ColorWrapper> = Some(ColorWrapper::Rgb {
    r: 255,
    g: 255,
    b: 255,
});

#[derive(Debug, Decode, Encode)]
pub struct Config {
    pub language: &'static str,
    pub entries: Vec<Entry<'static>>,
    pub colors: ColorOption,
    pub logo: LogoStyle<'static>,
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
        display: u8,
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
        Self::Info {
            kind,
            header: header.unwrap_or_else(|| language_func(kind.default_header())),
            format,
            separator: separator.unwrap_or_else(|| language_func("_colon_")),
            fields: kind
                .get_fields()
                .iter()
                .filter(|field| format.contains(&field.to_string()))
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
        Self {
            language: Locale::default().into(),
            entries: default_entries(Locale::default()),
            colors: ColorOption::default(),
            logo: LogoStyle::default(),
        }
    }
}

pub fn load_config() -> Config {
    let cache_path = dirs::cache_dir()
        .map(|p| p.join("afetch.bin"))
        .ok_or_else(|| {
            FetchInfosError::error_exit(
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
                    FetchInfosError::error_exit(
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
            display: 2,
        },
    ]
}
