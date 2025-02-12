use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Protocol definitions used with NAT port-forwarding rules.
#[derive(Debug)]
pub enum NATProtocol {
    /// Port-forwarding uses UDP protocol.
    UDP,
    /// Port-forwarding uses TCP protocol.
    TCP,
}

impl From<u32> for NATProtocol {
    fn from(value: u32) -> Self {
        match value {
            raw::NATProtocol_NATProtocol_UDP => NATProtocol::UDP,
            raw::NATProtocol_NATProtocol_TCP => NATProtocol::TCP,
            _ => {
                error!("Unknoown NATProtocol value: {}", value);
                NATProtocol::UDP
            }
        }
    }
}

impl Into<u32> for NATProtocol {
    fn into(self) -> u32 {
        match self {
            NATProtocol::UDP => raw::NATProtocol_NATProtocol_UDP,
            NATProtocol::TCP => raw::NATProtocol_NATProtocol_TCP,
        }
    }
}

impl Display for NATProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
