use crate::error::FetchInfosError;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use crate::util::convert_to_readable_unity;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub fn get_memory(_languages_func: fn(&str) -> &str) -> Result<InfosResult, FetchInfosError> {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );
    Ok(InfosResult::Single(InfoGroup {
        values: vec![
            InfoValue {
                field: InfoField::MemoryAvailable,
                value: convert_to_readable_unity(system.available_memory() as f64),
            },
            InfoValue {
                field: InfoField::MemoryFree,
                value: convert_to_readable_unity(system.free_memory() as f64),
            },
            InfoValue {
                field: InfoField::MemoryTotal,
                value: convert_to_readable_unity(system.total_memory() as f64),
            },
            InfoValue {
                field: InfoField::MemoryUsed,
                value: convert_to_readable_unity(system.used_memory() as f64),
            },
            InfoValue {
                field: InfoField::MemorySwapFree,
                value: convert_to_readable_unity(system.free_swap() as f64),
            },
            InfoValue {
                field: InfoField::MemorySwapTotal,
                value: convert_to_readable_unity(system.total_swap() as f64),
            },
            InfoValue {
                field: InfoField::MemorySwapUsage,
                value: convert_to_readable_unity(system.used_swap() as f64),
            },
        ],
    }))
}
