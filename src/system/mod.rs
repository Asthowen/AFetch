mod battery;
mod cpu;
mod disk;
mod disks;
mod host;
mod kernel;
mod loadavg;
mod memory;
mod motherboard;
mod networks;
mod product;
mod public_ip;
mod uptime;

pub use self::battery::battery_info;
pub use self::cpu::cpu_info;
pub use self::disk::disk_info;
pub use self::disks::disks_info;
pub use self::host::hostname_info;
pub use self::kernel::kernel_info;
pub use self::loadavg::loadavg_info;
pub use self::memory::memory_info;
pub use self::motherboard::motherboard_info;
pub use self::networks::networks_info;
pub use self::product::product_info;
pub use self::public_ip::public_ip_info;
pub use self::uptime::uptime_info;

use bitcode::{Decode, Encode};
use serde::Deserialize;
use strum::IntoStaticStr;

use crate::config::Config;
use crate::error::FetchInfoError;

pub type InfoFunction =
    fn(fn(&str) -> &str, &[InfoField], &Config) -> Result<InfoResult, FetchInfoError>;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Decode, Deserialize, Encode)]
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
    Motherboard,
    Networks,
    Product,
    PublicIp,
    Uptime,
}

impl InfoKind {
    pub const fn fields(&self) -> &[InfoField] {
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
                InfoField::DisksCountFiltered,
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
            Self::Motherboard => &[
                InfoField::MotherboardAssetTag,
                InfoField::MotherboardName,
                InfoField::MotherboardSerialNumber,
                InfoField::MotherboardVendorName,
                InfoField::MotherboardVersion,
            ],
            Self::Networks => &[
                InfoField::NetworkName,
                InfoField::NetworkFirstIp,
                InfoField::NetworkPreferFirstIpv4,
                InfoField::NetworkPreferFirstIpv6,
                InfoField::NetworkAllIp,
                InfoField::NetworkMacAddress,
                InfoField::NetworkMaximumTransferUnit,
                InfoField::NetworkErrorsOnReceived,
                InfoField::NetworkErrorsOnTransmitted,
                InfoField::NetworkPacketsReceived,
                InfoField::NetworkPacketsTransmitted,
                InfoField::NetworkReceived,
                InfoField::NetworkTransmitted,
            ],
            Self::Product => &[
                InfoField::ProductFamily,
                InfoField::ProductName,
                InfoField::ProductSerialNumber,
                InfoField::ProductStockKeepingUnit,
                InfoField::ProductUuid,
                InfoField::ProductVendorName,
                InfoField::ProductVersion,
            ],
            Self::PublicIp => &[
                InfoField::PublicIpAny,
                InfoField::PublicIpv4,
                InfoField::PublicIpv6,
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
            Self::Loadavg => "{load_avg_one}, {load_avg_five}, {load_avg_fifteen}",
            Self::Memory => "{memory_used} / {memory_total}",
            Self::Motherboard => "{motherboard_name} {motherboard_version}",
            Self::Networks => "{network_prefer_first_ipv4}",
            Self::Product => "{product_name} {product_version}",
            Self::PublicIp => "{public_ip_any}",
            Self::Uptime => "{uptime}",
        }
    }

    pub const fn default_header(&self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Battery => "battery",
            Self::Disk => "disk",
            Self::Disks => "disks",
            Self::Host | Self::Product => "host",
            Self::Kernel => "kernel",
            Self::Loadavg => "loadavg",
            Self::Memory => "memory",
            Self::Motherboard => "motherboard",
            Self::Networks => "networks",
            Self::PublicIp => "public-ip",
            Self::Uptime => "uptime",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Decode, Encode, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
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
    DisksCountFiltered,
    DisksAvailableSpace,
    DisksUsedSpace,
    DisksTotalSpace,
    Hostname,
    KernelVersion,
    KernelLongVersion,
    LoadAvgOne,
    LoadAvgFive,
    LoadAvgFifteen,
    MemoryAvailable,
    MemoryFree,
    MemoryTotal,
    MemoryUsed,
    MemorySwapFree,
    MemorySwapTotal,
    MemorySwapUsage,
    MotherboardAssetTag,
    MotherboardName,
    MotherboardSerialNumber,
    MotherboardVendorName,
    MotherboardVersion,
    NetworkName,
    NetworkFirstIp,
    NetworkPreferFirstIpv4,
    NetworkPreferFirstIpv6,
    NetworkAllIp,
    NetworkMacAddress,
    NetworkMaximumTransferUnit,
    NetworkErrorsOnReceived,
    NetworkErrorsOnTransmitted,
    NetworkPacketsReceived,
    NetworkPacketsTransmitted,
    NetworkReceived,
    NetworkTransmitted,
    ProductFamily,
    ProductName,
    ProductSerialNumber,
    ProductStockKeepingUnit,
    ProductUuid,
    ProductVendorName,
    ProductVersion,
    PublicIpAny,
    PublicIpv4,
    PublicIpv6,
    Uptime,
    Username,
}

impl std::fmt::Display for InfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let field: &'static str = self.into();
        write!(f, "{field}")
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
