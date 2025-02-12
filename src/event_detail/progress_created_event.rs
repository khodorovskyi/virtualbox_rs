#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{IProgressCreatedEvent, IPROGRESSCREATEDEVENT_IID_STR};

/// Notification of a new progress object creation/deletion.
#[derive(Debug)]
pub struct ProgressCreatedEvent {
    /// GUID of the progress this event relates to.
    pub progress_id: &'static str,
    /// If true, the progress object was created, otherwise it was deleted.
    pub create: bool,
}

#[cfg(not(is_v_6_1))]
impl ProgressCreatedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ProgressCreatedEvent(detail),
            Err(err) => {
                error!("ProgressCreatedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IPROGRESSCREATEDEVENT_IID_STR)?;
        let progress_id = Self::get_progress_id(obj1)?;
        let create = Self::get_create(obj1)?;
        Ok(Self {
            progress_id,
            create,
        })
    }

    fn get_progress_id(new_obj: *mut IProgressCreatedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetProgressId)
    }
    fn get_create(new_obj: *mut IProgressCreatedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetCreate)
    }
}

#[cfg(is_v_6_1)]
impl ProgressCreatedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}

impl Display for ProgressCreatedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
