use sysinfo::System;

use crate::config::Config;
use crate::error::FetchInfoError;
use crate::system::{InfoField, InfoGroup, InfoResult};
use crate::util::{ToOptionString, filtered_values, format_time};

pub fn uptime_info(
    languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [(
                InfoField::Uptime,
                format_time(System::uptime(), languages_func)
            ),]
        ),
    }))
}
