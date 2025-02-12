use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
};
use crate::{GuestFile, GuestSession, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestFile, IGuestFileOffsetChangedEvent, IGuestSession,
    IGUESTFILEOFFSETCHANGEDEVENT_IID_STR,
};

/// Notification when a guest file changed its current offset via File::seek.
#[derive(Debug)]
pub struct GuestFileOffsetChangedEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Guest file object which is related to this event.
    pub file: GuestFile,
    /// Current offset (in bytes).
    pub offset: i64,
    /// Processed input or output (in bytes).
    pub processed: u32,
    pub midl_does_not_like_empty_interfaces: bool,
}

impl GuestFileOffsetChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestFileOffsetChangedEvent(detail),
            Err(err) => {
                error!("GuestFileOffsetChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTFILEOFFSETCHANGEDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let file = Self::get_file(obj1)?;
        let offset = Self::get_offset(obj1)?;
        let processed = Self::get_processed(obj1)?;
        let midl_does_not_like_empty_interfaces =
            Self::get_midl_does_not_like_empty_interfaces(obj1)?;
        Ok(Self {
            session,
            file,
            offset,
            processed,
            midl_does_not_like_empty_interfaces,
        })
    }

    fn get_session(object: *mut IGuestFileOffsetChangedEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }
    fn get_file(object: *mut IGuestFileOffsetChangedEvent) -> Result<GuestFile, VboxError> {
        let file = get_function_result_pointer!(object, GetFile, *mut IGuestFile)?;
        Ok(GuestFile::new(file))
    }
    fn get_offset(object: *mut IGuestFileOffsetChangedEvent) -> Result<i64, VboxError> {
        get_function_result_number!(object, GetOffset, i64)
    }
    fn get_processed(object: *mut IGuestFileOffsetChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetProcessed, u32)
    }
    fn get_midl_does_not_like_empty_interfaces(
        new_obj: *mut IGuestFileOffsetChangedEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetMidlDoesNotLikeEmptyInterfaces)
    }
}

impl Display for GuestFileOffsetChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
