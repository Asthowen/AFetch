use whoami::{hostname, username};

use crate::config::Config;
use crate::error::FetchInfoError;
use crate::system::{InfoField, InfoGroup, InfoResult};
use crate::util::{ToOptionString, filtered_values};

pub fn hostname_info(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (InfoField::Username, username().ok()),
                (InfoField::Hostname, hostname().ok()),
            ]
        ),
    }))
}
