use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Type of emulated chipset (mostly southbridge).
#[derive(Debug)]
pub enum ChipsetType {
    /// null value.
    ///
    /// Never used by the API.
    Null,
    /// A PIIX3 (PCI IDE ISA Xcelerator) chipset.
    PIIX3,
    /// A ICH9 (I/O Controller Hub) chipset.
    ICH9,
}

impl From<u32> for ChipsetType {
    fn from(value: u32) -> Self {
        match value {
            raw::ChipsetType_ChipsetType_Null => ChipsetType::Null,
            raw::ChipsetType_ChipsetType_PIIX3 => ChipsetType::PIIX3,
            raw::ChipsetType_ChipsetType_ICH9 => ChipsetType::ICH9,
            _ => {
                error!("Unknown ChipsetType: {}", value);
                ChipsetType::Null
            }
        }
    }
}

impl Into<u32> for ChipsetType {
    fn into(self) -> u32 {
        match self {
            ChipsetType::Null => raw::ChipsetType_ChipsetType_Null,
            ChipsetType::PIIX3 => raw::ChipsetType_ChipsetType_PIIX3,
            ChipsetType::ICH9 => raw::ChipsetType_ChipsetType_ICH9,
        }
    }
}

impl Display for ChipsetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
