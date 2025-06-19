use crate::config::Config;
use crate::error::FetchInfosError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use std::collections::HashSet;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub fn get_cpu(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    _config: &Config,
) -> Result<InfosResult, FetchInfosError> {
    let system =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

    let mut cpu_info: Vec<InfoGroup> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();

    for cpu in system.cpus() {
        if seen.contains(cpu.brand()) {
            continue;
        }
        seen.insert(cpu.brand());
        cpu_info.push(InfoGroup {
            values: filtered_values!(
                fields,
                [
                    (InfoField::CpuName, cpu.brand().to_owned()),
                    (InfoField::CpuUsage, cpu.cpu_usage().to_string()),
                    (InfoField::CpuFrequency, cpu.frequency().to_string()),
                    (InfoField::CpuVendor, cpu.vendor_id().to_string()),
                    (InfoField::CpuArch, System::cpu_arch()),
                ]
            ),
        });
    }

    Ok(InfosResult::Several(cpu_info))
}
