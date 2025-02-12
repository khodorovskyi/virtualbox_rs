use crate::enums::ProcessStatus;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{GuestProcess, GuestSession, VboxError, VirtualBoxErrorInfo};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestProcess, IGuestProcessStateChangedEvent, IGuestSession, IVirtualBoxErrorInfo,
    IGUESTPROCESSSTATECHANGEDEVENT_IID_STR,
};

/// Notification when a guest process was registered or unregistered.
#[derive(Debug)]
pub struct GuestProcessStateChangedEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest process object which is related to this event.
    pub process: GuestProcess,
    /// Guest process ID (PID).
    pub pid: u32,
    /// New guest process status. [`ProcessStatus`]
    pub status: ProcessStatus,
    /// Error information in case of new session status is indicating an error.
    pub error: VirtualBoxErrorInfo,
}

impl GuestProcessStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestProcessStateChangedEvent(detail),
            Err(err) => {
                error!("GuestProcessStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTPROCESSSTATECHANGEDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let process = Self::get_process(obj1)?;
        let pid = Self::get_pid(obj1)?;
        let status = Self::get_status(obj1)?;
        let error = Self::get_error(obj1)?;
        Ok(Self {
            session,
            process,
            pid,
            status,
            error,
        })
    }

    fn get_session(object: *mut IGuestProcessStateChangedEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_process(object: *mut IGuestProcessStateChangedEvent) -> Result<GuestProcess, VboxError> {
        let process = get_function_result_pointer!(object, GetProcess, *mut IGuestProcess)?;
        Ok(GuestProcess::new(process))
    }

    fn get_pid(object: *mut IGuestProcessStateChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetPid, u32)
    }
    fn get_status(object: *mut IGuestProcessStateChangedEvent) -> Result<ProcessStatus, VboxError> {
        let status = get_function_result_number!(object, GetStatus, u32)?;
        Ok(ProcessStatus::from(status))
    }
    fn get_error(
        object: *mut IGuestProcessStateChangedEvent,
    ) -> Result<VirtualBoxErrorInfo, VboxError> {
        let error = get_function_result_pointer!(object, GetError, *mut IVirtualBoxErrorInfo)?;
        Ok(VirtualBoxErrorInfo::new(error))
    }
}

impl Display for GuestProcessStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
