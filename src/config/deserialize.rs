use crate::{
    config::SeparatorSizing, logos::get_logo, system::InfoKind, translations::get_language,
    util::colored::ColorWrapper,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ConfigWrapper<'a> {
    #[serde(default)]
    language: super::Locale,
    infos: Option<Vec<Entry<'a>>>,
    #[serde(default, borrow)]
    colors: Color<'a>,
    #[serde(default)]
    logo: super::LogoStyle<'a>,
}

#[derive(Deserialize, Debug)]
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
        sizing: Option<super::SeparatorSizing>,
    },
}

#[derive(Debug, Deserialize, Default)]
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

impl<'de: 'static> serde::Deserialize<'de> for super::Config {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let config = ConfigWrapper::deserialize(deserializer)?;
        let language_func = get_language(config.language.into());
        let logo_color = if let super::LogoStyle::Braille { logo: Some(logo) } = config.logo {
            Some(ColorWrapper::Ansi(get_logo(Some(logo.to_owned())).1))
        } else {
            None
        };

        Ok(Self {
            logo: config.logo,
            language: config.language.into(),
            colors: super::ColorOption {
                header: color_repr_to_wrapper(config.colors.header, logo_color, logo_color),
                header_separator: color_repr_to_wrapper(
                    config.colors.header_separator,
                    super::FALLBACK_COLOR,
                    logo_color,
                ),
                info: color_repr_to_wrapper(config.colors.info, super::FALLBACK_COLOR, logo_color),
                separator: color_repr_to_wrapper(
                    config.colors.separator,
                    super::FALLBACK_COLOR,
                    logo_color,
                ),
            },
            entries: config
                .infos
                .map(|infos| {
                    infos
                        .into_iter()
                        .map(|info| match info {
                            Entry::Info {
                                kind,
                                header,
                                value: format,
                                separator,
                            } => super::Entry::Info {
                                kind,
                                fields: kind
                                    .get_fields()
                                    .iter()
                                    .filter(|field| {
                                        format
                                            .unwrap_or_else(|| kind.default_format())
                                            .contains(&field.to_string())
                                    })
                                    .copied()
                                    .collect::<Vec<_>>(),
                                header: header
                                    .unwrap_or_else(|| language_func(kind.default_header())),
                                format: format.unwrap_or_else(|| kind.default_format()),
                                separator: separator.unwrap_or_else(|| language_func("_colon_")),
                            },
                            Entry::Separator {
                                separator: content,
                                sizing: Some(sizing @ SeparatorSizing::Fixed(size)),
                            } => super::Entry::Separator {
                                content: content.chars().cycle().take(size).collect(),
                                sizing,
                            },
                            Entry::Separator {
                                separator: content,
                                sizing,
                            } => super::Entry::Separator {
                                content: content.to_owned(),
                                sizing: sizing.unwrap_or_default(),
                            },
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_else(|| super::default_entries(config.language)),
        })
    }
}
