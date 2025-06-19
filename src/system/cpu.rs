use crate::error::FetchInfosError;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use std::collections::HashSet;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub fn get_cpu(_languages_func: fn(&str) -> &str) -> Result<InfosResult, FetchInfosError> {
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
            values: vec![
                InfoValue {
                    field: InfoField::CpuName,
                    value: cpu.brand().to_owned(),
                },
                InfoValue {
                    field: InfoField::CpuUsage,
                    value: cpu.cpu_usage().to_string(),
                },
                InfoValue {
                    field: InfoField::CpuFrequency,
                    value: cpu.frequency().to_string(),
                },
                InfoValue {
                    field: InfoField::CpuVendor,
                    value: cpu.vendor_id().to_string(),
                },
                InfoValue {
                    field: InfoField::CpuArch,
                    value: System::cpu_arch(),
                },
            ],
        });
    }

    Ok(InfosResult::Several(cpu_info))
}
