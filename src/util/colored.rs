use colored::{Color, ColoredString, Colorize};

#[derive(Debug, Clone, Copy, bitcode::Decode, bitcode::Encode)]
pub enum ColorWrapper {
    Rgb { r: u8, g: u8, b: u8 },
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
