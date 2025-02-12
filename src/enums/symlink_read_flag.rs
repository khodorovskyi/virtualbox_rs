use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Symbolic link reading flags.
///
#[derive(Debug, Copy, Clone)]
pub enum SymlinkReadFlag {
    /// No flags set.
    None,

    /// Don't allow symbolic links as part of the path.
    NoSymlinks,
}

impl From<u32> for SymlinkReadFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::SymlinkReadFlag_SymlinkReadFlag_None => SymlinkReadFlag::None,
            raw::SymlinkReadFlag_SymlinkReadFlag_NoSymlinks => SymlinkReadFlag::NoSymlinks,
            _ => {
                error!("None SymlinkReadFlag. SymlinkReadFlag: {}", value);
                SymlinkReadFlag::None
            }
        }
    }
}

impl Into<u32> for SymlinkReadFlag {
    fn into(self) -> u32 {
        match self {
            SymlinkReadFlag::None => raw::SymlinkReadFlag_SymlinkReadFlag_None,
            SymlinkReadFlag::NoSymlinks => raw::SymlinkReadFlag_SymlinkReadFlag_NoSymlinks,
        }
    }
}

impl Display for SymlinkReadFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
