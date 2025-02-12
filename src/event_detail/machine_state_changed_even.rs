use crate::enums::MachineState;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IMachineStateChangedEvent, IMACHINESTATECHANGEDEVENT_IID_STR};

/// Machine state change event
#[derive(Debug)]
pub struct MachineStateChangedEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// New execution state. [`MachineState`]
    pub state: MachineState,
}

impl MachineStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MachineStateChangedEvent(detail),
            Err(err) => {
                error!("MachineStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let object = cast_event(object, IMACHINESTATECHANGEDEVENT_IID_STR)?;

        let state = Self::get_state(object)?;
        let machine_id = Self::get_machine_id(object)?;
        Ok(Self { machine_id, state })
    }

    fn get_machine_id(object: *mut IMachineStateChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }

    fn get_state(object: *mut IMachineStateChangedEvent) -> Result<MachineState, VboxError> {
        let state = get_function_result_number!(object, GetState, u32)?;
        Ok(MachineState::from(state))
    }
}

impl Display for MachineStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
