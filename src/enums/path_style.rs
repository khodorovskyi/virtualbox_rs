use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// The path style of a system.
#[derive(Debug)]
pub enum PathStyle {
    /// DOS-style paths with forward and backward slashes, drive letters and UNC.
    ///
    /// Known from DOS, OS/2 and Windows.
    DOS,
    /// UNIX-style paths with forward slashes only.
    UNIX,
    /// The path style is not known, most likely because the Guest Additions aren't active yet.
    Unknown,
}

impl From<u32> for PathStyle {
    fn from(value: u32) -> Self {
        match value {
            raw::PathStyle_PathStyle_DOS => PathStyle::DOS,
            raw::PathStyle_PathStyle_UNIX => PathStyle::UNIX,
            raw::PathStyle_PathStyle_Unknown => PathStyle::Unknown,
            _ => {
                error!("Unknown PathStyle. PathStyle: {}", value);
                PathStyle::Unknown
            }
        }
    }
}

impl Into<u32> for PathStyle {
    fn into(self) -> u32 {
        match self {
            PathStyle::DOS => raw::PathStyle_PathStyle_DOS,
            PathStyle::UNIX => raw::PathStyle_PathStyle_UNIX,
            PathStyle::Unknown => raw::PathStyle_PathStyle_Unknown,
        }
    }
}

impl Display for PathStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
