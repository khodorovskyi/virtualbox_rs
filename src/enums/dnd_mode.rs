use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Drag and drop interchange mode.
#[derive(Debug)]
pub enum DnDMode {
    Disabled,
    HostToGuest,
    GuestToHost,
    Bidirectional,
}

impl From<u32> for DnDMode {
    fn from(value: u32) -> Self {
        match value {
            raw::DnDMode_DnDMode_Disabled => DnDMode::Disabled,
            raw::DnDMode_DnDMode_HostToGuest => DnDMode::HostToGuest,
            raw::DnDMode_DnDMode_GuestToHost => DnDMode::GuestToHost,
            raw::DnDMode_DnDMode_Bidirectional => DnDMode::Bidirectional,
            _ => {
                error!("Unknown DnDMode: {}", value);
                DnDMode::Disabled
            }
        }
    }
}

impl Into<u32> for DnDMode {
    fn into(self) -> u32 {
        match self {
            DnDMode::Disabled => raw::DnDMode_DnDMode_Disabled,
            DnDMode::HostToGuest => raw::DnDMode_DnDMode_HostToGuest,
            DnDMode::GuestToHost => raw::DnDMode_DnDMode_GuestToHost,
            DnDMode::Bidirectional => raw::DnDMode_DnDMode_Bidirectional,
        }
    }
}

impl Display for DnDMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
