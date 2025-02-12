use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_bool;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IClipboardFileTransferModeChangedEvent, IEvent, ICLIPBOARDFILETRANSFERMODECHANGEDEVENT_IID_STR,
};

/// Notification when the shared clipboard file transfer mode changes.
#[derive(Debug)]
pub struct ClipboardFileTransferModeChangedEvent {
    /// TWhether file transfers are allowed or not.
    pub enabled: bool,
}

impl ClipboardFileTransferModeChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ClipboardFileTransferModeChangedEvent(detail),
            Err(err) => {
                error!("ClipboardFileTransferModeChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICLIPBOARDFILETRANSFERMODECHANGEDEVENT_IID_STR)?;
        let enabled = Self::get_enabled(obj1)?;
        Ok(Self { enabled })
    }

    fn get_enabled(object: *mut IClipboardFileTransferModeChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetEnabled)
    }
}

impl Display for ClipboardFileTransferModeChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
