use crate::enums::UpdateState;
#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::UpdateAgent;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{
    IUpdateAgent, IUpdateAgentStateChangedEvent, IUPDATEAGENTSTATECHANGEDEVENT_IID_STR,
};

/// Notification when an update agent state has been changed.
#[derive(Debug)]
pub struct UpdateAgentStateChangedEvent {
    /// Update agent this event belongs to.
    pub agent: UpdateAgent,
    /// New update agent state. [`UpdateState`]
    pub state: UpdateState,
}

#[cfg(not(is_v_6_1))]
impl UpdateAgentStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::UpdateAgentStateChangedEvent(detail),
            Err(err) => {
                error!("UpdateAgentStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IUPDATEAGENTSTATECHANGEDEVENT_IID_STR)?;
        let agent = Self::get_agent(obj1)?;
        let state = Self::get_state(obj1)?;
        Ok(Self { agent, state })
    }

    fn get_agent(new_obj: *mut IUpdateAgentStateChangedEvent) -> Result<UpdateAgent, VboxError> {
        let agent = get_function_result_pointer!(new_obj, GetAgent, *mut IUpdateAgent)?;
        Ok(UpdateAgent::new(agent))
    }
    fn get_state(new_obj: *mut IUpdateAgentStateChangedEvent) -> Result<UpdateState, VboxError> {
        let state = get_function_result_number!(new_obj, GetState, u32)?;
        Ok(UpdateState::from(state))
    }
}
#[cfg(is_v_6_1)]
impl UpdateAgentStateChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for UpdateAgentStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
