use crate::config::Config;
use crate::error::FetchInfoError;
use bitcode::{Decode, Encode};
use serde::Deserialize;

pub mod battery;
pub mod cpu;
pub mod disk;
pub mod disks;
pub mod host;
pub mod kernel;
pub mod loadavg;
pub mod memory;
pub mod uptime;

pub type InfoFunction =
    fn(fn(&str) -> &str, &[InfoField], &Config) -> Result<InfoResult, FetchInfoError>;

#[macro_export]
macro_rules! filtered_values {
    ($fields:expr, [ $( ($field:expr, $value_expr:expr) ),* $(,)? ]) => {{
        let mut info: Vec<InfoValue> = Vec::new();
        $(
            if $fields.contains(&$field) {
                info.push(InfoValue {
                    field: $field,
                    value: $value_expr,
                });
            }
        )*
        info
    }};
}

#[derive(Deserialize, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum InfoKind {
    Battery,
    Cpu,
    Disk,
    Disks,
    Host,
    Kernel,
    Loadavg,
    Memory,
    Uptime,
}

impl InfoKind {
    pub const fn get_fields(&self) -> &[InfoField] {
        match self {
            Self::Battery => &[
                InfoField::BatteryModel,
                InfoField::BatteryCycleCount,
                InfoField::BatterySerialNumber,
                InfoField::BatteryVendor,
                InfoField::BatteryTechnology,
                InfoField::BatteryState,
                InfoField::BatteryTemperature,
                InfoField::BatteryStateOfHealth,
                InfoField::BatteryStateOfCharge,
                InfoField::BatteryEnergy,
                InfoField::BatteryEnergyFull,
                InfoField::BatteryEnergyFullDesign,
                InfoField::BatteryEnergyRate,
                InfoField::BatteryVoltage,
                InfoField::BatteryTimeToFull,
                InfoField::BatteryTimeToEmpty,
            ],
            Self::Cpu => &[
                InfoField::CpuName,
                InfoField::CpuUsage,
                InfoField::CpuFrequency,
                InfoField::CpuVendor,
                InfoField::CpuArch,
            ],
            Self::Disk => &[
                InfoField::DiskName,
                InfoField::DiskAvailableSpace,
                InfoField::DiskUsedSpace,
                InfoField::DiskTotalSpace,
                InfoField::DiskMountPoint,
                InfoField::DiskIsRemovable,
                InfoField::DiskIsReadOnly,
                InfoField::DiskKind,
                InfoField::DiskWrittenSinceBoot,
                InfoField::DiskReadSinceBoot,
                InfoField::DiskFileSystem,
            ],
            Self::Disks => &[
                InfoField::DisksCount,
                InfoField::DisksAvailableSpace,
                InfoField::DisksUsedSpace,
                InfoField::DisksTotalSpace,
            ],
            Self::Host => &[InfoField::Hostname, InfoField::Username],
            Self::Kernel => &[InfoField::KernelVersion, InfoField::KernelLongVersion],
            Self::Loadavg => &[
                InfoField::LoadAvgOne,
                InfoField::LoadAvgFive,
                InfoField::LoadAvgFifteen,
            ],
            Self::Memory => &[
                InfoField::MemoryAvailable,
                InfoField::MemoryFree,
                InfoField::MemoryTotal,
                InfoField::MemoryUsed,
                InfoField::MemorySwapFree,
                InfoField::MemorySwapTotal,
                InfoField::MemorySwapUsage,
            ],
            Self::Uptime => &[InfoField::Uptime],
        }
    }

    pub const fn default_format(&self) -> &'static str {
        match self {
            Self::Battery => "{battery_state_of_charge}%",
            Self::Cpu => "{cpu_name}",
            Self::Disk => "{disk_used_space} / {disk_total_space}",
            Self::Disks => "{disks_used_space} / {disks_total_space}",
            Self::Host => "{username}@{hostname}",
            Self::Kernel => "{kernel_long_version}",
            Self::Memory => "{memory_used} / {memory_total}",
            Self::Uptime => "{uptime}",
            Self::Loadavg => "{loadavg_one}, {loadavg_five}, {loadavg_fifteen}",
        }
    }

    pub const fn default_header(&self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Battery => "battery",
            Self::Disk => "disk",
            Self::Disks => "disks",
            Self::Host => "host",
            Self::Kernel => "kernel",
            Self::Memory => "memory",
            Self::Uptime => "uptime",
            Self::Loadavg => "loadavg",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Decode, Encode)]
pub enum InfoField {
    BatteryModel,
    BatteryCycleCount,
    BatterySerialNumber,
    BatteryVendor,
    BatteryTechnology,
    BatteryState,
    BatteryTemperature,
    BatteryStateOfHealth,
    BatteryStateOfCharge,
    BatteryEnergy,
    BatteryEnergyFull,
    BatteryEnergyFullDesign,
    BatteryEnergyRate,
    BatteryVoltage,
    BatteryTimeToFull,
    BatteryTimeToEmpty,
    CpuName,
    CpuUsage,
    CpuFrequency,
    CpuVendor,
    CpuArch,
    DiskName,
    DiskAvailableSpace,
    DiskUsedSpace,
    DiskTotalSpace,
    DiskMountPoint,
    DiskFileSystem,
    DiskIsRemovable,
    DiskIsReadOnly,
    DiskKind,
    DiskWrittenSinceBoot,
    DiskReadSinceBoot,
    DisksCount,
    DisksAvailableSpace,
    DisksUsedSpace,
    DisksTotalSpace,
    Hostname,
    KernelVersion,
    KernelLongVersion,
    MemoryAvailable,
    MemoryFree,
    MemoryTotal,
    MemoryUsed,
    MemorySwapFree,
    MemorySwapTotal,
    MemorySwapUsage,
    Uptime,
    Username,
    LoadAvgOne,
    LoadAvgFive,
    LoadAvgFifteen,
}

impl InfoField {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BatteryModel => "battery_model",
            Self::BatteryCycleCount => "battery_cycle_count",
            Self::BatterySerialNumber => "battery_serial_number",
            Self::BatteryVendor => "battery_vendor",
            Self::BatteryTechnology => "battery_technology",
            Self::BatteryState => "battery_state",
            Self::BatteryTemperature => "battery_temperature",
            Self::BatteryStateOfHealth => "battery_state_of_health",
            Self::BatteryStateOfCharge => "battery_state_of_charge",
            Self::BatteryEnergy => "battery_energy",
            Self::BatteryEnergyFull => "battery_energy_full",
            Self::BatteryEnergyFullDesign => "battery_energy_full_design",
            Self::BatteryEnergyRate => "battery_energy_rate",
            Self::BatteryVoltage => "battery_voltage",
            Self::BatteryTimeToFull => "battery_time_to_full",
            Self::BatteryTimeToEmpty => "battery_time_to_empty",
            Self::CpuName => "cpu_name",
            Self::CpuUsage => "cpu_usage",
            Self::CpuFrequency => "cpu_frequency",
            Self::CpuVendor => "cpu_vendor",
            Self::CpuArch => "cpu_arch",
            Self::DiskName => "disk_name",
            Self::DiskAvailableSpace => "disk_available_space",
            Self::DiskUsedSpace => "disk_used_space",
            Self::DiskTotalSpace => "disk_total_space",
            Self::DiskMountPoint => "disk_mount_point",
            Self::DiskFileSystem => "disk_file_system",
            Self::DiskIsRemovable => "disk_is_removable",
            Self::DiskIsReadOnly => "disk_is_readonly",
            Self::DiskKind => "disk_kind",
            Self::DiskWrittenSinceBoot => "disk_written_since_boot",
            Self::DiskReadSinceBoot => "disk_read_since_boot",
            Self::DisksCount => "disks_count",
            Self::DisksAvailableSpace => "disks_available_space",
            Self::DisksUsedSpace => "disks_used_space",
            Self::DisksTotalSpace => "disks_total_space",
            Self::Hostname => "hostname",
            Self::KernelVersion => "kernel_version",
            Self::KernelLongVersion => "kernel_long_version",
            Self::MemoryAvailable => "memory_available",
            Self::MemoryFree => "memory_free",
            Self::MemoryTotal => "memory_total",
            Self::MemoryUsed => "memory_used",
            Self::MemorySwapFree => "memory_swap_free",
            Self::MemorySwapTotal => "memory_swap_total",
            Self::MemorySwapUsage => "memory_swap_usage",
            Self::Uptime => "uptime",
            Self::Username => "username",
            Self::LoadAvgOne => "loadavg_one",
            Self::LoadAvgFive => "loadavg_five",
            Self::LoadAvgFifteen => "loadavg_fifteen",
        }
    }
}

impl std::fmt::Display for InfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct InfoValue {
    pub field: InfoField,
    pub value: String,
}

pub struct InfoGroup {
    pub values: Vec<InfoValue>,
}

pub enum InfoResult {
    Single(InfoGroup),
    Several(Vec<InfoGroup>),
}
