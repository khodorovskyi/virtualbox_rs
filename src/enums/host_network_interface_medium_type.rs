use std::fmt::Display;
use vbox_raw::sys_lib as raw;
///Type of encapsulation.
///
/// Ethernet encapsulation includes both wired and wireless Ethernet connections.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HostNetworkInterfaceMediumType {
    /// The type of interface cannot be determined.
    Unknown,
    /// Ethernet frame encapsulation.
    Ethernet,
    /// Point-to-point protocol encapsulation.
    PPP,
    /// Serial line IP encapsulation.
    SLIP,
}

impl From<u32> for HostNetworkInterfaceMediumType {
    fn from(value: u32) -> Self {
        match value {
            raw::HostNetworkInterfaceMediumType_HostNetworkInterfaceMediumType_Ethernet => {
                Self::Ethernet
            }
            raw::HostNetworkInterfaceMediumType_HostNetworkInterfaceMediumType_PPP => Self::PPP,
            raw::HostNetworkInterfaceMediumType_HostNetworkInterfaceMediumType_SLIP => Self::SLIP,
            _ => Self::Unknown,
        }
    }
}

impl Display for HostNetworkInterfaceMediumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
