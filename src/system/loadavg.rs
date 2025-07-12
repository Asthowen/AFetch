use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use crate::util::ToOptionString;
use sysinfo::System;

pub fn get_loadavg(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    let loadavg = System::load_average();
    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (InfoField::LoadAvgOne, loadavg.one.to_string()),
                (InfoField::LoadAvgFive, loadavg.five.to_string()),
                (InfoField::LoadAvgFifteen, loadavg.fifteen.to_string()),
            ]
        ),
    }))
}
