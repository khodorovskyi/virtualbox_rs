use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Process execution statuses.
#[derive(Debug)]
pub enum ProcessStatus {
    /// Process is in an undefined state.
    Undefined,
    /// Process is being started.
    Starting,
    /// Process has been started.
    Started,
    /// Process has been paused.
    Paused,
    /// Process is being terminated.
    Terminating,
    /// Process terminated normally.
    TerminatedNormally,
    /// Process terminated via signal.
    TerminatedSignal,
    /// Process terminated abnormally.
    TerminatedAbnormally,
    /// Process timed out and was killed.
    TimedOutKilled,
    /// Process timed out and was not killed successfully.
    TimedOutAbnormally,
    /// Service/OS is stopping, process was killed.
    Down,
    /// Something went wrong.
    Error,
}

impl From<u32> for ProcessStatus {
    fn from(value: u32) -> Self {
        match value {
            raw::ProcessStatus_ProcessStatus_Undefined => ProcessStatus::Undefined,
            raw::ProcessStatus_ProcessStatus_Starting => ProcessStatus::Starting,
            raw::ProcessStatus_ProcessStatus_Started => ProcessStatus::Started,
            raw::ProcessStatus_ProcessStatus_Paused => ProcessStatus::Paused,
            raw::ProcessStatus_ProcessStatus_Terminating => ProcessStatus::Terminating,
            raw::ProcessStatus_ProcessStatus_TerminatedNormally => {
                ProcessStatus::TerminatedNormally
            }
            raw::ProcessStatus_ProcessStatus_TerminatedSignal => ProcessStatus::TerminatedSignal,
            raw::ProcessStatus_ProcessStatus_TerminatedAbnormally => {
                ProcessStatus::TerminatedAbnormally
            }
            raw::ProcessStatus_ProcessStatus_TimedOutKilled => ProcessStatus::TimedOutKilled,
            raw::ProcessStatus_ProcessStatus_TimedOutAbnormally => {
                ProcessStatus::TimedOutAbnormally
            }
            raw::ProcessStatus_ProcessStatus_Down => ProcessStatus::Down,
            raw::ProcessStatus_ProcessStatus_Error => ProcessStatus::Error,
            _ => {
                error!("Unknown status: {}", value);
                ProcessStatus::Error
            }
        }
    }
}

impl Into<u32> for ProcessStatus {
    fn into(self) -> u32 {
        match self {
            ProcessStatus::Undefined => raw::ProcessStatus_ProcessStatus_Undefined,
            ProcessStatus::Starting => raw::ProcessStatus_ProcessStatus_Starting,
            ProcessStatus::Started => raw::ProcessStatus_ProcessStatus_Started,
            ProcessStatus::Paused => raw::ProcessStatus_ProcessStatus_Paused,
            ProcessStatus::Terminating => raw::ProcessStatus_ProcessStatus_Terminating,
            ProcessStatus::TerminatedNormally => {
                raw::ProcessStatus_ProcessStatus_TerminatedNormally
            }
            ProcessStatus::TerminatedSignal => raw::ProcessStatus_ProcessStatus_TerminatedSignal,
            ProcessStatus::TerminatedAbnormally => {
                raw::ProcessStatus_ProcessStatus_TerminatedAbnormally
            }
            ProcessStatus::TimedOutKilled => raw::ProcessStatus_ProcessStatus_TimedOutKilled,
            ProcessStatus::TimedOutAbnormally => {
                raw::ProcessStatus_ProcessStatus_TimedOutAbnormally
            }
            ProcessStatus::Down => raw::ProcessStatus_ProcessStatus_Down,
            ProcessStatus::Error => raw::ProcessStatus_ProcessStatus_Error,
        }
    }
}

impl Display for ProcessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
