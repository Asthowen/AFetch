use crate::error::FetchInfosError;

pub mod cpu;
pub mod host;
pub mod kernel;
pub mod uptime;

pub type InfoFunction = fn(fn(&str) -> &str) -> Result<InfosResult, FetchInfosError>;

#[derive(Debug)]
pub enum InfoField {
    Hostname,
    Username,
    CpuName,
    CpuUsage,
    CpuFrequency,
    CpuVendor,
    CpuArch,
    KernelVersion,
    KernelLongVersion,
    Uptime,
}

impl std::fmt::Display for InfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hostname => write!(f, "hostname"),
            Self::Username => write!(f, "username"),
            Self::CpuName => write!(f, "cpuname"),
            Self::CpuUsage => write!(f, "cpuusage"),
            Self::CpuFrequency => write!(f, "cpufrequency"),
            Self::CpuVendor => write!(f, "cpuvendor"),
            Self::CpuArch => write!(f, "cpuarch"),
            Self::KernelVersion => write!(f, "kernelversion"),
            Self::KernelLongVersion => write!(f, "kernellongversion"),
            Self::Uptime => write!(f, "uptime"),
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
