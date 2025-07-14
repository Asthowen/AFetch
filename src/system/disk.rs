use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::disks::ignore_disk;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use crate::util::ToOptionString;
use crate::util::convert_to_readable_unity;
use sysinfo::Disks;

pub fn get_disk(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    let mut disks_info: Vec<InfoGroup> = Vec::new();

    for disk in Disks::new_with_refreshed_list().list() {
        let mount_point = disk.mount_point().to_string_lossy().to_string();

        if ignore_disk(config, &mount_point) {
            continue;
        }

        let disk_usage = disk.usage();
        disks_info.push(InfoGroup {
            values: filtered_values!(
                fields,
                [
                    (
                        InfoField::DiskName,
                        disk.name().to_os_string().into_string().ok()
                    ),
                    (
                        InfoField::DiskAvailableSpace,
                        convert_to_readable_unity(disk.available_space() as f64)
                    ),
                    (
                        InfoField::DiskUsedSpace,
                        convert_to_readable_unity(
                            (disk.total_space() - disk.available_space()) as f64
                        )
                    ),
                    (
                        InfoField::DiskTotalSpace,
                        convert_to_readable_unity(disk.total_space() as f64)
                    ),
                    (InfoField::DiskMountPoint, mount_point.to_owned()),
                    (
                        InfoField::DiskFileSystem,
                        disk.file_system().to_string_lossy().to_string()
                    ),
                    (InfoField::DiskIsReadOnly, disk.is_read_only().to_string()),
                    (InfoField::DiskIsRemovable, disk.is_removable().to_string()),
                    (InfoField::DiskKind, disk.kind().to_string()),
                    (
                        InfoField::DiskWrittenSinceBoot,
                        convert_to_readable_unity(disk_usage.total_written_bytes as f64)
                    ),
                    (
                        InfoField::DiskReadSinceBoot,
                        convert_to_readable_unity(disk_usage.total_read_bytes as f64)
                    ),
                ]
            ),
        });
    }

    Ok(InfoResult::Several(disks_info))
}
