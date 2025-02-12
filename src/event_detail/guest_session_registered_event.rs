use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_pointer};
use crate::{GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestSession, IGuestSessionRegisteredEvent, IGUESTSESSIONREGISTEREDEVENT_IID_STR,
};

/// Notification when a guest session was registered or unregistered.
#[derive(Debug)]
pub struct GuestSessionRegisteredEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// If true, the guest session was registered, otherwise it was unregistered.
    pub registered: bool,
}

impl GuestSessionRegisteredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestSessionRegisteredEvent(detail),
            Err(err) => {
                error!("GuestSessionRegisteredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTSESSIONREGISTEREDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let registered = Self::get_registered(obj1)?;
        Ok(Self {
            session,
            registered,
        })
    }

    fn get_session(object: *mut IGuestSessionRegisteredEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }

    fn get_registered(object: *mut IGuestSessionRegisteredEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRegistered)
    }
}

impl Display for GuestSessionRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
