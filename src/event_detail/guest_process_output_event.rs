use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{GuestProcess, GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestProcess, IGuestProcessOutputEvent, IGuestSession,
    IGUESTPROCESSOUTPUTEVENT_IID_STR,
};

/// Notification when there is guest process output available for reading.
#[derive(Debug)]
pub struct GuestProcessOutputEvent {
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
    /// Actual output data.
    pub data: Vec<u8>,
}

impl GuestProcessOutputEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestProcessOutputEvent(detail),
            Err(err) => {
                error!("GuestProcessOutputEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTPROCESSOUTPUTEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let process = Self::get_process(obj1)?;
        let pid = Self::get_pid(obj1)?;
        let handle = Self::get_handle(obj1)?;
        let processed = Self::get_processed(obj1)?;
        let data = Self::get_data(obj1)?;
        Ok(Self {
            session,
            process,
            pid,
            handle,
            processed,
            data,
        })
    }

    fn get_session(object: *mut IGuestProcessOutputEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_process(object: *mut IGuestProcessOutputEvent) -> Result<GuestProcess, VboxError> {
        let process = get_function_result_pointer!(object, GetProcess, *mut IGuestProcess)?;
        Ok(GuestProcess::new(process))
    }

    fn get_pid(object: *mut IGuestProcessOutputEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetPid, u32)
    }
    fn get_handle(object: *mut IGuestProcessOutputEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetHandle, u32)
    }
    fn get_processed(object: *mut IGuestProcessOutputEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetProcessed, u32)
    }
    fn get_data(object: *mut IGuestProcessOutputEvent) -> Result<Vec<u8>, VboxError> {
        // get_function_result_number!(object, GetData, u32)
        let mut count = 0;
        let data_ptr = get_function_result_pointer!(object, GetData, *mut u8, &mut count)?;
        let data = unsafe { Vec::from_raw_parts(data_ptr, count as usize, count as usize) };

        Ok(data)
    }
}

impl Display for GuestProcessOutputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
