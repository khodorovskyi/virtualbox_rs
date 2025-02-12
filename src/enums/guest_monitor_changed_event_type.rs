use std::fmt::Display;
use vbox_raw::sys_lib as raw;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]

pub enum GuestMonitorChangedEventType {
    /// The guest monitor has been enabled by the guest.
    Enabled,
    /// The guest monitor has been disabled by the guest.
    Disabled,
    /// The guest monitor origin has changed in the guest.
    NewOrigin,
}

impl From<u32> for GuestMonitorChangedEventType {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestMonitorChangedEventType_GuestMonitorChangedEventType_Enabled => {
                GuestMonitorChangedEventType::Enabled
            }
            raw::GuestMonitorChangedEventType_GuestMonitorChangedEventType_Disabled => {
                GuestMonitorChangedEventType::Disabled
            }
            _ => GuestMonitorChangedEventType::NewOrigin,
        }
    }
}
impl Display for GuestMonitorChangedEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
