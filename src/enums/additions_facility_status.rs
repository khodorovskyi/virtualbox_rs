use log::error;
use std::fmt::Display;
pub use vbox_raw::sys_lib as raw;
/// Guest Additions facility states.
#[derive(Debug)]
pub enum AdditionsFacilityStatus {
    /// Facility is not active.
    Inactive,
    /// Facility has been paused.
    Paused,
    /// Facility is preparing to initialize.
    PreInit,
    /// Facility is initializing.
    Init,
    /// Facility is up and running.
    Active,
    /// Facility is shutting down.
    Terminating,
    /// Facility successfully shut down.
    Terminated,
    /// Facility failed to start.
    Failed,
    /// Facility status is unknown.
    Unknown,
}

impl From<u32> for AdditionsFacilityStatus {
    fn from(value: u32) -> Self {
        match value {
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Inactive => {
                AdditionsFacilityStatus::Inactive
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Paused => {
                AdditionsFacilityStatus::Paused
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_PreInit => {
                AdditionsFacilityStatus::PreInit
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Init => {
                AdditionsFacilityStatus::Init
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Active => {
                AdditionsFacilityStatus::Active
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Terminating => {
                AdditionsFacilityStatus::Terminating
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Terminated => {
                AdditionsFacilityStatus::Terminated
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Failed => {
                AdditionsFacilityStatus::Failed
            }
            raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Unknown => {
                AdditionsFacilityStatus::Unknown
            }
            _ => {
                error!("Unknown AdditionsFacilityStatus. Status: {}", value);
                AdditionsFacilityStatus::Unknown
            }
        }
    }
}

impl Into<u32> for AdditionsFacilityStatus {
    fn into(self) -> u32 {
        match self {
            AdditionsFacilityStatus::Inactive => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Inactive
            }
            AdditionsFacilityStatus::Paused => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Paused
            }
            AdditionsFacilityStatus::PreInit => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_PreInit
            }
            AdditionsFacilityStatus::Init => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Init
            }
            AdditionsFacilityStatus::Active => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Active
            }
            AdditionsFacilityStatus::Terminating => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Terminating
            }
            AdditionsFacilityStatus::Terminated => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Terminated
            }
            AdditionsFacilityStatus::Failed => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Failed
            }
            AdditionsFacilityStatus::Unknown => {
                raw::AdditionsFacilityStatus_AdditionsFacilityStatus_Unknown
            }
        }
    }
}

impl Display for AdditionsFacilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
