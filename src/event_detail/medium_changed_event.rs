use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::{MediumAttachment, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IMediumAttachment, IMediumChangedEvent, IMEDIUMCHANGEDEVENT_IID_STR,
};

/// Notification when a medium attachment changes.
#[derive(Debug)]
pub struct MediumChangedEvent {
    /// Medium attachment that is subject to change.
    pub medium_attachment: MediumAttachment,
}

impl MediumChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MediumChangedEvent(detail),
            Err(err) => {
                error!("MediumChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMEDIUMCHANGEDEVENT_IID_STR)?;
        let medium_attachment = Self::get_medium_attachment(obj1)?;
        Ok(Self { medium_attachment })
    }

    fn get_medium_attachment(
        new_obj: *mut IMediumChangedEvent,
    ) -> Result<MediumAttachment, VboxError> {
        let medium_attachment =
            get_function_result_pointer!(new_obj, GetMediumAttachment, *mut IMediumAttachment)?;
        Ok(MediumAttachment::new(medium_attachment))
    }
}

impl Display for MediumChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
