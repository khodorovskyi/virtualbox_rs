use crate::enums::FileStatus;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{GuestFile, GuestSession, VboxError, VirtualBoxErrorInfo};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestFile, IGuestFileStateChangedEvent, IGuestSession, IVirtualBoxErrorInfo,
    IGUESTFILESTATECHANGEDEVENT_IID_STR,
};

/// Notification when a guest file changed its state.
#[derive(Debug)]
pub struct GuestFileStateChangedEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest file object which is related to this event.
    pub file: GuestFile,
    /// New guest file status. [`FileStatus`]
    pub status: FileStatus,
    /// Error information in case of new session status is indicating an error. [`VirtualBoxErrorInfo`]
    pub error: VirtualBoxErrorInfo,
}

impl GuestFileStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestFileStateChangedEvent(detail),
            Err(err) => {
                error!("GuestFileStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTFILESTATECHANGEDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let file = Self::get_file(obj1)?;
        let status = Self::get_status(obj1)?;
        let error = Self::get_error(obj1)?;
        Ok(Self {
            session,
            file,
            status,
            error,
        })
    }

    fn get_session(object: *mut IGuestFileStateChangedEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_file(object: *mut IGuestFileStateChangedEvent) -> Result<GuestFile, VboxError> {
        let file = get_function_result_pointer!(object, GetFile, *mut IGuestFile)?;
        Ok(GuestFile::new(file))
    }
    fn get_status(object: *mut IGuestFileStateChangedEvent) -> Result<FileStatus, VboxError> {
        let status = get_function_result_number!(object, GetStatus, u32)?;
        Ok(FileStatus::from(status))
    }
    fn get_error(
        object: *mut IGuestFileStateChangedEvent,
    ) -> Result<VirtualBoxErrorInfo, VboxError> {
        let error = get_function_result_pointer!(object, GetError, *mut IVirtualBoxErrorInfo)?;
        Ok(VirtualBoxErrorInfo::new(error))
    }
}

impl Display for GuestFileStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
