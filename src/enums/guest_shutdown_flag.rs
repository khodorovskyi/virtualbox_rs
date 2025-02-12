#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;
/// Guest shutdown flags.
#[derive(Debug, Copy, Clone)]
pub enum GuestShutdownFlag {
    /// No flag set.
    None,

    /// Performs a reboot after shutdown.
    PowerOff,

    /// Performs a reboot after shutdown.
    Reboot,

    ///Force the system to shutdown/reboot regardless of objecting application or other stuff.
    ///
    ///This flag might not be realized on all systems.
    Force,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for GuestShutdownFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestShutdownFlag_GuestShutdownFlag_None => GuestShutdownFlag::None,
            raw::GuestShutdownFlag_GuestShutdownFlag_PowerOff => GuestShutdownFlag::PowerOff,
            raw::GuestShutdownFlag_GuestShutdownFlag_Reboot => GuestShutdownFlag::Reboot,
            raw::GuestShutdownFlag_GuestShutdownFlag_Force => GuestShutdownFlag::Force,
            _ => {
                error!("Unknown GuestShutdownFlag. Flag: {}", value);
                GuestShutdownFlag::None
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for GuestShutdownFlag {
    fn into(self) -> u32 {
        match self {
            GuestShutdownFlag::None => raw::GuestShutdownFlag_GuestShutdownFlag_None,
            GuestShutdownFlag::PowerOff => raw::GuestShutdownFlag_GuestShutdownFlag_PowerOff,
            GuestShutdownFlag::Reboot => raw::GuestShutdownFlag_GuestShutdownFlag_Reboot,
            GuestShutdownFlag::Force => raw::GuestShutdownFlag_GuestShutdownFlag_Force,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for GuestShutdownFlag {
    fn from(_value: u32) -> Self {
        GuestShutdownFlag::None
    }
}

#[cfg(is_v_6_1)]
impl Into<u32> for GuestShutdownFlag {
    fn into(self) -> u32 {
        0
    }
}

impl Display for GuestShutdownFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
