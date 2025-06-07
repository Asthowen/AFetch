use crate::error::FetchInfosError;

pub mod battery;
pub mod cpu;
pub mod host;
pub mod kernel;
pub mod memory;
pub mod uptime;

pub type InfoFunction = fn(fn(&str) -> &str) -> Result<InfosResult, FetchInfosError>;

#[derive(Debug)]
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
}

impl std::fmt::Display for InfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BatteryModel => write!(f, "batterymodel"),
            Self::BatteryCycleCount => write!(f, "batterycyclecount"),
            Self::BatterySerialNumber => write!(f, "batteryserialnumber"),
            Self::BatteryVendor => write!(f, "batteryvendor"),
            Self::BatteryTechnology => write!(f, "batterytechnology"),
            Self::BatteryState => write!(f, "batterystate"),
            Self::BatteryTemperature => write!(f, "batterytemperature"),
            Self::BatteryStateOfHealth => write!(f, "batterystateofhealth"),
            Self::BatteryStateOfCharge => write!(f, "batterystateofcharge"),
            Self::BatteryEnergy => write!(f, "batteryenergy"),
            Self::BatteryEnergyFull => write!(f, "batteryenergyfull"),
            Self::BatteryEnergyFullDesign => write!(f, "batteryenergyfulldesign"),
            Self::BatteryEnergyRate => write!(f, "batteryenergyrate"),
            Self::BatteryVoltage => write!(f, "batteryvoltage"),
            Self::BatteryTimeToFull => write!(f, "batterytimetofull"),
            Self::BatteryTimeToEmpty => write!(f, "batterytimetoempty"),
            Self::CpuName => write!(f, "cpuname"),
            Self::CpuUsage => write!(f, "cpuusage"),
            Self::CpuFrequency => write!(f, "cpufrequency"),
            Self::CpuVendor => write!(f, "cpuvendor"),
            Self::CpuArch => write!(f, "cpuarch"),
            Self::Hostname => write!(f, "hostname"),
            Self::KernelVersion => write!(f, "kernelversion"),
            Self::KernelLongVersion => write!(f, "kernellongversion"),
            Self::MemoryAvailable => write!(f, "memoryavailable"),
            Self::MemoryFree => write!(f, "memoryfree"),
            Self::MemoryTotal => write!(f, "memorytotal"),
            Self::MemoryUsed => write!(f, "memoryused"),
            Self::MemorySwapFree => write!(f, "memoryswapfree"),
            Self::MemorySwapTotal => write!(f, "memoryswaptotal"),
            Self::MemorySwapUsage => write!(f, "memoryswapusage"),
            Self::Uptime => write!(f, "uptime"),
            Self::Username => write!(f, "username"),
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
