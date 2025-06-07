use colored::{Color, ColoredString, Colorize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ColorWrapper {
    #[serde(rename = "rgb")]
    Rgb { r: u8, g: u8, b: u8 },
    #[serde(rename = "ansi")]
    Ansi(u8),
}

impl Default for ColorWrapper {
    fn default() -> Self {
        Self::Ansi(0)
    }
}

pub trait ColorizeExt: Colorize {
    fn custom_color_wrapper(self, color: impl Into<ColorWrapper>) -> ColoredString
    where
        Self: Sized,
    {
        match color.into() {
            ColorWrapper::Ansi(color) => self.color(Color::AnsiColor(color)),
            ColorWrapper::Rgb { r, g, b } => self.color(Color::TrueColor { r, g, b }),
        }
    }
}

impl ColorizeExt for &str {}
