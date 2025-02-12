use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IMachineDataChangedEvent, IMACHINEDATACHANGEDEVENT_IID_STR};

/// Any of the settings of the given machine has changed
#[derive(Debug)]
pub struct MachineDataChangedEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// true if the settings change is temporary.
    pub temporary: bool,
}

impl MachineDataChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MachineDataChangedEvent(detail),
            Err(err) => {
                error!("MachineDataChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMACHINEDATACHANGEDEVENT_IID_STR)?;
        let temporary = Self::get_state(obj1)?;
        let machine_id = Self::get_machine_id(obj1)?;
        Ok(Self {
            machine_id,
            temporary,
        })
    }

    fn get_machine_id(object: *mut IMachineDataChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }

    fn get_state(new_obj: *mut IMachineDataChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetTemporary)
    }
}

impl Display for MachineDataChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
