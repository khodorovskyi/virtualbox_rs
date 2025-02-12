use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// This enumeration represents possible values of the GuestSession::status attribute.
#[derive(Debug)]
pub enum GuestSessionStatus {
    /// Guest session is in an undefined state.
    Undefined,
    /// Guest session is being started.
    Starting,
    /// Guest session has been started.
    Started,
    /// Guest session is being terminated.
    Terminating,
    /// Guest session terminated normally.
    Terminated,
    /// Guest session timed out and was killed.
    TimedOutKilled,
    /// Guest session timed out and was not killed successfully.
    TimedOutAbnormally,
    /// Service/OS is stopping, guest session was killed.
    Down,
    /// Something went wrong.
    Error,
}

impl From<u32> for GuestSessionStatus {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestSessionStatus_GuestSessionStatus_Undefined => GuestSessionStatus::Undefined,
            raw::GuestSessionStatus_GuestSessionStatus_Starting => GuestSessionStatus::Starting,
            raw::GuestSessionStatus_GuestSessionStatus_Started => GuestSessionStatus::Started,
            raw::GuestSessionStatus_GuestSessionStatus_Terminating => {
                GuestSessionStatus::Terminating
            }
            raw::GuestSessionStatus_GuestSessionStatus_Terminated => GuestSessionStatus::Terminated,
            raw::GuestSessionStatus_GuestSessionStatus_TimedOutKilled => {
                GuestSessionStatus::TimedOutKilled
            }
            raw::GuestSessionStatus_GuestSessionStatus_TimedOutAbnormally => {
                GuestSessionStatus::TimedOutAbnormally
            }
            raw::GuestSessionStatus_GuestSessionStatus_Down => GuestSessionStatus::Down,
            raw::GuestSessionStatus_GuestSessionStatus_Error => GuestSessionStatus::Error,
            _ => {
                error!("Unknown status: {}", value);
                GuestSessionStatus::Error
            }
        }
    }
}

impl Into<u32> for GuestSessionStatus {
    fn into(self) -> u32 {
        match self {
            GuestSessionStatus::Undefined => raw::GuestSessionStatus_GuestSessionStatus_Undefined,
            GuestSessionStatus::Starting => raw::GuestSessionStatus_GuestSessionStatus_Starting,
            GuestSessionStatus::Started => raw::GuestSessionStatus_GuestSessionStatus_Started,
            GuestSessionStatus::Terminating => {
                raw::GuestSessionStatus_GuestSessionStatus_Terminating
            }
            GuestSessionStatus::Terminated => raw::GuestSessionStatus_GuestSessionStatus_Terminated,
            GuestSessionStatus::TimedOutKilled => {
                raw::GuestSessionStatus_GuestSessionStatus_TimedOutKilled
            }
            GuestSessionStatus::TimedOutAbnormally => {
                raw::GuestSessionStatus_GuestSessionStatus_TimedOutAbnormally
            }
            GuestSessionStatus::Down => raw::GuestSessionStatus_GuestSessionStatus_Down,
            GuestSessionStatus::Error => raw::GuestSessionStatus_GuestSessionStatus_Error,
        }
    }
}

impl Display for GuestSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
