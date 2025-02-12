#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{IMachineGroupsChangedEvent, IMACHINEGROUPSCHANGEDEVENT_IID_STR};

/// The groups of the machine have changed.
#[derive(Debug)]
pub struct MachineGroupsChangedEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// Dummy state because you can't have an event without attributes apparently.
    pub dummy: bool,
}

#[cfg(not(is_v_6_1))]
impl MachineGroupsChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MachineGroupsChangedEvent(detail),
            Err(err) => {
                error!("MachineGroupsChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMACHINEGROUPSCHANGEDEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let dummy = Self::get_dummy(obj1)?;
        Ok(Self { machine_id, dummy })
    }

    fn get_machine_id(new_obj: *mut IMachineGroupsChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetMachineId)
    }
    fn get_dummy(new_obj: *mut IMachineGroupsChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetDummy)
    }
}
#[cfg(is_v_6_1)]
impl MachineGroupsChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for MachineGroupsChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
