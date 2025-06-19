use crate::config::Config;
use crate::error::FetchInfosError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use crate::util::format_time;
use sysinfo::System;

pub fn get_uptime(
    languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfosResult, FetchInfosError> {
    Ok(InfosResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [(
                InfoField::Uptime,
                format_time(System::uptime(), languages_func)
                    .ok_or_else(FetchInfosError::missing)?
            ),]
        ),
    }))
}
