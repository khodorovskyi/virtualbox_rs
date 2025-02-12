use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// The bus type of the storage controller (IDE, SATA, SCSI, SAS or Floppy).
#[derive(Debug, Eq, PartialEq)]
pub enum StorageBus {
    /// null value.
    ///
    /// Never used by the API.
    Null,
    IDE,
    SATA,
    SCSI,
    Floppy,
    SAS,
    USB,
    PCIe,
    VirtioSCSI,
}
impl Into<u32> for StorageBus {
    fn into(self) -> u32 {
        match self {
            StorageBus::IDE => raw::StorageBus_StorageBus_IDE,
            StorageBus::SATA => raw::StorageBus_StorageBus_SATA,
            StorageBus::SCSI => raw::StorageBus_StorageBus_SCSI,
            StorageBus::Floppy => raw::StorageBus_StorageBus_Floppy,
            StorageBus::SAS => raw::StorageBus_StorageBus_SAS,
            StorageBus::USB => raw::StorageBus_StorageBus_USB,
            StorageBus::PCIe => raw::StorageBus_StorageBus_PCIe,
            StorageBus::VirtioSCSI => raw::StorageBus_StorageBus_VirtioSCSI,
            _ => raw::StorageBus_StorageBus_Null,
        }
    }
}

impl From<u32> for StorageBus {
    fn from(value: u32) -> Self {
        match value {
            raw::StorageBus_StorageBus_Null => StorageBus::Null,
            raw::StorageBus_StorageBus_IDE => StorageBus::IDE,
            raw::StorageBus_StorageBus_SATA => StorageBus::SATA,
            raw::StorageBus_StorageBus_SCSI => StorageBus::SCSI,
            raw::StorageBus_StorageBus_Floppy => StorageBus::Floppy,
            raw::StorageBus_StorageBus_SAS => StorageBus::SAS,
            raw::StorageBus_StorageBus_USB => StorageBus::USB,
            raw::StorageBus_StorageBus_PCIe => StorageBus::PCIe,
            raw::StorageBus_StorageBus_VirtioSCSI => StorageBus::VirtioSCSI,
            _ => {
                error!("StorageBus::from. Unknown type: {}", value);
                StorageBus::Null
            }
        }
    }
}

impl Display for StorageBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
