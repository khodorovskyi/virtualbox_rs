#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;

/// Audio device state enumeration.
#[derive(Debug)]
pub enum AudioDeviceState {
    /// Device state is unknown / cannot be determined.
    Unknown,
    /// Device is active and can be used.
    Active,
    /// Device is in a disabled state.
    Disabled,
    /// Device is marked as not being present.
    NotPresent,
    /// Device has been unplugged.
    Unplugged,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for AudioDeviceState {
    fn from(value: u32) -> Self {
        match value {
            raw::AudioDeviceState_AudioDeviceState_Unknown => AudioDeviceState::Unknown,
            raw::AudioDeviceState_AudioDeviceState_Active => AudioDeviceState::Active,
            raw::AudioDeviceState_AudioDeviceState_Disabled => AudioDeviceState::Disabled,
            raw::AudioDeviceState_AudioDeviceState_NotPresent => AudioDeviceState::NotPresent,
            raw::AudioDeviceState_AudioDeviceState_Unplugged => AudioDeviceState::Unplugged,
            _ => {
                error!("Unknown AudioDeviceState. State: {}", value);
                AudioDeviceState::Unknown
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for AudioDeviceState {
    fn into(self) -> u32 {
        match self {
            AudioDeviceState::Unknown => raw::AudioDeviceState_AudioDeviceState_Unknown,
            AudioDeviceState::Active => raw::AudioDeviceState_AudioDeviceState_Active,
            AudioDeviceState::Disabled => raw::AudioDeviceState_AudioDeviceState_Disabled,
            AudioDeviceState::NotPresent => raw::AudioDeviceState_AudioDeviceState_NotPresent,
            AudioDeviceState::Unplugged => raw::AudioDeviceState_AudioDeviceState_Unplugged,
        }
    }
}

#[cfg(is_v_6_1)]
impl Into<u32> for AudioDeviceState {
    fn into(self) -> u32 {
        0
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for AudioDeviceState {
    fn from(_value: u32) -> Self {
        AudioDeviceState::Unknown
    }
}

impl Display for AudioDeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
