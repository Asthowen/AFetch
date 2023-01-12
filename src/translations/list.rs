use crate::translations::{english, french};
use std::collections::HashMap;

pub fn language_list() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    HashMap::from_iter(vec![("fr", french::french()), ("en", english::english())])
}

pub fn language_code_list() -> Vec<&'static str> {
    vec!["fr", "en"]
}
