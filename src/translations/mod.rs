use crate::translations::english::english;
use crate::translations::french::french;

pub mod english;
pub mod french;

fn get_language_func(country_code: &str) -> fn(&str) -> &str {
    match country_code {
        "fr" => french,
        _ => english,
    }
}

const fn language_code_list() -> [&'static str; 2] {
    ["fr", "en"]
}

pub fn get_language(language: &str) -> fn(&str) -> &str {
    if language == "auto" {
        let locale_value_base: String = sys_locale::get_locale()
            .unwrap_or_else(|| "en-US".to_owned())
            .replace('_', "-");
        let locale_value: &str = locale_value_base
            .split('-')
            .next()
            .unwrap_or(&locale_value_base);
        if language_code_list().contains(&locale_value) {
            get_language_func(locale_value)
        } else {
            get_language_func("en")
        }
    } else if language_code_list().contains(&language) {
        get_language_func(language)
    } else {
        get_language_func("en")
    }
}
