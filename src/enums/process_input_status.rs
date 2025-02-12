use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Process input statuses.
#[derive(Debug)]
pub enum ProcessInputStatus {
    /// Undefined state.
    Undefined,
    /// Input pipe is broken.
    Broken,
    /// Input pipe became available for writing.
    Available,
    /// Data has been successfully written.
    Written,
    /// Too much input data supplied, data overflow.
    Overflow,
}

impl From<u32> for ProcessInputStatus {
    fn from(value: u32) -> Self {
        match value {
            raw::ProcessInputStatus_ProcessInputStatus_Undefined => ProcessInputStatus::Undefined,
            raw::ProcessInputStatus_ProcessInputStatus_Broken => ProcessInputStatus::Broken,
            raw::ProcessInputStatus_ProcessInputStatus_Available => ProcessInputStatus::Available,
            raw::ProcessInputStatus_ProcessInputStatus_Written => ProcessInputStatus::Written,
            raw::ProcessInputStatus_ProcessInputStatus_Overflow => ProcessInputStatus::Overflow,
            _ => {
                error!("Unknown ProcessInputStatus. Status: {}", value);
                ProcessInputStatus::Undefined
            }
        }
    }
}

impl Into<u32> for ProcessInputStatus {
    fn into(self) -> u32 {
        match self {
            ProcessInputStatus::Undefined => raw::ProcessInputStatus_ProcessInputStatus_Undefined,
            ProcessInputStatus::Broken => raw::ProcessInputStatus_ProcessInputStatus_Broken,
            ProcessInputStatus::Available => raw::ProcessInputStatus_ProcessInputStatus_Available,
            ProcessInputStatus::Written => raw::ProcessInputStatus_ProcessInputStatus_Written,
            ProcessInputStatus::Overflow => raw::ProcessInputStatus_ProcessInputStatus_Overflow,
        }
    }
}

impl Display for ProcessInputStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
