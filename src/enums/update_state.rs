#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;

#[derive(Debug)]
pub enum UpdateState {
    Invalid,
    Available,
    NotAvailable,
    Downloading,
    Downloaded,
    Installing,
    Installed,
    UserInteraction,
    Canceled,
    Maintenance,
    Error,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for UpdateState {
    fn from(value: u32) -> Self {
        match value {
            raw::UpdateState_UpdateState_Invalid => UpdateState::Invalid,
            raw::UpdateState_UpdateState_Available => UpdateState::Available,
            raw::UpdateState_UpdateState_NotAvailable => UpdateState::NotAvailable,
            raw::UpdateState_UpdateState_Downloading => UpdateState::Downloading,
            raw::UpdateState_UpdateState_Downloaded => UpdateState::Downloaded,
            raw::UpdateState_UpdateState_Installing => UpdateState::Installing,
            raw::UpdateState_UpdateState_Installed => UpdateState::Installed,
            raw::UpdateState_UpdateState_UserInteraction => UpdateState::UserInteraction,
            raw::UpdateState_UpdateState_Canceled => UpdateState::Canceled,
            raw::UpdateState_UpdateState_Maintenance => UpdateState::Maintenance,
            raw::UpdateState_UpdateState_Error => UpdateState::Error,
            _ => {
                error!("Unknown UpdateState. State: {}", value);
                UpdateState::Invalid
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for UpdateState {
    fn into(self) -> u32 {
        match self {
            UpdateState::Invalid => raw::UpdateState_UpdateState_Invalid,
            UpdateState::Available => raw::UpdateState_UpdateState_Available,
            UpdateState::NotAvailable => raw::UpdateState_UpdateState_NotAvailable,
            UpdateState::Downloading => raw::UpdateState_UpdateState_Downloading,
            UpdateState::Downloaded => raw::UpdateState_UpdateState_Downloaded,
            UpdateState::Installing => raw::UpdateState_UpdateState_Installing,
            UpdateState::Installed => raw::UpdateState_UpdateState_Installed,
            UpdateState::UserInteraction => raw::UpdateState_UpdateState_UserInteraction,
            UpdateState::Canceled => raw::UpdateState_UpdateState_Canceled,
            UpdateState::Maintenance => raw::UpdateState_UpdateState_Maintenance,
            UpdateState::Error => raw::UpdateState_UpdateState_Error,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for UpdateState {
    fn from(_value: u32) -> Self {
        UpdateState::Invalid
    }
}
#[cfg(is_v_6_1)]
impl Into<u32> for UpdateState {
    fn into(self) -> u32 {
        0
    }
}
impl Display for UpdateState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
