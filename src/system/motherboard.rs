use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use crate::util::ToOptionString;
use sysinfo::Motherboard;

pub fn get_motherboard(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    let motherboard = Motherboard::new().ok_or_else(FetchInfoError::missing)?;
    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (InfoField::MotherboardVendorName, motherboard.vendor_name()),
                (InfoField::MotherboardVersion, motherboard.version()),
                (
                    InfoField::MotherboardSerialNumber,
                    motherboard.serial_number()
                ),
                (InfoField::MotherboardName, motherboard.name()),
                (InfoField::MotherboardAssetTag, motherboard.asset_tag())
            ]
        ),
    }))
}
