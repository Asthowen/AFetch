use crate::config::Config;
use crate::error::FetchInfosError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoValue, InfosResult};
use crate::util::convert_to_readable_unity;
use sysinfo::{DiskRefreshKind, Disks};

pub fn get_disks(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    config: &Config,
) -> Result<InfosResult, FetchInfosError> {
    let mut available_space = 0;
    let mut total_space = 0;
    let mut count = 0;

    for disk in
        Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing().with_storage()).list()
    {
        let mount_point = disk.mount_point().to_str().unwrap_or_default();

        if config
            .parameters
            .disks
            .exclude
            .iter()
            .any(|ignore| mount_point.starts_with(ignore))
        {
            continue;
        }

        available_space += disk.available_space();
        total_space += disk.total_space();
        count += 1;
    }

    Ok(InfosResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (InfoField::DisksCount, count.to_string()),
                (
                    InfoField::DisksAvailableSpace,
                    convert_to_readable_unity(available_space as f64)
                ),
                (
                    InfoField::DisksUsedSpace,
                    convert_to_readable_unity((total_space - available_space) as f64)
                ),
                (
                    InfoField::DisksTotalSpace,
                    convert_to_readable_unity(total_space as f64)
                ),
            ]
        ),
    }))
}
