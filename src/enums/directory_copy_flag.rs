#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;
/// Directory copying flags.
#[derive(Debug, Copy, Clone)]
pub enum DirectoryCopyFlag {
    /// No flag set.
    None,

    /// Allow copying into an existing destination directory.
    CopyIntoExisting,

    /// Copy directories recursively.
    Recursive,

    /// Follow symbolic links.
    FollowLinks,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for DirectoryCopyFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::DirectoryCopyFlag_DirectoryCopyFlag_None => DirectoryCopyFlag::None,
            raw::DirectoryCopyFlag_DirectoryCopyFlag_CopyIntoExisting => {
                DirectoryCopyFlag::CopyIntoExisting
            }
            raw::DirectoryCopyFlag_DirectoryCopyFlag_Recursive => DirectoryCopyFlag::Recursive,
            raw::DirectoryCopyFlag_DirectoryCopyFlag_FollowLinks => DirectoryCopyFlag::FollowLinks,
            _ => {
                error!("Unknown DirectoryCopyFlag. DirectoryCopyFlag: {}", value);
                DirectoryCopyFlag::None
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for DirectoryCopyFlag {
    fn into(self) -> u32 {
        match self {
            DirectoryCopyFlag::None => raw::DirectoryCopyFlag_DirectoryCopyFlag_None,
            DirectoryCopyFlag::CopyIntoExisting => {
                raw::DirectoryCopyFlag_DirectoryCopyFlag_CopyIntoExisting
            }
            DirectoryCopyFlag::Recursive => raw::DirectoryCopyFlag_DirectoryCopyFlag_Recursive,
            DirectoryCopyFlag::FollowLinks => raw::DirectoryCopyFlag_DirectoryCopyFlag_FollowLinks,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for DirectoryCopyFlag {
    fn from(_value: u32) -> Self {
        DirectoryCopyFlag::None
    }
}

#[cfg(is_v_6_1)]
impl Into<u32> for DirectoryCopyFlag {
    fn into(self) -> u32 {
        0
    }
}

impl Display for DirectoryCopyFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
