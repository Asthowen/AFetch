use crate::translations::english::english;
use crate::translations::french::french;
use std::collections::HashMap;

pub mod english;
pub mod french;

pub fn get_language(country_code: &str) -> HashMap<&'static str, &'static str> {
    match country_code {
        "fr" => french(),
        _ => english(),
    }
}

pub fn language_code_list() -> [&'static str; 2] {
    ["fr", "en"]
}
