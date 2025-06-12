use crate::error::FetchInfosError;
use bitcode::{Decode, Encode};
use serde::Deserialize;

pub mod battery;
pub mod cpu;
pub mod host;
pub mod kernel;
pub mod loadavg;
pub mod memory;
pub mod uptime;

pub type InfoFunction = fn(fn(&str) -> &str) -> Result<InfosResult, FetchInfosError>;

#[derive(Deserialize, Clone, Copy, Debug, Decode, Encode)]
#[serde(rename_all = "snake_case")]
pub enum InfoKind {
    Battery,
    Cpu,
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
            Self::Host => &[InfoField::Hostname],
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
            Self::Host => "host",
            Self::Kernel => "kernel",
            Self::Memory => "memory",
            Self::Uptime => "uptime",
            Self::Loadavg => "loadavg",
        }
    }
}

#[derive(Debug, Clone, Copy, Decode, Encode)]
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

impl std::fmt::Display for InfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BatteryModel => write!(f, "battery_model"),
            Self::BatteryCycleCount => write!(f, "battery_cycle_count"),
            Self::BatterySerialNumber => write!(f, "battery_serial_number"),
            Self::BatteryVendor => write!(f, "battery_vendor"),
            Self::BatteryTechnology => write!(f, "battery_technology"),
            Self::BatteryState => write!(f, "battery_state"),
            Self::BatteryTemperature => write!(f, "battery_temperature"),
            Self::BatteryStateOfHealth => write!(f, "battery_state_of_health"),
            Self::BatteryStateOfCharge => write!(f, "battery_state_of_charge"),
            Self::BatteryEnergy => write!(f, "battery_energy"),
            Self::BatteryEnergyFull => write!(f, "battery_energy_full"),
            Self::BatteryEnergyFullDesign => write!(f, "battery_energy_full_design"),
            Self::BatteryEnergyRate => write!(f, "battery_energy_rate"),
            Self::BatteryVoltage => write!(f, "battery_voltage"),
            Self::BatteryTimeToFull => write!(f, "battery_time_to_full"),
            Self::BatteryTimeToEmpty => write!(f, "battery_time_to_empty"),
            Self::CpuName => write!(f, "cpu_name"),
            Self::CpuUsage => write!(f, "cpu_usage"),
            Self::CpuFrequency => write!(f, "cpu_frequency"),
            Self::CpuVendor => write!(f, "cpu_vendor"),
            Self::CpuArch => write!(f, "cpu_arch"),
            Self::Hostname => write!(f, "hostname"),
            Self::KernelVersion => write!(f, "kernel_version"),
            Self::KernelLongVersion => write!(f, "kernel_long_version"),
            Self::MemoryAvailable => write!(f, "memory_available"),
            Self::MemoryFree => write!(f, "memory_free"),
            Self::MemoryTotal => write!(f, "memory_total"),
            Self::MemoryUsed => write!(f, "memory_used"),
            Self::MemorySwapFree => write!(f, "memory_swap_free"),
            Self::MemorySwapTotal => write!(f, "memory_swap_total"),
            Self::MemorySwapUsage => write!(f, "memory_swap_usage"),
            Self::Uptime => write!(f, "uptime"),
            Self::Username => write!(f, "username"),
            Self::LoadAvgOne => write!(f, "loadavg_one"),
            Self::LoadAvgFive => write!(f, "loadavg_five"),
            Self::LoadAvgFifteen => write!(f, "loadavg_fifteen"),
        }
    }
}

pub struct InfoValue {
    pub field: InfoField,
    pub value: String,
}

pub struct InfoGroup {
    pub values: Vec<InfoValue>,
}

pub enum InfosResult {
    Single(InfoGroup),
    Several(Vec<InfoGroup>),
}
