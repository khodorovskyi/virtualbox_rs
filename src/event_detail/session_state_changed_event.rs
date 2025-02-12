use crate::enums::SessionState;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, ISessionStateChangedEvent, ISESSIONSTATECHANGEDEVENT_IID_STR};

/// The state of the session for the given machine was changed
#[derive(Debug)]
pub struct SessionStateChangedEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// New session state [`SessionState`].
    pub state: SessionState,
}

impl SessionStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::SessionStateChangedEvent(detail),
            Err(err) => {
                error!("SessionStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ISESSIONSTATECHANGEDEVENT_IID_STR)?;
        let state = Self::get_state(obj1)?;
        let machine_id = Self::get_machine_id(obj1)?;
        Ok(Self { machine_id, state })
    }

    fn get_machine_id(object: *mut ISessionStateChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }

    fn get_state(new_obj: *mut ISessionStateChangedEvent) -> Result<SessionState, VboxError> {
        let state = get_function_result_number!(new_obj, GetState, u32)?;
        Ok(SessionState::from(state))
    }
}

impl Display for SessionStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
