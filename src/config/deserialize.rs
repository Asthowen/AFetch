use bitcode::{Decode, Encode};
use owo_colors::DynColors;
use serde::Deserialize;

use super::{SeparatorSizing, constants};
use crate::config::constants::DEFAULT_COLOR;
use crate::logos::system_logo;
use crate::system::InfoKind;
use crate::translations::get_language;

#[derive(Debug, Deserialize)]
struct ConfigWrapper<'a> {
    #[serde(default)]
    language: super::Locale,
    info: Option<Vec<Entry<'a>>>,
    #[serde(default, borrow)]
    colors: Color<'a>,
    #[serde(default)]
    logo: super::LogoStyle<'a>,
    #[serde(default)]
    parameters: Option<InfoConfig<'a>>,
}

#[derive(Debug, Deserialize)]
struct InfoConfig<'a> {
    #[serde(default, borrow)]
    disks: Option<DisksInfoConfig<'a>>,
    #[serde(default, borrow)]
    networks: Option<NetworksInfoConfig<'a>>,
    #[serde(default, borrow)]
    public_ip: Option<PublicIpInfoConfig<'a>>,
}

#[derive(Debug, Deserialize)]
struct DisksInfoConfig<'a> {
    #[serde(default, borrow)]
    exclude: Option<Vec<&'a str>>,
    #[serde(default, borrow)]
    include: Option<Vec<&'a str>>,
}

#[derive(Debug, Deserialize)]
struct NetworksInfoConfig<'a> {
    #[serde(default, borrow)]
    exclude: Option<Vec<&'a str>>,
    #[serde(default, borrow)]
    include: Option<Vec<&'a str>>,
    #[serde(default)]
    private_only: Option<bool>,
    #[serde(default)]
    assigned_only: Option<bool>,
    #[serde(default)]
    ignore_loopback: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct PublicIpInfoConfig<'a> {
    #[serde(default, borrow)]
    ipv4_domain: Option<&'a str>,
    #[serde(default)]
    ipv4_port: Option<u16>,
    #[serde(default, borrow)]
    ipv4_path: Option<&'a str>,
    #[serde(default, borrow)]
    ipv6_domain: Option<&'a str>,
    #[serde(default)]
    ipv6_port: Option<u16>,
    #[serde(default, borrow)]
    ipv6_path: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ColorBlockStyle<'a> {
    Circle,
    Classic,
    Diamond,
    Triangle,
    Square,
    Star,
    Custom(&'a str),
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ColorBlockDisplay {
    Normal,
    Bright,
    #[default]
    Both,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Entry<'a> {
    Info {
        kind: InfoKind,
        header: Option<&'a str>,
        value: Option<&'a str>,
        separator: Option<&'a str>,
    },
    Separator {
        separator: String,
        sizing: Option<SeparatorSizing>,
    },
    ColorBlocks {
        color_block_style: ColorBlockStyle<'a>,
        display: Option<ColorBlockDisplay>,
    },
}

#[derive(Debug, Default, Deserialize)]
struct Color<'a> {
    #[serde(default, borrow)]
    header: Option<ColorRepr<'a>>,
    #[serde(default)]
    header_separator: Option<ColorRepr<'a>>,
    #[serde(default)]
    info: Option<ColorRepr<'a>>,
    #[serde(default)]
    separator: Option<ColorRepr<'a>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ColorRepr<'a> {
    Ansi(u8),
    Text(&'a str),
}

#[derive(Copy, Clone, Debug, Decode, Encode)]
pub enum ColorWrapper {
    Rgb { r: u8, g: u8, b: u8 },
    Ansi(u8),
}

impl Default for ColorWrapper {
    fn default() -> Self {
        DEFAULT_COLOR
    }
}

impl From<ColorWrapper> for DynColors {
    fn from(value: ColorWrapper) -> Self {
        match value {
            ColorWrapper::Ansi(color) => Self::Xterm(color.into()),
            ColorWrapper::Rgb { r, g, b } => Self::Rgb(r, g, b),
        }
    }
}

#[inline]
fn color_repr_to_wrapper(
    color: Option<ColorRepr>,
    default: Option<ColorWrapper>,
    logo_color: Option<ColorWrapper>,
) -> Option<ColorWrapper> {
    color.map_or(default, |c| match c {
        ColorRepr::Ansi(v) => Some(ColorWrapper::Ansi(v)),
        ColorRepr::Text("auto") => logo_color,
        ColorRepr::Text(c) => csscolorparser::parse(c)
            .map(|c| {
                let rgb = c.to_rgba8();
                Some(ColorWrapper::Rgb {
                    r: rgb[0],
                    g: rgb[1],
                    b: rgb[2],
                })
            })
            .unwrap_or(default),
    })
}

impl<'de> serde::Deserialize<'de> for super::Config<'de> {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let config = ConfigWrapper::deserialize(deserializer)?;
        let language_func = get_language(config.language.into());
        let logo_color = if let super::LogoStyle::Braille { logo: Some(logo) } = config.logo {
            Some(ColorWrapper::Ansi(system_logo(Some(logo.to_owned())).1))
        } else {
            None
        };

        let entries = config.info.map_or_else(
            || super::default_entries(config.language),
            |info| {
                info.into_iter()
                    .map(|info| match info {
                        Entry::Info {
                            kind,
                            header,
                            value: format,
                            separator,
                        } => {
                            let header =
                                header.unwrap_or_else(|| language_func(kind.default_header()));
                            let format = format.unwrap_or_else(|| kind.default_format());

                            super::Entry::Info {
                                kind,
                                fields: kind
                                    .fields()
                                    .iter()
                                    .filter(|field| {
                                        let field_str: &'static str = (*field).into();
                                        header.contains(field_str) || format.contains(field_str)
                                    })
                                    .copied()
                                    .collect::<Vec<_>>(),
                                header,
                                format,
                                separator: separator.unwrap_or_else(|| language_func("_colon_")),
                            }
                        }
                        Entry::Separator {
                            separator: content,
                            sizing: Some(sizing @ SeparatorSizing::Fixed(size)),
                        } => super::Entry::Separator {
                            content: content.chars().cycle().take(size).collect(),
                            sizing,
                        },
                        Entry::Separator { separator, sizing } => super::Entry::Separator {
                            content: separator,
                            sizing: sizing.unwrap_or_default(),
                        },
                        Entry::ColorBlocks {
                            color_block_style,
                            display,
                        } => super::Entry::ColorBlocks {
                            content: match color_block_style {
                                ColorBlockStyle::Circle => "● ",
                                ColorBlockStyle::Classic => "███",
                                ColorBlockStyle::Diamond => "◆ ",
                                ColorBlockStyle::Triangle => "▲ ",
                                ColorBlockStyle::Square => "■ ",
                                ColorBlockStyle::Star => "★ ",
                                ColorBlockStyle::Custom(content) => content,
                            },
                            display: match display {
                                Some(ColorBlockDisplay::Normal) => super::ColorBlockDisplay::Normal,
                                Some(ColorBlockDisplay::Bright) => super::ColorBlockDisplay::Bright,
                                _ => super::ColorBlockDisplay::Both,
                            },
                        },
                    })
                    .collect::<Vec<_>>()
            },
        );

        Ok(Self {
            info: super::group_fields_by_kind(&entries),
            logo: config.logo,
            language: config.language.into(),
            colors: super::ColorOption {
                header: color_repr_to_wrapper(config.colors.header, logo_color, logo_color),
                header_separator: color_repr_to_wrapper(
                    config.colors.header_separator,
                    constants::DEFAULT_COLOR_OPTION,
                    logo_color,
                ),
                info: color_repr_to_wrapper(
                    config.colors.info,
                    constants::DEFAULT_COLOR_OPTION,
                    logo_color,
                ),
                separator: color_repr_to_wrapper(
                    config.colors.separator,
                    constants::DEFAULT_COLOR_OPTION,
                    logo_color,
                ),
            },
            entries,
            parameters: config
                .parameters
                .map(|info| super::InfoConfig {
                    disks: info
                        .disks
                        .map(|disks| super::DisksInfoConfig {
                            exclude: disks.exclude.unwrap_or_default(),
                            include: disks.include,
                        })
                        .unwrap_or_default(),
                    networks: info
                        .networks
                        .map(|networks| super::NetworksInfoConfig {
                            exclude: networks.exclude.unwrap_or_default(),
                            include: networks.include,
                            private_only: networks.private_only.unwrap_or(true),
                            assigned_only: networks.assigned_only.unwrap_or(true),
                            ignore_loopback: networks.ignore_loopback.unwrap_or(true),
                        })
                        .unwrap_or_default(),
                    public_ip: info
                        .public_ip
                        .map(|public_ip| super::PublicIpInfoConfig {
                            ipv4_domain: public_ip
                                .ipv4_domain
                                .unwrap_or(constants::DEFAULT_IPV4_DOMAIN),
                            ipv4_port: public_ip.ipv4_port.unwrap_or(constants::DEFAULT_IPV4_PORT),
                            ipv4_path: public_ip.ipv4_path.unwrap_or(constants::DEFAULT_IPV4_PATH),
                            ipv6_domain: public_ip
                                .ipv6_domain
                                .unwrap_or(constants::DEFAULT_IPV6_DOMAIN),
                            ipv6_port: public_ip.ipv6_port.unwrap_or(constants::DEFAULT_IPV6_PORT),
                            ipv6_path: public_ip.ipv6_path.unwrap_or(constants::DEFAULT_IPV6_PATH),
                        })
                        .unwrap_or_default(),
                })
                .unwrap_or_default(),
        })
    }
}
