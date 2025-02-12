#[cfg(doc)]
use crate::enums::GuestSessionWaitForFlag;
#[cfg(doc)]
use crate::{Guest, GuestSession};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Guest session waiting results.
///
/// Depending on the session waiting flags (for more information see [`GuestSessionWaitForFlag`]) the waiting result can vary based on the session's current status.
///
/// To wait for a guest session to terminate after it has been created by [`Guest::create_session`] one would specify [`GuestSessionWaitResult::Terminate`].
#[derive(Debug)]
pub enum GuestSessionWaitResult {
    /// No result was returned.
    ///
    /// Not being used.
    None,
    /// The guest session has been started.
    Start,
    /// The guest session has been terminated.
    Terminate,
    /// The guest session has changed its status.
    ///
    /// The status then can be retrieved via [`GuestSession::get_status`].
    Status,
    /// Error while executing the process.
    Error,
    /// The waiting operation timed out.
    ///
    /// This also will happen when no event has been occurred matching the current waiting flags in a [`GuestSession::wait_for`] call.
    Timeout,
    /// A waiting flag specified in the [`GuestSession::wait_for`] call is not supported by the guest.
    WaitFlagNotSupported,
}

impl From<u32> for GuestSessionWaitResult {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestSessionWaitResult_GuestSessionWaitResult_None => GuestSessionWaitResult::None,
            raw::GuestSessionWaitResult_GuestSessionWaitResult_Start => {
                GuestSessionWaitResult::Start
            }
            raw::GuestSessionWaitResult_GuestSessionWaitResult_Terminate => {
                GuestSessionWaitResult::Terminate
            }
            raw::GuestSessionWaitResult_GuestSessionWaitResult_Status => {
                GuestSessionWaitResult::Status
            }
            raw::GuestSessionWaitResult_GuestSessionWaitResult_Error => {
                GuestSessionWaitResult::Error
            }
            raw::GuestSessionWaitResult_GuestSessionWaitResult_Timeout => {
                GuestSessionWaitResult::Timeout
            }
            raw::GuestSessionWaitResult_GuestSessionWaitResult_WaitFlagNotSupported => {
                GuestSessionWaitResult::WaitFlagNotSupported
            }
            _ => {
                error!("Unknown GuestSessionWaitResult. Type: {}", value);
                GuestSessionWaitResult::None
            }
        }
    }
}

impl Into<u32> for GuestSessionWaitResult {
    fn into(self) -> u32 {
        match self {
            GuestSessionWaitResult::None => raw::GuestSessionWaitResult_GuestSessionWaitResult_None,
            GuestSessionWaitResult::Start => {
                raw::GuestSessionWaitResult_GuestSessionWaitResult_Start
            }
            GuestSessionWaitResult::Terminate => {
                raw::GuestSessionWaitResult_GuestSessionWaitResult_Terminate
            }
            GuestSessionWaitResult::Status => {
                raw::GuestSessionWaitResult_GuestSessionWaitResult_Status
            }
            GuestSessionWaitResult::Error => {
                raw::GuestSessionWaitResult_GuestSessionWaitResult_Error
            }
            GuestSessionWaitResult::Timeout => {
                raw::GuestSessionWaitResult_GuestSessionWaitResult_Timeout
            }
            GuestSessionWaitResult::WaitFlagNotSupported => {
                raw::GuestSessionWaitResult_GuestSessionWaitResult_WaitFlagNotSupported
            }
        }
    }
}

impl Display for GuestSessionWaitResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
