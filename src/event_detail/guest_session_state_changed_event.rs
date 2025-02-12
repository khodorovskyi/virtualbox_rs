use crate::enums::GuestSessionStatus;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{GuestSession, VboxError, VirtualBoxErrorInfo};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestSession, IGuestSessionStateChangedEvent, IVirtualBoxErrorInfo,
    IGUESTSESSIONSTATECHANGEDEVENT_IID_STR,
};

/// Notification when a guest session changed its state
#[derive(Debug)]
pub struct GuestSessionStateChangedEvent {
    /// Guest session that is subject to change.
    pub session: GuestSession,
    /// Session ID of guest session which was changed.
    pub id: u32,
    /// New session status. [`GuestSessionStatus`]
    pub status: GuestSessionStatus,
    /// Error information in case of new session status is indicating an error [`VirtualBoxErrorInfo`].
    pub error: VirtualBoxErrorInfo,
}

impl GuestSessionStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestSessionStateChangedEvent(detail),
            Err(err) => {
                error!("GuestSessionStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTSESSIONSTATECHANGEDEVENT_IID_STR)?;
        let session = Self::get_session(obj1)?;
        let id = Self::get_id(obj1)?;
        let status = Self::get_status(obj1)?;
        let error = Self::get_error(obj1)?;
        Ok(Self {
            session,
            id,
            status,
            error,
        })
    }

    fn get_session(object: *mut IGuestSessionStateChangedEvent) -> Result<GuestSession, VboxError> {
        let session = get_function_result_pointer!(object, GetSession, *mut IGuestSession)?;
        Ok(GuestSession::new(session))
    }

    fn get_id(object: *mut IGuestSessionStateChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetId, u32)
    }
    fn get_status(
        object: *mut IGuestSessionStateChangedEvent,
    ) -> Result<GuestSessionStatus, VboxError> {
        let status = get_function_result_number!(object, GetStatus, u32)?;
        Ok(GuestSessionStatus::from(status))
    }
    fn get_error(
        object: *mut IGuestSessionStateChangedEvent,
    ) -> Result<VirtualBoxErrorInfo, VboxError> {
        let error = get_function_result_pointer!(object, GetError, *mut IVirtualBoxErrorInfo)?;
        Ok(VirtualBoxErrorInfo::new(error))
    }
}

impl Display for GuestSessionStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
