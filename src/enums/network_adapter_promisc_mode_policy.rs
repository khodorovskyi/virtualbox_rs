use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// The promiscuous mode policy of an interface.
#[derive(Debug)]
pub enum NetworkAdapterPromiscModePolicy {
    /// Deny promiscuous mode requests.
    Deny,
    /// Allow promiscuous mode, but restrict the scope it to the internal network so that it only applies to other VMs.
    AllowNetwork,
    /// Allow promiscuous mode, include unrelated traffic going over the wire and internally on the host.
    AllowAll,
}

impl From<u32> for NetworkAdapterPromiscModePolicy {
    fn from(value: u32) -> Self {
        match value {
            raw::NetworkAdapterPromiscModePolicy_NetworkAdapterPromiscModePolicy_Deny => {
                NetworkAdapterPromiscModePolicy::Deny
            }
            raw::NetworkAdapterPromiscModePolicy_NetworkAdapterPromiscModePolicy_AllowNetwork => {
                NetworkAdapterPromiscModePolicy::AllowNetwork
            }
            raw::NetworkAdapterPromiscModePolicy_NetworkAdapterPromiscModePolicy_AllowAll => {
                NetworkAdapterPromiscModePolicy::AllowAll
            }
            _ => {
                error!(
                    "Unknown NetworkAdapterPromiscModePolicy. ModePolicy: {}",
                    value
                );
                NetworkAdapterPromiscModePolicy::Deny
            }
        }
    }
}

impl Into<u32> for NetworkAdapterPromiscModePolicy {
    fn into(self) -> u32 {
        match self {
            NetworkAdapterPromiscModePolicy::Deny => {
                raw::NetworkAdapterPromiscModePolicy_NetworkAdapterPromiscModePolicy_Deny
            }
            NetworkAdapterPromiscModePolicy::AllowNetwork => {
                raw::NetworkAdapterPromiscModePolicy_NetworkAdapterPromiscModePolicy_AllowNetwork
            }
            NetworkAdapterPromiscModePolicy::AllowAll => {
                raw::NetworkAdapterPromiscModePolicy_NetworkAdapterPromiscModePolicy_AllowAll
            }
        }
    }
}

impl Display for NetworkAdapterPromiscModePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
