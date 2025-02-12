use crate::enums::ProcessInputStatus;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{GuestProcess, GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestProcess, IGuestProcessInputNotifyEvent, IGuestSession,
    IGUESTPROCESSINPUTNOTIFYEVENT_IID_STR,
};

/// Notification when a guest process' stdin became available.
#[derive(Debug)]
pub struct GuestProcessInputNotifyEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest process object which is related to this event.
    pub process: GuestProcess,
    /// Guest process ID (PID).
    pub pid: u32,
    /// Input/output (IO) handle involved in this event.
    pub handle: u32,
    /// Processed input or output (in bytes).
    pub processed: u32,
    /// Current process input status. [`ProcessInputStatus`]
    pub status: ProcessInputStatus,
}

impl GuestProcessInputNotifyEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestProcessInputNotifyEvent(detail),
            Err(err) => {
                error!("GuestProcessInputNotifyEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTPROCESSINPUTNOTIFYEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let process = Self::get_process(obj1)?;
        let pid = Self::get_pid(obj1)?;
        let handle = Self::get_handle(obj1)?;
        let processed = Self::get_processed(obj1)?;
        let status = Self::get_status(obj1)?;
        Ok(Self {
            session,
            process,
            pid,
            handle,
            processed,
            status,
        })
    }

    fn get_session(object: *mut IGuestProcessInputNotifyEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_process(object: *mut IGuestProcessInputNotifyEvent) -> Result<GuestProcess, VboxError> {
        let process = get_function_result_pointer!(object, GetProcess, *mut IGuestProcess)?;
        Ok(GuestProcess::new(process))
    }

    fn get_pid(object: *mut IGuestProcessInputNotifyEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetPid, u32)
    }
    fn get_handle(object: *mut IGuestProcessInputNotifyEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetHandle, u32)
    }
    fn get_processed(object: *mut IGuestProcessInputNotifyEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetProcessed, u32)
    }
    fn get_status(
        object: *mut IGuestProcessInputNotifyEvent,
    ) -> Result<ProcessInputStatus, VboxError> {
        let status = get_function_result_number!(object, GetStatus, u32)?;
        Ok(ProcessInputStatus::from(status))
    }
}

impl Display for GuestProcessInputNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
