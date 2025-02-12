#[cfg(is_v_7_1)]
use log::error;
use std::fmt::Display;
#[cfg(is_v_7_1)]
use vbox_raw::sys_lib as raw;

/// Firmware boot menu mode.
#[derive(Debug, Copy, Clone)]
pub enum FirmwareBootMenuMode {
    Disabled,
    MenuOnly,
    MessageAndMenu
}

#[cfg(is_v_7_1)]
impl From<u32> for FirmwareBootMenuMode {
    fn from(value: u32) -> Self {
        match value {
            raw::FirmwareBootMenuMode_FirmwareBootMenuMode_Disabled => FirmwareBootMenuMode::Disabled,
            raw::FirmwareBootMenuMode_FirmwareBootMenuMode_MenuOnly => FirmwareBootMenuMode::MenuOnly,
            raw::FirmwareBootMenuMode_FirmwareBootMenuMode_MessageAndMenu => FirmwareBootMenuMode::MessageAndMenu,
            _ => {
                error!("Unknown FirmwareBootMenuMode. FirmwareBootMenuMode: {}", value);
                FirmwareBootMenuMode::Disabled
            }
        }
    }
}
#[cfg(is_v_7_1)]
impl Into<u32> for FirmwareBootMenuMode {
    fn into(self) -> u32 {
        match self {
            FirmwareBootMenuMode::Disabled => raw::FirmwareBootMenuMode_FirmwareBootMenuMode_Disabled,
            FirmwareBootMenuMode::MenuOnly => raw::FirmwareBootMenuMode_FirmwareBootMenuMode_MenuOnly,
            FirmwareBootMenuMode::MessageAndMenu => raw::FirmwareBootMenuMode_FirmwareBootMenuMode_MessageAndMenu,
        }
    }
}


#[cfg(not(is_v_7_1))]
impl From<u32> for FirmwareBootMenuMode {
    fn from(_value: u32) -> Self {
        FirmwareBootMenuMode::Disabled
    }
}

#[cfg(not(is_v_7_1))]
impl Into<u32> for FirmwareBootMenuMode {
    fn into(self) -> u32 {
        0
    }
}

impl Display for FirmwareBootMenuMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
