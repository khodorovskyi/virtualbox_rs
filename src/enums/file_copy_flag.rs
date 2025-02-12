use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// File copying flags.

#[derive(Debug, Copy, Clone)]
pub enum FileCopyFlag {
    /// No flag set.
    None,

    /// Do not replace the destination file if it exists.
    NoReplace,

    /// Follow symbolic links.
    FollowLinks,

    /// Only copy when the source file is newer than the destination file or when the destination file is missing.
    Update,
}

impl From<u32> for FileCopyFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::FileCopyFlag_FileCopyFlag_None => FileCopyFlag::None,
            raw::FileCopyFlag_FileCopyFlag_NoReplace => FileCopyFlag::NoReplace,
            raw::FileCopyFlag_FileCopyFlag_FollowLinks => FileCopyFlag::FollowLinks,
            raw::FileCopyFlag_FileCopyFlag_Update => FileCopyFlag::Update,
            _ => {
                error!("Unknown FileCopyFlag. FileCopyFlag: {}", value);
                FileCopyFlag::None
            }
        }
    }
}

impl Into<u32> for FileCopyFlag {
    fn into(self) -> u32 {
        match self {
            FileCopyFlag::None => raw::FileCopyFlag_FileCopyFlag_None,
            FileCopyFlag::NoReplace => raw::FileCopyFlag_FileCopyFlag_NoReplace,
            FileCopyFlag::FollowLinks => raw::FileCopyFlag_FileCopyFlag_FollowLinks,
            FileCopyFlag::Update => raw::FileCopyFlag_FileCopyFlag_Update,
        }
    }
}

impl Display for FileCopyFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
