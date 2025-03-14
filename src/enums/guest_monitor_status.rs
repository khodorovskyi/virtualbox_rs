use std::fmt::Display;
use log::error;
use vbox_raw::sys_lib as raw;

/// The current status of the guest display.
#[derive(Debug)]
pub enum GuestMonitorStatus {
    /// The guest monitor is disabled in the guest.
    Disabled,
    /// The guest monitor is enabled in the guest.
    Enabled,
    /// The guest monitor is enabled in the guest but should display nothing.
    Blank,
}

impl From<u32> for GuestMonitorStatus {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestMonitorStatus_GuestMonitorStatus_Disabled => GuestMonitorStatus::Disabled,
            raw::GuestMonitorStatus_GuestMonitorStatus_Enabled => GuestMonitorStatus::Enabled,
            raw::GuestMonitorStatus_GuestMonitorStatus_Blank => GuestMonitorStatus::Blank,
            _ => {
                error!("Unknown GuestMonitorStatus: {}", value);
                GuestMonitorStatus::Blank
            }
        }
    }
}

impl Into<u32> for GuestMonitorStatus {
    fn into(self) -> u32 {
        match self {
            GuestMonitorStatus::Disabled => raw::GuestMonitorStatus_GuestMonitorStatus_Disabled,
            GuestMonitorStatus::Enabled => raw::GuestMonitorStatus_GuestMonitorStatus_Enabled,
            GuestMonitorStatus::Blank => raw::GuestMonitorStatus_GuestMonitorStatus_Blank,
        }
    }
}

impl Display for GuestMonitorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
