use crate::error::FetchInfosError;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use whoami::fallible::hostname;
use whoami::username;

pub fn get_hostname(_languages_func: fn(&str) -> &str) -> Result<InfosResult, FetchInfosError> {
    Ok(InfosResult::Single(InfoGroup {
        values: vec![
            InfoValue {
                field: InfoField::Username,
                value: username(),
            },
            InfoValue {
                field: InfoField::Hostname,
                value: hostname().unwrap_or_default(),
            },
        ],
    }))
}
