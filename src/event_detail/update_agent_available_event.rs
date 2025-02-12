use crate::enums::{UpdateChannel, UpdateSeverity};
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
use vbox_raw::sys_lib::{
    IUpdateAgent, IUpdateAgentAvailableEvent, IUPDATEAGENTAVAILABLEEVENT_IID_STR,
};

/// Notification when an update is available.
#[derive(Debug)]
pub struct UpdateAgentAvailableEvent {
    /// Update agent this event belongs to.
    pub agent: UpdateAgent,
    /// Version of the update.
    pub version: &'static str,
    /// Channel containing the update. [`UpdateChannel`]
    pub channel: UpdateChannel,
    /// Severity of the update. [`UpdateSeverity`]
    pub severity: UpdateSeverity,
    /// Download URL of the update.
    pub download_url: &'static str,
    /// Web URL of the update.
    pub web_url: &'static str,
    /// Release notes of the update.
    pub release_notes: &'static str,
}

#[cfg(not(is_v_6_1))]
impl UpdateAgentAvailableEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::UpdateAgentAvailableEvent(detail),
            Err(err) => {
                error!("UpdateAgentAvailableEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IUPDATEAGENTAVAILABLEEVENT_IID_STR)?;
        let agent = Self::get_agent(obj1)?;
        let version = Self::get_version(obj1)?;
        let channel = Self::get_channel(obj1)?;
        let severity = Self::get_severity(obj1)?;
        let download_url = Self::get_download_url(obj1)?;
        let web_url = Self::get_web_url(obj1)?;
        let release_notes = Self::get_release_notes(obj1)?;
        Ok(Self {
            agent,
            version,
            channel,
            severity,
            download_url,
            web_url,
            release_notes,
        })
    }

    fn get_agent(new_obj: *mut IUpdateAgentAvailableEvent) -> Result<UpdateAgent, VboxError> {
        let agent = get_function_result_pointer!(new_obj, GetAgent, *mut IUpdateAgent)?;
        Ok(UpdateAgent::new(agent))
    }
    fn get_version(new_obj: *mut IUpdateAgentAvailableEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetVersion)
    }
    fn get_channel(new_obj: *mut IUpdateAgentAvailableEvent) -> Result<UpdateChannel, VboxError> {
        let channel = get_function_result_number!(new_obj, GetChannel, u32)?;
        Ok(UpdateChannel::from(channel))
    }
    fn get_severity(new_obj: *mut IUpdateAgentAvailableEvent) -> Result<UpdateSeverity, VboxError> {
        let severity = get_function_result_number!(new_obj, GetSeverity, u32)?;
        Ok(UpdateSeverity::from(severity))
    }
    fn get_download_url(
        new_obj: *mut IUpdateAgentAvailableEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetDownloadURL)
    }
    fn get_web_url(new_obj: *mut IUpdateAgentAvailableEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetWebURL)
    }
    fn get_release_notes(
        new_obj: *mut IUpdateAgentAvailableEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetReleaseNotes)
    }
}
#[cfg(is_v_6_1)]
impl UpdateAgentAvailableEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for UpdateAgentAvailableEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
