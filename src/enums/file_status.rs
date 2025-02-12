use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// File statuses.
#[derive(Debug)]
pub enum FileStatus {
    /// File is in an undefined state.
    Undefined,
    /// Guest file is opening.
    Opening,
    /// Guest file has been successfully opened.
    Open,
    /// Guest file closing.
    Closing,
    /// Guest file has been closed.
    Closed,
    /// Service/OS is stopping, guest file was closed.
    Down,
    /// Something went wrong.
    Error,
}

impl From<u32> for FileStatus {
    fn from(value: u32) -> Self {
        match value {
            raw::FileStatus_FileStatus_Undefined => FileStatus::Undefined,
            raw::FileStatus_FileStatus_Opening => FileStatus::Opening,
            raw::FileStatus_FileStatus_Open => FileStatus::Open,
            raw::FileStatus_FileStatus_Closing => FileStatus::Closing,
            raw::FileStatus_FileStatus_Down => FileStatus::Down,
            raw::FileStatus_FileStatus_Error => FileStatus::Error,
            _ => {
                error!("Unknown FileStatus. Status: {}", value);
                FileStatus::Undefined
            }
        }
    }
}

impl Into<u32> for FileStatus {
    fn into(self) -> u32 {
        match self {
            FileStatus::Undefined => raw::FileStatus_FileStatus_Undefined,
            FileStatus::Opening => raw::FileStatus_FileStatus_Opening,
            FileStatus::Open => raw::FileStatus_FileStatus_Open,
            FileStatus::Closing => raw::FileStatus_FileStatus_Closing,
            FileStatus::Closed => raw::FileStatus_FileStatus_Closed,
            FileStatus::Down => raw::FileStatus_FileStatus_Down,
            FileStatus::Error => raw::FileStatus_FileStatus_Error,
        }
    }
}

impl Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
