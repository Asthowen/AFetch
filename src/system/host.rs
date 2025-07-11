use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use crate::util::ToOptionString;
use whoami::fallible::hostname;
use whoami::username;

pub fn get_hostname(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (InfoField::Username, username()),
                (InfoField::Hostname, hostname().ok()),
            ]
        ),
    }))
}
