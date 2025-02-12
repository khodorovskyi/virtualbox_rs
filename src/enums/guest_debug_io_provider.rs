#[cfg(doc)]
use crate::enums::GuestDebugProvider;
#[cfg(doc)]
use crate::GuestDebugControl;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;

/// The enabled guest debug I/O provider.
///
/// This enumeration represents possible values for the [`GuestDebugControl::get_debug_io_provider`] attribute.
#[derive(Debug)]
pub enum GuestDebugIoProvider {
    /// No connection available (only useful with [`GuestDebugProvider::None`]).
    None,
    /// The remote stub is available through a TCP connection.
    TCP,
    /// The remote stub is available through a UDP connection.
    UDP,
    /// The remote stub is available through a IPC connection, namely a named pipe on Windows or a unix socket on other hosts.
    IPC,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for GuestDebugIoProvider {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestDebugIoProvider_GuestDebugIoProvider_None => GuestDebugIoProvider::None,
            raw::GuestDebugIoProvider_GuestDebugIoProvider_TCP => GuestDebugIoProvider::TCP,
            raw::GuestDebugIoProvider_GuestDebugIoProvider_UDP => GuestDebugIoProvider::UDP,
            raw::GuestDebugIoProvider_GuestDebugIoProvider_IPC => GuestDebugIoProvider::IPC,
            _ => {
                error!("Unknown GuestDebugIoProvider. Provider: {}", value);
                GuestDebugIoProvider::None
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for GuestDebugIoProvider {
    fn into(self) -> u32 {
        match self {
            GuestDebugIoProvider::None => raw::GuestDebugIoProvider_GuestDebugIoProvider_None,
            GuestDebugIoProvider::TCP => raw::GuestDebugIoProvider_GuestDebugIoProvider_TCP,
            GuestDebugIoProvider::UDP => raw::GuestDebugIoProvider_GuestDebugIoProvider_UDP,
            GuestDebugIoProvider::IPC => raw::GuestDebugIoProvider_GuestDebugIoProvider_IPC,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for GuestDebugIoProvider {
    fn from(_value: u32) -> Self {
        GuestDebugIoProvider::None
    }
}

#[cfg(is_v_6_1)]
impl Into<u32> for GuestDebugIoProvider {
    fn into(self) -> u32 {
        0
    }
}

impl Display for GuestDebugIoProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
