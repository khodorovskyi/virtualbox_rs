#[cfg(is_v_7_1)]
use log::error;
use std::fmt::Display;
#[cfg(is_v_7_1)]
use vbox_raw::sys_lib as raw;

/// BIOS APIC initialization mode.
///
/// If the hardware does not support the mode then the code falls back to a lower mode.
#[derive(Debug, Copy, Clone)]
pub enum APICMode {
    Disabled,
    APIC,
    X2APIC
}

#[cfg(is_v_7_1)]
impl From<u32> for APICMode {
    fn from(value: u32) -> Self {
        match value {
            raw::APICMode_APICMode_Disabled => APICMode::Disabled,
            raw::APICMode_APICMode_APIC => APICMode::APIC,
            raw::APICMode_APICMode_X2APIC => APICMode::X2APIC,
            _ => {
                error!("Unknown APICMode. APICMode: {}", value);
                APICMode::Disabled
            }
        }
    }
}
#[cfg(is_v_7_1)]
impl Into<u32> for APICMode {
    fn into(self) -> u32 {
        match self {
            APICMode::Disabled => raw::APICMode_APICMode_Disabled,
            APICMode::APIC => raw::APICMode_APICMode_APIC,
            APICMode::X2APIC => raw::APICMode_APICMode_X2APIC,
        }
    }
}


#[cfg(not(is_v_7_1))]
impl From<u32> for APICMode {
    fn from(_value: u32) -> Self {
        APICMode::Disabled
    }
}

#[cfg(not(is_v_7_1))]
impl Into<u32> for APICMode {
    fn into(self) -> u32 {
        0
    }
}

impl Display for APICMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
