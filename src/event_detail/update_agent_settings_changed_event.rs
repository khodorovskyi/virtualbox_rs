#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::{get_function_result_pointer, get_function_result_str};
use crate::UpdateAgent;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{
    IUpdateAgent, IUpdateAgentSettingsChangedEvent, IUPDATEAGENTSETTINGSCHANGEDEVENT_IID_STR,
};

/// Notification when update agent settings have been changed.
#[derive(Debug)]
pub struct UpdateAgentSettingsChangedEvent {
    /// Update agent this event belongs to.
    pub agent: UpdateAgent,
    pub attribute_hint: &'static str,
}

#[cfg(not(is_v_6_1))]
impl UpdateAgentSettingsChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::UpdateAgentSettingsChangedEvent(detail),
            Err(err) => {
                error!("UpdateAgentSettingsChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IUPDATEAGENTSETTINGSCHANGEDEVENT_IID_STR)?;
        let agent = Self::get_agent(obj1)?;
        let attribute_hint = Self::get_attribute_hint(obj1)?;
        Ok(Self {
            agent,
            attribute_hint,
        })
    }

    fn get_agent(new_obj: *mut IUpdateAgentSettingsChangedEvent) -> Result<UpdateAgent, VboxError> {
        let agent = get_function_result_pointer!(new_obj, GetAgent, *mut IUpdateAgent)?;
        Ok(UpdateAgent::new(agent))
    }
    fn get_attribute_hint(
        new_obj: *mut IUpdateAgentSettingsChangedEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetAttributeHint)
    }
}
#[cfg(is_v_6_1)]
impl UpdateAgentSettingsChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for UpdateAgentSettingsChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
