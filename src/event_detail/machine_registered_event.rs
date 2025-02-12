use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IMachineRegisteredEvent, IMACHINEREGISTEREDEVENT_IID_STR};

/// The given machine was registered or unregistered within this VirtualBox installation.
#[derive(Debug)]
pub struct MachineRegisteredEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// If true, the machine was registered, otherwise it was unregistered.
    pub registered: bool,
}

impl MachineRegisteredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MachineRegisteredEvent(detail),
            Err(err) => {
                error!("MachineRegisteredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMACHINEREGISTEREDEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let registered = Self::get_registered(obj1)?;
        Ok(Self {
            machine_id,
            registered,
        })
    }

    fn get_machine_id(object: *mut IMachineRegisteredEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }
    fn get_registered(object: *mut IMachineRegisteredEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRegistered)
    }
}

impl Display for MachineRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
