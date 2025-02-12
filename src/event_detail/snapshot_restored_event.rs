use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, ISnapshotRestoredEvent, ISNAPSHOTRESTOREDEVENT_IID_STR};

/// Snapshot of the given machine has been restored
#[derive(Debug)]
pub struct SnapshotRestoredEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// ID of the snapshot this event relates to.
    pub snapshot_id: &'static str,
    pub midl_does_not_like_empty_interfaces: bool,
}

impl SnapshotRestoredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::SnapshotRestoredEvent(detail),
            Err(err) => {
                error!("SnapshotRestoredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ISNAPSHOTRESTOREDEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let snapshot_id = Self::get_snapshot_id(obj1)?;
        let midl_does_not_like_empty_interfaces =
            Self::get_midl_does_not_like_empty_interfaces(obj1)?;
        Ok(Self {
            machine_id,
            snapshot_id,
            midl_does_not_like_empty_interfaces,
        })
    }

    fn get_midl_does_not_like_empty_interfaces(
        new_obj: *mut ISnapshotRestoredEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetMidlDoesNotLikeEmptyInterfaces)
    }

    fn get_machine_id(object: *mut ISnapshotRestoredEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }
    fn get_snapshot_id(object: *mut ISnapshotRestoredEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetSnapshotId)
    }
}

impl Display for SnapshotRestoredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
