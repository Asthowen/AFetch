use crate::config::Config;
use crate::error::FetchInfosError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use sysinfo::System;

pub fn get_kernel(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfosResult, FetchInfosError> {
    Ok(InfosResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (
                    InfoField::KernelVersion,
                    System::kernel_version().ok_or_else(FetchInfosError::missing)?
                ),
                (InfoField::KernelLongVersion, System::kernel_long_version()),
            ]
        ),
    }))
}
