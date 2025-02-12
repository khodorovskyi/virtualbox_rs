#[cfg(doc)]
use crate::{Guest, GuestSession};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Guest session waiting flags.

#[derive(Debug, Copy, Clone)]
pub enum GuestSessionWaitForFlag {
    /// No waiting flags specified.
    ///
    /// Not being used.
    None,
    /// Wait for the guest session being started.
    Start,
    /// Wait for the guest session being terminated.
    Terminate,
    /// Wait for the next guest session status change.
    Status,
}

impl From<u32> for GuestSessionWaitForFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_None => {
                GuestSessionWaitForFlag::None
            }
            raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_Start => {
                GuestSessionWaitForFlag::Start
            }
            raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_Terminate => {
                GuestSessionWaitForFlag::Terminate
            }
            raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_Status => {
                GuestSessionWaitForFlag::Status
            }
            _ => {
                error!("Unknown GuestSessionWaitForFlag. Type: {}", value);
                GuestSessionWaitForFlag::None
            }
        }
    }
}

impl Into<u32> for GuestSessionWaitForFlag {
    fn into(self) -> u32 {
        match self {
            GuestSessionWaitForFlag::None => {
                raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_None
            }
            GuestSessionWaitForFlag::Start => {
                raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_Start
            }
            GuestSessionWaitForFlag::Terminate => {
                raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_Terminate
            }
            GuestSessionWaitForFlag::Status => {
                raw::GuestSessionWaitForFlag_GuestSessionWaitForFlag_Status
            }
        }
    }
}

impl Display for GuestSessionWaitForFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
