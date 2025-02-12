use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::{Medium, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IMedium, IMediumConfigChangedEvent, IMEDIUMCONFIGCHANGEDEVENT_IID_STR,
};

/// The configuration of the given medium was changed (location, properties, child/parent or anything else).
#[derive(Debug)]
pub struct MediumConfigChangedEvent {
    /// ID of the medium this event relates to.
    pub medium: Medium,
}

impl MediumConfigChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MediumConfigChangedEvent(detail),
            Err(err) => {
                error!("MediumConfigChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMEDIUMCONFIGCHANGEDEVENT_IID_STR)?;
        let medium = Self::get_medium(obj1)?;
        Ok(Self { medium })
    }

    fn get_medium(new_obj: *mut IMediumConfigChangedEvent) -> Result<Medium, VboxError> {
        let medium = get_function_result_pointer!(new_obj, GetMedium, *mut IMedium)?;
        Ok(Medium::new(medium))
    }
}

impl Display for MediumConfigChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
