use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestMonitorInfoChangedEvent, IGUESTMONITORINFOCHANGEDEVENT_IID_STR,
};

/// The guest reports cursor position data.
#[derive(Debug)]
pub struct GuestMonitorInfoChangedEvent {
    /// The virtual display output on which the monitor has changed.
    pub output: u32,
}

impl GuestMonitorInfoChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestMonitorInfoChangedEvent(detail),
            Err(err) => {
                error!("GuestMonitorInfoChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTMONITORINFOCHANGEDEVENT_IID_STR)?;
        let output = Self::get_output(obj1)?;
        Ok(Self { output })
    }

    fn get_output(object: *mut IGuestMonitorInfoChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetOutput, u32)
    }
}

impl Display for GuestMonitorInfoChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
