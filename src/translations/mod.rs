use crate::translations::english::english;
use crate::translations::french::french;

pub mod english;
pub mod french;

fn get_language_func(country_code: &str) -> fn(&str) -> &'static str {
    match country_code {
        "fr" => french,
        _ => english,
    }
}

pub fn get_language(language: &str) -> fn(&str) -> &'static str {
    if language == "auto" {
        let locale_value_base: String = sys_locale::get_locale()
            .unwrap_or_else(|| "en-US".to_owned())
            .replace('_', "-");
        let locale_value: &str = locale_value_base
            .split('-')
            .next()
            .unwrap_or(&locale_value_base);
        get_language_func(locale_value)
    } else {
        get_language_func(language)
    }
}
