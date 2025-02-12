#[cfg(doc)]
use crate::GuestSession;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Flags for use when renaming file system objects (files, directories, symlink, etc), see [`GuestSession::fs_obj_rename`].

#[derive(Debug, Copy, Clone)]
pub enum FsObjRenameFlag {
    /// Do not replace any destination object.
    NoReplace,

    /// This will attempt to replace any destination object other except directories.
    ///
    /// (The default is to fail if the destination exists.)
    Replace,
}

impl From<u32> for FsObjRenameFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::FsObjRenameFlag_FsObjRenameFlag_NoReplace => FsObjRenameFlag::NoReplace,
            raw::FsObjRenameFlag_FsObjRenameFlag_Replace => FsObjRenameFlag::Replace,
            _ => {
                error!("Unknown FsObjRenameFlag. FsObjRenameFlag: {}", value);
                FsObjRenameFlag::NoReplace
            }
        }
    }
}

impl Into<u32> for FsObjRenameFlag {
    fn into(self) -> u32 {
        match self {
            FsObjRenameFlag::NoReplace => raw::FsObjRenameFlag_FsObjRenameFlag_NoReplace,
            FsObjRenameFlag::Replace => raw::FsObjRenameFlag_FsObjRenameFlag_Replace,
        }
    }
}

impl Display for FsObjRenameFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
