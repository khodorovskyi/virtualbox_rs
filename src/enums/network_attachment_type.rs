use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Network attachment type.
#[derive(Debug)]
pub enum NetworkAttachmentType {
    /// Null value, also means "not attached".
    Null,
    NAT,
    Bridged,
    Internal,
    HostOnly,
    Generic,
    NATNetwork,
    Cloud,
    HostOnlyNetwork,
}

impl From<u32> for NetworkAttachmentType {
    fn from(value: u32) -> Self {
        match value {
            raw::NetworkAttachmentType_NetworkAttachmentType_Null => NetworkAttachmentType::Null,
            raw::NetworkAttachmentType_NetworkAttachmentType_NAT => NetworkAttachmentType::NAT,
            raw::NetworkAttachmentType_NetworkAttachmentType_Bridged => {
                NetworkAttachmentType::Bridged
            }
            raw::NetworkAttachmentType_NetworkAttachmentType_Internal => {
                NetworkAttachmentType::Internal
            }
            raw::NetworkAttachmentType_NetworkAttachmentType_HostOnly => {
                NetworkAttachmentType::HostOnly
            }
            raw::NetworkAttachmentType_NetworkAttachmentType_Generic => {
                NetworkAttachmentType::Generic
            }
            raw::NetworkAttachmentType_NetworkAttachmentType_NATNetwork => {
                NetworkAttachmentType::NATNetwork
            }
            raw::NetworkAttachmentType_NetworkAttachmentType_Cloud => NetworkAttachmentType::Cloud,
            #[cfg(not(is_v_6_1))]
            raw::NetworkAttachmentType_NetworkAttachmentType_HostOnlyNetwork => {
                NetworkAttachmentType::HostOnlyNetwork
            }
            _ => {
                error!("Unknown NetworkAttachmentType. Type: {}", value);
                NetworkAttachmentType::Null
            }
        }
    }
}

impl Into<u32> for NetworkAttachmentType {
    fn into(self) -> u32 {
        match self {
            NetworkAttachmentType::Null => raw::NetworkAttachmentType_NetworkAttachmentType_Null,
            NetworkAttachmentType::NAT => raw::NetworkAttachmentType_NetworkAttachmentType_NAT,
            NetworkAttachmentType::Bridged => {
                raw::NetworkAttachmentType_NetworkAttachmentType_Bridged
            }
            NetworkAttachmentType::Internal => {
                raw::NetworkAttachmentType_NetworkAttachmentType_Internal
            }
            NetworkAttachmentType::HostOnly => {
                raw::NetworkAttachmentType_NetworkAttachmentType_HostOnly
            }
            NetworkAttachmentType::Generic => {
                raw::NetworkAttachmentType_NetworkAttachmentType_Generic
            }
            NetworkAttachmentType::NATNetwork => {
                raw::NetworkAttachmentType_NetworkAttachmentType_NATNetwork
            }
            NetworkAttachmentType::Cloud => raw::NetworkAttachmentType_NetworkAttachmentType_Cloud,
            #[cfg(not(is_v_6_1))]
            NetworkAttachmentType::HostOnlyNetwork => {
                raw::NetworkAttachmentType_NetworkAttachmentType_HostOnlyNetwork
            }
            #[cfg(is_v_6_1)]
            _ => raw::NetworkAttachmentType_NetworkAttachmentType_Null,
        }
    }
}

impl Display for NetworkAttachmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
