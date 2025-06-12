use crate::error::FetchInfosError;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use sysinfo::System;

pub fn get_loadavg(_languages_func: fn(&str) -> &str) -> Result<InfosResult, FetchInfosError> {
    let loadavg = System::load_average();
    Ok(InfosResult::Single(InfoGroup {
        values: vec![
            InfoValue {
                field: InfoField::LoadAvgOne,
                value: loadavg.one.to_string(),
            },
            InfoValue {
                field: InfoField::LoadAvgFive,
                value: loadavg.five.to_string(),
            },
            InfoValue {
                field: InfoField::LoadAvgFifteen,
                value: loadavg.fifteen.to_string(),
            },
        ],
    }))
}
