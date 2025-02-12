use crate::enums::MachineState;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IStateChangedEvent, ISTATECHANGEDEVENT_IID_STR};

/// Notification when the execution state of the machine has changed
#[derive(Debug)]
pub struct StateChangedEvent {
    /// New execution state. [`MachineState`]
    pub state: MachineState,
}

impl StateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::StateChangedEvent(detail),
            Err(err) => {
                error!("StateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let object = cast_event(object, ISTATECHANGEDEVENT_IID_STR)?;

        let state = Self::get_state(object)?;
        Ok(Self { state })
    }

    fn get_state(object: *mut IStateChangedEvent) -> Result<MachineState, VboxError> {
        let state = get_function_result_number!(object, GetState, u32)?;
        Ok(MachineState::from(state))
    }
}

impl Display for StateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
