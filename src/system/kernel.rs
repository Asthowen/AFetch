use crate::error::FetchInfosError;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use sysinfo::System;

pub fn get_kernel(_languages_func: fn(&str) -> &str) -> Result<InfosResult, FetchInfosError> {
    Ok(InfosResult::Single(InfoGroup {
        values: vec![
            InfoValue {
                field: InfoField::KernelVersion,
                value: System::kernel_version().ok_or_else(FetchInfosError::missing)?,
            },
            InfoValue {
                field: InfoField::KernelLongVersion,
                value: System::kernel_long_version(),
            },
        ],
    }))
}
