use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IProgressPercentageChangedEvent, IPROGRESSPERCENTAGECHANGEDEVENT_IID_STR,
};

/// Progress state change event.
#[derive(Debug)]
pub struct ProgressPercentageChangedEvent {
    /// GUID of the progress this event relates to.
    pub progress_id: &'static str,
    /// New percent.
    pub percent: i32,
}

impl ProgressPercentageChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ProgressPercentageChangedEvent(detail),
            Err(err) => {
                error!("ProgressPercentageChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IPROGRESSPERCENTAGECHANGEDEVENT_IID_STR)?;
        let progress_id = Self::get_progress_id(obj1)?;
        let percent = Self::get_percent(obj1)?;
        Ok(Self {
            progress_id,
            percent,
        })
    }

    fn get_progress_id(
        new_obj: *mut IProgressPercentageChangedEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetProgressId)
    }
    fn get_percent(new_obj: *mut IProgressPercentageChangedEvent) -> Result<i32, VboxError> {
        get_function_result_number!(new_obj, GetPercent, i32)
    }
}

impl Display for ProgressPercentageChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
