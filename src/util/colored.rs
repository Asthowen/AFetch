use colored::{Color, ColoredString, Colorize, CustomColor};

#[derive(Debug, Copy, Clone)]
pub enum ColorWrapper {
    Ansi(u8),
    CustomColor(CustomColor),
}

pub trait ColorizeExt: Colorize {
    fn custom_color_wrapper(self, color: impl Into<ColorWrapper>) -> ColoredString
    where
        Self: Sized,
    {
        match color.into() {
            ColorWrapper::Ansi(color) => self.color(Color::AnsiColor(color)),
            ColorWrapper::CustomColor(color) => self.color(Color::TrueColor {
                r: color.r,
                g: color.g,
                b: color.b,
            }),
        }
    }
}

impl ColorizeExt for &str {}
