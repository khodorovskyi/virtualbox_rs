#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;
#[derive(Debug)]
pub enum UpdateSeverity {
    /// No severity specified.
    ///
    /// Do not use this.
    Invalid,
    /// Update contains critical patches.
    Critical,
    /// Update contains a major version.
    Major,
    /// Update contains a minor version.
    Minor,
    /// Update contains a testing version.
    ////
    /// Use with caution!
    Testing,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for UpdateSeverity {
    fn from(value: u32) -> Self {
        match value {
            raw::UpdateSeverity_UpdateSeverity_Invalid => UpdateSeverity::Invalid,
            raw::UpdateSeverity_UpdateSeverity_Critical => UpdateSeverity::Critical,
            raw::UpdateSeverity_UpdateSeverity_Major => UpdateSeverity::Major,
            raw::UpdateSeverity_UpdateSeverity_Minor => UpdateSeverity::Minor,
            raw::UpdateSeverity_UpdateSeverity_Testing => UpdateSeverity::Testing,
            _ => {
                error!("Unknown UpdateSeverity. Severity: {}", value);
                UpdateSeverity::Invalid
            }
        }
    }
}
#[cfg(not(is_v_6_1))]
impl Into<u32> for UpdateSeverity {
    fn into(self) -> u32 {
        match self {
            UpdateSeverity::Invalid => raw::UpdateSeverity_UpdateSeverity_Invalid,
            UpdateSeverity::Critical => raw::UpdateSeverity_UpdateSeverity_Critical,
            UpdateSeverity::Major => raw::UpdateSeverity_UpdateSeverity_Major,
            UpdateSeverity::Minor => raw::UpdateSeverity_UpdateSeverity_Minor,
            UpdateSeverity::Testing => raw::UpdateSeverity_UpdateSeverity_Testing,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for UpdateSeverity {
    fn from(_value: u32) -> Self {
        UpdateSeverity::Invalid
    }
}
#[cfg(is_v_6_1)]
impl Into<u32> for UpdateSeverity {
    fn into(self) -> u32 {
        0
    }
}

impl Display for UpdateSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
