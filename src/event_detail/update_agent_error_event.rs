#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_pointer, get_function_result_str,
};
use crate::UpdateAgent;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{IUpdateAgent, IUpdateAgentErrorEvent, IUPDATEAGENTERROREVENT_IID_STR};

/// Notification when an update agent error occurred.
#[derive(Debug)]
pub struct UpdateAgentErrorEvent {
    /// Update agent this event belongs to.
    pub agent: UpdateAgent,
    /// Error message in human readable format.
    pub msg: &'static str,
    /// IPRT-style error code.
    pub rc_error: i32,
}

#[cfg(not(is_v_6_1))]
impl UpdateAgentErrorEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::UpdateAgentErrorEvent(detail),
            Err(err) => {
                error!("UpdateAgentErrorEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IUPDATEAGENTERROREVENT_IID_STR)?;
        let agent = Self::get_agent(obj1)?;
        let msg = Self::get_msg(obj1)?;
        let rc_error = Self::get_rc_error(obj1)?;
        Ok(Self {
            agent,
            msg,
            rc_error,
        })
    }

    fn get_agent(new_obj: *mut IUpdateAgentErrorEvent) -> Result<UpdateAgent, VboxError> {
        let agent = get_function_result_pointer!(new_obj, GetAgent, *mut IUpdateAgent)?;
        Ok(UpdateAgent::new(agent))
    }
    fn get_msg(new_obj: *mut IUpdateAgentErrorEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetMsg)
    }
    fn get_rc_error(new_obj: *mut IUpdateAgentErrorEvent) -> Result<i32, VboxError> {
        get_function_result_number!(new_obj, GetRcError, i32)
    }
}
#[cfg(is_v_6_1)]
impl UpdateAgentErrorEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for UpdateAgentErrorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
