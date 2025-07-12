use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use crate::util::ToOptionString;
use crate::util::format_time;
use starship_battery::units::time::second;

pub fn get_battery(
    languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    let mut batteries = starship_battery::Manager::new()
        .map_err(|error| FetchInfoError::error(error.to_string()))?
        .batteries()
        .map_err(|error| FetchInfoError::error(error.to_string()))?;

    let mut batteries_info: Vec<InfoGroup> = Vec::new();

    while let Some(Ok(battery)) = batteries.next() {
        batteries_info.push(InfoGroup {
            values: filtered_values!(
                fields,
                [
                    (
                        InfoField::BatteryTechnology,
                        battery.technology().to_string()
                    ),
                    (InfoField::BatteryState, battery.state().to_string()),
                    (
                        InfoField::BatteryStateOfHealth,
                        (battery.state_of_health().value * 100.0).to_string()
                    ),
                    (
                        InfoField::BatteryStateOfCharge,
                        (battery.state_of_charge().value * 100.0).to_string()
                    ),
                    (InfoField::BatteryEnergy, battery.energy().value.to_string()),
                    (
                        InfoField::BatteryEnergyFull,
                        battery.energy_full().value.to_string()
                    ),
                    (
                        InfoField::BatteryEnergyFullDesign,
                        battery.energy_full().value.to_string()
                    ),
                    (
                        InfoField::BatteryEnergyRate,
                        battery.energy_rate().value.to_string()
                    ),
                    (
                        InfoField::BatteryVoltage,
                        battery.voltage().value.to_string()
                    ),
                    (InfoField::BatteryModel, battery.model()),
                    (
                        InfoField::BatteryCycleCount,
                        battery.cycle_count().map(|value| value.to_string())
                    ),
                    (
                        InfoField::BatterySerialNumber,
                        battery.serial_number().map(|value| value.trim().to_owned())
                    ),
                    (InfoField::BatteryVendor, battery.vendor()),
                    (
                        InfoField::BatteryTemperature,
                        battery
                            .temperature()
                            .map(|temperature| temperature.value.to_string())
                    ),
                    (
                        InfoField::BatteryTimeToFull,
                        battery.time_to_full().and_then(|time| format_time(
                            time.get::<second>().round() as u64,
                            languages_func
                        ))
                    ),
                    (
                        InfoField::BatteryTimeToEmpty,
                        battery.time_to_empty().and_then(|time| format_time(
                            time.get::<second>().round() as u64,
                            languages_func
                        ))
                    ),
                ]
            ),
        });
    }

    Ok(InfoResult::Several(batteries_info))
}
