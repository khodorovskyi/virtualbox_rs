use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Symbolic link types.
///
/// This is significant when creating links on the Windows platform, ignored elsewhere.

#[derive(Debug, Copy, Clone)]
pub enum SymlinkType {
    /// It is not known what is being targeted.
    Unknown,

    /// The link targets a directory.
    Directory,

    /// The link targets a file (or whatever else except directories).
    File,
}

impl From<u32> for SymlinkType {
    fn from(value: u32) -> Self {
        match value {
            raw::SymlinkType_SymlinkType_Unknown => SymlinkType::Unknown,
            raw::SymlinkType_SymlinkType_Directory => SymlinkType::Directory,
            raw::SymlinkType_SymlinkType_File => SymlinkType::File,
            _ => {
                error!("Unknown SymlinkType. SymlinkType: {}", value);
                SymlinkType::Unknown
            }
        }
    }
}

impl Into<u32> for SymlinkType {
    fn into(self) -> u32 {
        match self {
            SymlinkType::Unknown => raw::SymlinkType_SymlinkType_Unknown,
            SymlinkType::Directory => raw::SymlinkType_SymlinkType_Directory,
            SymlinkType::File => raw::SymlinkType_SymlinkType_File,
        }
    }
}

impl Display for SymlinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
