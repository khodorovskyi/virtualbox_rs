use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
};
use crate::{GuestProcess, GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestProcess, IGuestProcessRegisteredEvent, IGuestSession,
    IGUESTPROCESSREGISTEREDEVENT_IID_STR,
};

/// Notification when a guest process was registered or unregistered.
#[derive(Debug)]
pub struct GuestProcessRegisteredEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest process object which is related to this event.
    pub process: GuestProcess,
    /// Guest process ID (PID).
    pub pid: u32,
    /// If true, the guest session was registered, otherwise it was unregistered.
    pub registered: bool,
}

impl GuestProcessRegisteredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestProcessRegisteredEvent(detail),
            Err(err) => {
                error!("GuestProcessRegisteredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTPROCESSREGISTEREDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let process = Self::get_process(obj1)?;
        let pid = Self::get_pid(obj1)?;
        let registered = Self::get_registered(obj1)?;
        Ok(Self {
            session,
            process,
            pid,
            registered,
        })
    }

    fn get_session(object: *mut IGuestProcessRegisteredEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_process(object: *mut IGuestProcessRegisteredEvent) -> Result<GuestProcess, VboxError> {
        let process = get_function_result_pointer!(object, GetProcess, *mut IGuestProcess)?;
        Ok(GuestProcess::new(process))
    }

    fn get_pid(object: *mut IGuestProcessRegisteredEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetPid, u32)
    }
    fn get_registered(object: *mut IGuestProcessRegisteredEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRegistered)
    }
}

impl Display for GuestProcessRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
