use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Partition table types.
#[derive(Debug, Copy, Clone)]
pub enum PartitionTableType {
    MBR,
    GPT
}

impl From<u32> for PartitionTableType {
    fn from(value: u32) -> Self {
        match value {
            raw::PartitionTableType_PartitionTableType_MBR => PartitionTableType::MBR,
            raw::PartitionTableType_PartitionTableType_GPT  => PartitionTableType::GPT ,
            _ => {
                error!("Unknown PartitionTableType. Flag: {}", value);
                PartitionTableType::MBR
            }
        }
    }
}

impl Into<u32> for PartitionTableType {
    fn into(self) -> u32 {
        match self {
            PartitionTableType::MBR => raw::PartitionTableType_PartitionTableType_MBR,
            PartitionTableType::GPT  => raw::PartitionTableType_PartitionTableType_GPT ,
        }
    }
}

impl Display for PartitionTableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
