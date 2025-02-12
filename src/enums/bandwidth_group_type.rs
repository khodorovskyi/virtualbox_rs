use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Type of a bandwidth control group.
#[derive(Debug)]
pub enum BandwidthGroupType {
    /// Null type, must be first.
    Null,
    /// The bandwidth group controls disk I/O.
    Disk,
    /// The bandwidth group controls network I/O.
    Network,
}

impl From<u32> for BandwidthGroupType {
    fn from(value: u32) -> Self {
        match value {
            raw::BandwidthGroupType_BandwidthGroupType_Null => BandwidthGroupType::Null,
            raw::BandwidthGroupType_BandwidthGroupType_Disk => BandwidthGroupType::Disk,
            raw::BandwidthGroupType_BandwidthGroupType_Network => BandwidthGroupType::Network,
            _ => {
                error!("Unknown BandwidthGroupType. Type: {}", value);
                BandwidthGroupType::Null
            }
        }
    }
}

impl Into<u32> for BandwidthGroupType {
    fn into(self) -> u32 {
        match self {
            BandwidthGroupType::Null => raw::BandwidthGroupType_BandwidthGroupType_Null,
            BandwidthGroupType::Disk => raw::BandwidthGroupType_BandwidthGroupType_Disk,
            BandwidthGroupType::Network => raw::BandwidthGroupType_BandwidthGroupType_Network,
        }
    }
}

impl Display for BandwidthGroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
