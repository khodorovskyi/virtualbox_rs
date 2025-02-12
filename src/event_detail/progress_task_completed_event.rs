use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IProgressTaskCompletedEvent, IPROGRESSTASKCOMPLETEDEVENT_IID_STR};

/// Progress state change event.
#[derive(Debug)]
pub struct ProgressTaskCompletedEvent {
    /// GUID of the progress this event relates to.
    pub progress_id: &'static str,
    pub midl_does_not_like_empty_interfaces: bool,
}

impl ProgressTaskCompletedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ProgressTaskCompletedEvent(detail),
            Err(err) => {
                error!("ProgressTaskCompletedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IPROGRESSTASKCOMPLETEDEVENT_IID_STR)?;
        let progress_id = Self::get_progress_id(obj1)?;
        let midl_does_not_like_empty_interfaces =
            Self::get_midl_does_not_like_empty_interfaces(obj1)?;
        Ok(Self {
            progress_id,
            midl_does_not_like_empty_interfaces,
        })
    }

    fn get_progress_id(
        new_obj: *mut IProgressTaskCompletedEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetProgressId)
    }

    fn get_midl_does_not_like_empty_interfaces(
        new_obj: *mut IProgressTaskCompletedEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetMidlDoesNotLikeEmptyInterfaces)
    }
}

impl Display for ProgressTaskCompletedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
