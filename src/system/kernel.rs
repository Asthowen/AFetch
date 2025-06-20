use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use sysinfo::System;

pub fn get_kernel(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (
                    InfoField::KernelVersion,
                    System::kernel_version().ok_or_else(FetchInfoError::missing)?
                ),
                (InfoField::KernelLongVersion, System::kernel_long_version()),
            ]
        ),
    }))
}
