use crate::translations::english::english;
use crate::translations::french::french;
use std::collections::HashMap;

pub fn language_list() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    HashMap::from_iter(vec![("fr", french()), ("en", english())])
}

pub fn language_code_list() -> Vec<&'static str> {
    vec!["fr", "en"]
}
