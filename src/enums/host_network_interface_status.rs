use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Current status of the interface.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HostNetworkInterfaceStatus {
    /// The state of interface cannot be determined.
    Unknown,
    /// The interface is fully operational.
    Up,
    /// The interface is not functioning.
    Down,
}

impl From<u32> for HostNetworkInterfaceStatus {
    fn from(value: u32) -> Self {
        match value {
            raw::HostNetworkInterfaceStatus_HostNetworkInterfaceStatus_Up => Self::Up,
            raw::HostNetworkInterfaceStatus_HostNetworkInterfaceStatus_Down => Self::Down,
            _ => Self::Unknown,
        }
    }
}

impl Display for HostNetworkInterfaceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
