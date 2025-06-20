use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
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
        let mut info_group = InfoGroup {
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
                ]
            ),
        };

        if fields.contains(&InfoField::BatteryModel) {
            if let Some(model) = battery.model() {
                info_group.values.push(InfoValue {
                    field: InfoField::BatteryModel,
                    value: model.to_owned(),
                });
            }
        }

        if fields.contains(&InfoField::BatteryCycleCount) {
            if let Some(cycle_count) = battery.cycle_count() {
                info_group.values.push(InfoValue {
                    field: InfoField::BatteryCycleCount,
                    value: cycle_count.to_string(),
                });
            }
        }

        if fields.contains(&InfoField::BatterySerialNumber) {
            if let Some(serial_number) = battery.serial_number() {
                info_group.values.push(InfoValue {
                    field: InfoField::BatterySerialNumber,
                    value: serial_number.trim().to_owned(),
                });
            }
        }

        if fields.contains(&InfoField::BatteryVendor) {
            if let Some(vendor) = battery.vendor() {
                info_group.values.push(InfoValue {
                    field: InfoField::BatteryVendor,
                    value: vendor.to_owned(),
                });
            }
        }

        if fields.contains(&InfoField::BatteryTemperature) {
            if let Some(temperature) = battery.temperature() {
                info_group.values.push(InfoValue {
                    field: InfoField::BatteryTemperature,
                    value: temperature.value.to_string(),
                });
            }
        }

        if fields.contains(&InfoField::BatteryTimeToFull) {
            if let Some(time_to_full) = battery
                .time_to_full()
                .and_then(|time| format_time(time.get::<second>().round() as u64, languages_func))
            {
                info_group.values.push(InfoValue {
                    field: InfoField::BatteryTimeToFull,
                    value: time_to_full,
                });
            }
        }

        if fields.contains(&InfoField::BatteryTimeToEmpty) {
            if let Some(time_to_empty) = battery
                .time_to_empty()
                .and_then(|time| format_time(time.get::<second>().round() as u64, languages_func))
            {
                info_group.values.push(InfoValue {
                    field: InfoField::BatteryTimeToEmpty,
                    value: time_to_empty,
                });
            }
        }

        batteries_info.push(info_group);
    }

    Ok(InfoResult::Several(batteries_info))
}
