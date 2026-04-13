mod english;
mod french;

fn get_language_func(country_code: &str) -> fn(&str) -> &'static str {
    match country_code {
        "fr" => french::french,
        _ => english::english,
    }
}

pub fn get_language(language: &str) -> fn(&str) -> &'static str {
    if language == "auto" {
        let locale_value_base: String = sys_locale::get_locale()
            .as_deref()
            .unwrap_or("en-US")
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
