use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Network interface type.
///
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HostNetworkInterfaceType {
    Bridged,
    HostOnly,
}

impl From<u32> for HostNetworkInterfaceType {
    fn from(value: u32) -> Self {
        match value {
            raw::HostNetworkInterfaceType_HostNetworkInterfaceType_Bridged => Self::Bridged,
            _ => Self::HostOnly,
        }
    }
}

impl Display for HostNetworkInterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
