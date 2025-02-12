use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// File moving flags.

#[derive(Debug, Copy, Clone)]
pub enum FsObjMoveFlag {
    /// No flag set.
    None,

    /// Replace the destination file, symlink, etc if it exists, however this does not allow replacing any directories.
    Replace,

    /// Follow symbolic links in the final components or not (only applied to the given source and target paths, not to anything else).
    FollowLinks,

    /// Allow moving directories accross file system boundraries.
    ///
    /// Because it is could be a big undertaking, we require extra assurance that we should do it when requested.
    AllowDirectoryMoves,
}

impl From<u32> for FsObjMoveFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::FsObjMoveFlag_FsObjMoveFlag_None => FsObjMoveFlag::None,
            raw::FsObjMoveFlag_FsObjMoveFlag_Replace => FsObjMoveFlag::Replace,
            raw::FsObjMoveFlag_FsObjMoveFlag_FollowLinks => FsObjMoveFlag::FollowLinks,
            raw::FsObjMoveFlag_FsObjMoveFlag_AllowDirectoryMoves => {
                FsObjMoveFlag::AllowDirectoryMoves
            }
            _ => {
                error!("Unknown FsObjMoveFlag. FsObjMoveFlag: {}", value);
                FsObjMoveFlag::None
            }
        }
    }
}

impl Into<u32> for FsObjMoveFlag {
    fn into(self) -> u32 {
        match self {
            FsObjMoveFlag::None => raw::FsObjMoveFlag_FsObjMoveFlag_None,
            FsObjMoveFlag::Replace => raw::FsObjMoveFlag_FsObjMoveFlag_Replace,
            FsObjMoveFlag::FollowLinks => raw::FsObjMoveFlag_FsObjMoveFlag_FollowLinks,
            FsObjMoveFlag::AllowDirectoryMoves => {
                raw::FsObjMoveFlag_FsObjMoveFlag_AllowDirectoryMoves
            }
        }
    }
}

impl Display for FsObjMoveFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
