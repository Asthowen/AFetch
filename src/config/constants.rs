use super::deserialize::ColorWrapper;

pub const DEFAULT_COLOR: ColorWrapper = ColorWrapper::Rgb {
    r: 255,
    g: 255,
    b: 255,
};
pub const DEFAULT_COLOR_OPTION: Option<ColorWrapper> = Some(DEFAULT_COLOR);

pub const DEFAULT_IPV4_DOMAIN: &str = "ipinfo.io";
pub const DEFAULT_IPV4_PORT: u16 = 80;
pub const DEFAULT_IPV4_PATH: &str = "/ip";
pub const DEFAULT_IPV6_DOMAIN: &str = "v6.ipinfo.io";
pub const DEFAULT_IPV6_PORT: u16 = 80;
pub const DEFAULT_IPV6_PATH: &str = "/ip";
