
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

#[cfg(doc)]
use crate::Medium;

/// Virtual medium state.
#[derive(Debug, Eq, PartialEq)]
pub enum MediumState {
    /// Associated medium storage does not exist (either was not created yet or was deleted).
    NotCreated,
    /// Associated storage exists and accessible; this gets set if the accessibility check performed by [`Medium::refresh_state`] was successful.
    Created,
    /// Medium is locked for reading (see [`Medium::lock_read`]), no data modification is possible.
    LockedRead,
    /// Medium is locked for writing (see [`Medium::lock_write`]), no concurrent data reading or modification is possible.
    LockedWrite,
    /// Medium accessibility check (see [`Medium::refresh_state`]) has not yet been performed, or else, associated medium storage is not accessible.
    ///
    /// In the first case, [`Medium::get_last_access_error`] is empty, in the second case, it describes the error that occurred.
    Inaccessible,
    /// Associated medium storage is being created.
    Creating,
    /// Associated medium storage is being deleted.
    Deleting,

}
impl Into<u32> for MediumState {
    fn into(self) -> u32 {
        match self {
            MediumState::NotCreated => raw::MediumState_MediumState_NotCreated,
            MediumState::Created => raw::MediumState_MediumState_Created,
            MediumState::LockedRead => raw::MediumState_MediumState_LockedRead,
            MediumState::LockedWrite => raw::MediumState_MediumState_LockedWrite,
            MediumState::Inaccessible => raw::MediumState_MediumState_Inaccessible,
            MediumState::Creating => raw::MediumState_MediumState_Creating,
            MediumState::Deleting => raw::MediumState_MediumState_Deleting,
        }
    }
}

impl From<u32> for MediumState {
    fn from(value: u32) -> Self {
        match value {
            raw::MediumState_MediumState_NotCreated => MediumState::NotCreated,
            raw::MediumState_MediumState_Created => MediumState::Created,
            raw::MediumState_MediumState_LockedRead => MediumState::LockedRead,
            raw::MediumState_MediumState_LockedWrite => MediumState::LockedWrite,
            raw::MediumState_MediumState_Inaccessible => MediumState::Inaccessible,
            raw::MediumState_MediumState_Creating => MediumState::Creating,
            raw::MediumState_MediumState_Deleting => MediumState::Deleting,
            _ => {
                error!("MediumState::from. Unknown type: {}", value);
                MediumState::NotCreated
            }
        }
    }
}

impl Display for MediumState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
