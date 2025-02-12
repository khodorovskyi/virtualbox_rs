use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{GuestFile, GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestFile, IGuestFileSizeChangedEvent, IGuestSession,
    IGUESTFILESIZECHANGEDEVENT_IID_STR,
};

/// Notification when a guest file changed its size via File::setSize.
#[derive(Debug)]
pub struct GuestFileSizeChangedEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest file object which is related to this event.
    pub file: GuestFile,
    ///New file size (in bytes).
    pub new_size: i64,
}

impl GuestFileSizeChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestFileSizeChangedEvent(detail),
            Err(err) => {
                error!("GuestFileSizeChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTFILESIZECHANGEDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let file = Self::get_file(obj1)?;
        let new_size = Self::get_new_size(obj1)?;
        Ok(Self {
            session,
            file,
            new_size,
        })
    }

    fn get_session(object: *mut IGuestFileSizeChangedEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_file(object: *mut IGuestFileSizeChangedEvent) -> Result<GuestFile, VboxError> {
        let file = get_function_result_pointer!(object, GetFile, *mut IGuestFile)?;
        Ok(GuestFile::new(file))
    }
    fn get_new_size(object: *mut IGuestFileSizeChangedEvent) -> Result<i64, VboxError> {
        get_function_result_number!(object, GetNewSize, i64)
    }
}

impl Display for GuestFileSizeChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
