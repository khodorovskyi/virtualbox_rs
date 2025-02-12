#[cfg(doc)]
use crate::Machine;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Cleanup mode, used with [`Machine::unregister`].
#[derive(Debug, Eq, PartialEq)]
pub enum CleanupMode {
    /// Unregister only the machine, but neither delete snapshots nor detach media.
    UnregisterOnly,
    /// Delete all snapshots and detach all media but return none; this will keep all media registered.
    DetachAllReturnNone,
    /// Delete all snapshots, detach all media and return hard disks for closing, but not removable media.
    DetachAllReturnHardDisksOnly,
    /// Delete all snapshots, detach all media and return all media for closing.
    Full,
    /// Delete all snapshots, detach all media and return hard disks and removable media located in the folder of the machine to be deleted and referenced only by this machine, for closing.
    DetachAllReturnHardDisksAndVMRemovable,
}
impl Into<u32> for CleanupMode {
    fn into(self) -> u32 {
        match self {
            CleanupMode::DetachAllReturnNone => raw::CleanupMode_CleanupMode_DetachAllReturnNone,
            CleanupMode::DetachAllReturnHardDisksOnly => {
                raw::CleanupMode_CleanupMode_DetachAllReturnHardDisksOnly
            }
            CleanupMode::Full => raw::CleanupMode_CleanupMode_Full,
            #[cfg(not(is_v_6_1))]
            CleanupMode::DetachAllReturnHardDisksAndVMRemovable => {
                raw::CleanupMode_CleanupMode_DetachAllReturnHardDisksAndVMRemovable
            }
            _ => raw::CleanupMode_CleanupMode_UnregisterOnly,
        }
    }
}

impl From<u32> for CleanupMode {
    fn from(value: u32) -> Self {
        match value {
            raw::CleanupMode_CleanupMode_UnregisterOnly => CleanupMode::UnregisterOnly,
            raw::CleanupMode_CleanupMode_DetachAllReturnNone => CleanupMode::DetachAllReturnNone,
            raw::CleanupMode_CleanupMode_DetachAllReturnHardDisksOnly => {
                CleanupMode::DetachAllReturnHardDisksOnly
            }
            raw::CleanupMode_CleanupMode_Full => CleanupMode::Full,
            #[cfg(not(is_v_6_1))]
            raw::CleanupMode_CleanupMode_DetachAllReturnHardDisksAndVMRemovable => {
                CleanupMode::DetachAllReturnHardDisksAndVMRemovable
            }
            _ => {
                error!("CleanupMode::from. Unknown type: {}", value);
                CleanupMode::UnregisterOnly
            }
        }
    }
}

impl Display for CleanupMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
