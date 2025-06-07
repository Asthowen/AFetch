use crate::error::FetchInfosError;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use crate::util::format_time;
use sysinfo::System;

pub fn get_uptime(languages_func: fn(&str) -> &str) -> Result<InfosResult, FetchInfosError> {
    Ok(InfosResult::Single(InfoGroup {
        values: vec![InfoValue {
            field: InfoField::Uptime,
            value: format_time(System::uptime(), languages_func)
                .ok_or_else(FetchInfosError::missing)?,
        }],
    }))
}
