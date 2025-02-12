use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_pointer};
use crate::{GuestFile, GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestFile, IGuestFileRegisteredEvent, IGuestSession, IGUESTFILEREGISTEREDEVENT_IID_STR,
};

/// Notification when a guest process was registered or unregistered.
#[derive(Debug)]
pub struct GuestFileRegisteredEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest file object which is related to this event.
    pub file: GuestFile,
    /// If true, the guest file was registered, otherwise it was unregistered.
    pub registered: bool,
}

impl GuestFileRegisteredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestFileRegisteredEvent(detail),
            Err(err) => {
                error!("GuestFileRegisteredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTFILEREGISTEREDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let file = Self::get_file(obj1)?;
        let registered = Self::get_registered(obj1)?;
        Ok(Self {
            session,
            file,
            registered,
        })
    }

    fn get_session(object: *mut IGuestFileRegisteredEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_file(object: *mut IGuestFileRegisteredEvent) -> Result<GuestFile, VboxError> {
        let file = get_function_result_pointer!(object, GetFile, *mut IGuestFile)?;
        Ok(GuestFile::new(file))
    }
    fn get_registered(object: *mut IGuestFileRegisteredEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRegistered)
    }
}

impl Display for GuestFileRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
