use crate::config::Config;
use crate::error::FetchInfosError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use whoami::fallible::hostname;
use whoami::username;

pub fn get_hostname(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfosResult, FetchInfosError> {
    Ok(InfosResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (InfoField::Username, username()),
                (InfoField::Hostname, hostname().unwrap_or_default()),
            ]
        ),
    }))
}
