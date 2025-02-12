use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Device type.
#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    /// Null value, may also mean "no device".
    Null,
    /// Floppy device.
    Floppy,
    /// CD/DVD-ROM device.
    DVD,
    /// Hard disk device.
    HardDisk,
    /// Network device.
    Network,
    /// USB device.
    USB,
    /// Shared folder device.
    SharedFolder,
    /// Graphics device 3D activity.
    Graphics3D,
    /// End of valid device types (exclusive).
    ///
    /// Used for invalid validation and such.
    End,
}
impl Into<u32> for DeviceType {
    fn into(self) -> u32 {
        match self {
            DeviceType::Floppy => raw::DeviceType_DeviceType_Floppy,
            DeviceType::DVD => raw::DeviceType_DeviceType_DVD,
            DeviceType::HardDisk => raw::DeviceType_DeviceType_HardDisk,
            DeviceType::Network => raw::DeviceType_DeviceType_Network,
            DeviceType::USB => raw::DeviceType_DeviceType_USB,
            DeviceType::SharedFolder => raw::DeviceType_DeviceType_SharedFolder,
            DeviceType::Graphics3D => raw::DeviceType_DeviceType_Graphics3D,
            #[cfg(not(is_v_6_1))]
            DeviceType::End => raw::DeviceType_DeviceType_End,
            _ => raw::DeviceType_DeviceType_Null,
        }
    }
}

impl From<u32> for DeviceType {
    fn from(value: u32) -> Self {
        match value {
            raw::DeviceType_DeviceType_Null => DeviceType::Null,
            raw::DeviceType_DeviceType_Floppy => DeviceType::Floppy,
            raw::DeviceType_DeviceType_DVD => DeviceType::DVD,
            raw::DeviceType_DeviceType_HardDisk => DeviceType::HardDisk,
            raw::DeviceType_DeviceType_Network => DeviceType::Network,
            raw::DeviceType_DeviceType_USB => DeviceType::USB,
            raw::DeviceType_DeviceType_SharedFolder => DeviceType::SharedFolder,
            raw::DeviceType_DeviceType_Graphics3D => DeviceType::Graphics3D,
            #[cfg(not(is_v_6_1))]
            raw::DeviceType_DeviceType_End => DeviceType::End,
            _ => {
                error!("DeviceType::from. Unknown type: {}", value);
                DeviceType::Null
            }
        }
    }
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
