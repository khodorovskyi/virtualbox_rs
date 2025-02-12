#[cfg(doc)]
use crate::GuestDebugControl;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;

/// The enabled guest debug provider.
///
/// This enumeration represents possible values for the [`GuestDebugControl::get_debug_provider`] attribute.
#[derive(Debug)]
pub enum GuestDebugProvider {
    /// Guest debugging is disabled.
    None,
    /// The native debugger console is enabled.
    Native,
    /// The GDB remote stub is enabled.
    GDB,
    /// The WinDbg/KD remote stub is enabled.
    KD,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for GuestDebugProvider {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestDebugProvider_GuestDebugProvider_None => GuestDebugProvider::None,
            raw::GuestDebugProvider_GuestDebugProvider_Native => GuestDebugProvider::Native,
            raw::GuestDebugProvider_GuestDebugProvider_GDB => GuestDebugProvider::GDB,
            raw::GuestDebugProvider_GuestDebugProvider_KD => GuestDebugProvider::KD,
            _ => {
                error!("Unknown GuestDebugProvider. Provider: {}", value);
                GuestDebugProvider::None
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for GuestDebugProvider {
    fn into(self) -> u32 {
        match self {
            GuestDebugProvider::None => raw::GuestDebugProvider_GuestDebugProvider_None,
            GuestDebugProvider::Native => raw::GuestDebugProvider_GuestDebugProvider_Native,
            GuestDebugProvider::GDB => raw::GuestDebugProvider_GuestDebugProvider_GDB,
            GuestDebugProvider::KD => raw::GuestDebugProvider_GuestDebugProvider_KD,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for GuestDebugProvider {
    fn from(_value: u32) -> Self {
        GuestDebugProvider::None
    }
}

#[cfg(is_v_6_1)]
impl Into<u32> for GuestDebugProvider {
    fn into(self) -> u32 {
        0
    }
}
impl Display for GuestDebugProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
