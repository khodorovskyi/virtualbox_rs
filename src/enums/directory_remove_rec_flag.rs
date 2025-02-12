use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Directory recursive removement flags.
#[derive(Debug, Copy, Clone)]
pub enum DirectoryRemoveRecFlag {
    /// No flag set.
    None,

    /// Delete the content of the directory and the directory itself.
    ContentAndDir,

    /// Only delete the content of the directory, omit the directory it self.
    ContentOnly,
}

impl From<u32> for DirectoryRemoveRecFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::DirectoryRemoveRecFlag_DirectoryRemoveRecFlag_None => DirectoryRemoveRecFlag::None,
            raw::DirectoryRemoveRecFlag_DirectoryRemoveRecFlag_ContentAndDir => {
                DirectoryRemoveRecFlag::ContentAndDir
            }
            raw::DirectoryRemoveRecFlag_DirectoryRemoveRecFlag_ContentOnly => {
                DirectoryRemoveRecFlag::ContentOnly
            }
            _ => {
                error!(
                    "Unknown DirectoryRemoveRecFlag. DirectoryRemoveRecFlag: {}",
                    value
                );
                DirectoryRemoveRecFlag::None
            }
        }
    }
}

impl Into<u32> for DirectoryRemoveRecFlag {
    fn into(self) -> u32 {
        match self {
            DirectoryRemoveRecFlag::None => raw::DirectoryRemoveRecFlag_DirectoryRemoveRecFlag_None,
            DirectoryRemoveRecFlag::ContentAndDir => {
                raw::DirectoryRemoveRecFlag_DirectoryRemoveRecFlag_ContentAndDir
            }
            DirectoryRemoveRecFlag::ContentOnly => {
                raw::DirectoryRemoveRecFlag_DirectoryRemoveRecFlag_ContentOnly
            }
        }
    }
}

impl Display for DirectoryRemoveRecFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
