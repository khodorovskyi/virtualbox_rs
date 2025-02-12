#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;

#[derive(Debug)]
pub enum UpdateChannel {
    /// No channel specified.
    ///
    /// Do not use this.
    Invalid,
    /// All stable releases (maintenance and minor releases within the same major release).
    Stable,
    /// All stable releases, including major versions.
    All,
    /// All stable and major releases, including beta versions.
    WithBetas,
    /// All stable, major and beta releases, including testing versions.
    WithTesting,
}
#[cfg(not(is_v_6_1))]
impl From<u32> for UpdateChannel {
    fn from(value: u32) -> Self {
        match value {
            raw::UpdateChannel_UpdateChannel_Invalid => UpdateChannel::Invalid,
            raw::UpdateChannel_UpdateChannel_Stable => UpdateChannel::Stable,
            raw::UpdateChannel_UpdateChannel_All => UpdateChannel::All,
            raw::UpdateChannel_UpdateChannel_WithBetas => UpdateChannel::WithBetas,
            raw::UpdateChannel_UpdateChannel_WithTesting => UpdateChannel::WithTesting,
            _ => {
                error!("Unknown UpdateChannel. Channel: {}", value);
                UpdateChannel::Invalid
            }
        }
    }
}
#[cfg(not(is_v_6_1))]
impl Into<u32> for UpdateChannel {
    fn into(self) -> u32 {
        match self {
            UpdateChannel::Invalid => raw::UpdateChannel_UpdateChannel_Invalid,
            UpdateChannel::Stable => raw::UpdateChannel_UpdateChannel_Stable,
            UpdateChannel::All => raw::UpdateChannel_UpdateChannel_All,
            UpdateChannel::WithBetas => raw::UpdateChannel_UpdateChannel_WithBetas,
            UpdateChannel::WithTesting => raw::UpdateChannel_UpdateChannel_WithTesting,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for UpdateChannel {
    fn from(_value: u32) -> Self {
        UpdateChannel::Invalid
    }
}
#[cfg(is_v_6_1)]
impl Into<u32> for UpdateChannel {
    fn into(self) -> u32 {
        0
    }
}

impl Display for UpdateChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
