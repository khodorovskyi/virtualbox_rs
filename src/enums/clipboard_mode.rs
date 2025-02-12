use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Host-Guest clipboard interchange mode.
#[derive(Debug)]
pub enum ClipboardMode {
    Disabled,
    HostToGuest,
    GuestToHost,
    Bidirectional,
}

impl From<u32> for ClipboardMode {
    fn from(value: u32) -> Self {
        match value {
            raw::ClipboardMode_ClipboardMode_Disabled => ClipboardMode::Disabled,
            raw::ClipboardMode_ClipboardMode_HostToGuest => ClipboardMode::HostToGuest,
            raw::ClipboardMode_ClipboardMode_GuestToHost => ClipboardMode::GuestToHost,
            raw::ClipboardMode_ClipboardMode_Bidirectional => ClipboardMode::Bidirectional,
            _ => {
                error!("Unknown ClipboardMode: {}", value);
                ClipboardMode::Disabled
            }
        }
    }
}

impl Into<u32> for ClipboardMode {
    fn into(self) -> u32 {
        match self {
            ClipboardMode::Disabled => raw::ClipboardMode_ClipboardMode_Disabled,
            ClipboardMode::HostToGuest => raw::ClipboardMode_ClipboardMode_HostToGuest,
            ClipboardMode::GuestToHost => raw::ClipboardMode_ClipboardMode_GuestToHost,
            ClipboardMode::Bidirectional => raw::ClipboardMode_ClipboardMode_Bidirectional,
        }
    }
}

impl Display for ClipboardMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
