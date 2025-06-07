use crate::error::FetchInfosError;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use crate::util::format_time;
use starship_battery::units::time::second;

pub fn get_battery(languages_func: fn(&str) -> &str) -> Result<InfosResult, FetchInfosError> {
    let mut batteries = starship_battery::Manager::new()
        .map_err(|error| FetchInfosError::error(error.to_string()))?
        .batteries()
        .map_err(|error| FetchInfosError::error(error.to_string()))?;

    let mut batteries_infos: Vec<InfoGroup> = Vec::new();

    while let Some(Ok(battery)) = batteries.next() {
        let mut info_group = InfoGroup {
            values: vec![
                InfoValue {
                    field: InfoField::BatteryTechnology,
                    value: battery.technology().to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryState,
                    value: battery.state().to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryStateOfHealth,
                    value: (battery.state_of_health().value * 100.0).to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryStateOfCharge,
                    value: (battery.state_of_charge().value * 100.0).to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryEnergy,
                    value: battery.energy().value.to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryEnergyFull,
                    value: battery.energy_full().value.to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryEnergyFullDesign,
                    value: battery.energy_full_design().value.to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryEnergyRate,
                    value: battery.energy_rate().value.to_string(),
                },
                InfoValue {
                    field: InfoField::BatteryVoltage,
                    value: battery.voltage().value.to_string(),
                },
            ],
        };

        if let Some(model) = battery.model() {
            info_group.values.push(InfoValue {
                field: InfoField::BatteryModel,
                value: model.to_owned(),
            });
        }

        if let Some(cycle_count) = battery.cycle_count() {
            info_group.values.push(InfoValue {
                field: InfoField::BatteryCycleCount,
                value: cycle_count.to_string(),
            });
        }

        if let Some(serial_number) = battery.serial_number() {
            info_group.values.push(InfoValue {
                field: InfoField::BatterySerialNumber,
                value: serial_number.trim().to_owned(),
            });
        }

        if let Some(vendor) = battery.vendor() {
            info_group.values.push(InfoValue {
                field: InfoField::BatteryVendor,
                value: vendor.to_owned(),
            });
        }

        if let Some(temperature) = battery.temperature() {
            info_group.values.push(InfoValue {
                field: InfoField::BatteryTemperature,
                value: temperature.value.to_string(),
            });
        }

        if let Some(time_to_full) = battery
            .time_to_full()
            .and_then(|time| format_time(time.get::<second>().round() as u64, languages_func))
        {
            info_group.values.push(InfoValue {
                field: InfoField::BatteryTimeToFull,
                value: time_to_full,
            });
        }

        if let Some(time_to_empty) = battery
            .time_to_empty()
            .and_then(|time| format_time(time.get::<second>().round() as u64, languages_func))
        {
            info_group.values.push(InfoValue {
                field: InfoField::BatteryTimeToEmpty,
                value: time_to_empty,
            });
        }

        batteries_infos.push(info_group);
    }

    Ok(InfosResult::Several(batteries_infos))
}
