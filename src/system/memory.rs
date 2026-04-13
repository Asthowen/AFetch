use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use crate::config::Config;
use crate::error::FetchInfoError;
use crate::system::{InfoField, InfoGroup, InfoResult};
use crate::util::{ToOptionString, convert_to_readable_unity, filtered_values};

pub fn memory_info(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );
    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (
                    InfoField::MemoryAvailable,
                    convert_to_readable_unity(system.available_memory() as f64)
                ),
                (
                    InfoField::MemoryFree,
                    convert_to_readable_unity(system.free_memory() as f64)
                ),
                (
                    InfoField::MemoryTotal,
                    convert_to_readable_unity(system.total_memory() as f64)
                ),
                (
                    InfoField::MemoryUsed,
                    convert_to_readable_unity(system.used_memory() as f64)
                ),
                (
                    InfoField::MemorySwapFree,
                    convert_to_readable_unity(system.free_swap() as f64)
                ),
                (
                    InfoField::MemorySwapTotal,
                    convert_to_readable_unity(system.total_swap() as f64)
                ),
                (
                    InfoField::MemorySwapUsage,
                    convert_to_readable_unity(system.used_swap() as f64)
                ),
            ]
        ),
    }))
}
