use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{GuestFile, GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestFile, IGuestFileReadEvent, IGuestSession, IGUESTFILEREADEVENT_IID_STR,
};

/// Notification when data has been read from a guest file
#[derive(Debug)]
pub struct GuestFileReadEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest file object which is related to this event.
    pub file: GuestFile,
    /// Current offset (in bytes).
    pub offset: i64,
    /// Processed input or output (in bytes).
    pub processed: u32,
    /// Actual data read.
    pub data: Vec<u8>,
}

impl GuestFileReadEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestFileReadEvent(detail),
            Err(err) => {
                error!("GuestFileReadEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTFILEREADEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let file = Self::get_file(obj1)?;
        let offset = Self::get_offset(obj1)?;
        let processed = Self::get_processed(obj1)?;
        let data = Self::get_data(obj1)?;
        Ok(Self {
            session,
            file,
            offset,
            processed,
            data,
        })
    }

    fn get_session(object: *mut IGuestFileReadEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_file(object: *mut IGuestFileReadEvent) -> Result<GuestFile, VboxError> {
        let file = get_function_result_pointer!(object, GetFile, *mut IGuestFile)?;
        Ok(GuestFile::new(file))
    }
    fn get_offset(object: *mut IGuestFileReadEvent) -> Result<i64, VboxError> {
        get_function_result_number!(object, GetOffset, i64)
    }
    fn get_processed(object: *mut IGuestFileReadEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetProcessed, u32)
    }

    fn get_data(object: *mut IGuestFileReadEvent) -> Result<Vec<u8>, VboxError> {
        let mut count = 0;
        let data_ptr = get_function_result_pointer!(object, GetData, *mut u8, &mut count)?;
        let data = unsafe { Vec::from_raw_parts(data_ptr, count as usize, count as usize) };

        Ok(data)
    }
}

impl Display for GuestFileReadEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
