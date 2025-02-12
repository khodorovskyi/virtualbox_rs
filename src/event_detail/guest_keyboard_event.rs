use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IGuestKeyboardEvent, IGUESTKEYBOARDEVENT_IID_STR};

/// Notification when guest keyboard event happens.
#[derive(Debug)]
pub struct GuestKeyboardEvent {
    /// Array of scancodes.
    pub scancodes: Vec<i32>,
}

impl GuestKeyboardEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestKeyboardEvent(detail),
            Err(err) => {
                error!("GuestKeyboardEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTKEYBOARDEVENT_IID_STR)?;
        let scancodes = Self::get_scancodes(obj1)?;
        Ok(Self { scancodes })
    }

    fn get_scancodes(object: *mut IGuestKeyboardEvent) -> Result<Vec<i32>, VboxError> {
        let mut count = 0;
        let scancodes_ptr =
            get_function_result_pointer!(object, GetScancodes, *mut i32, &mut count)?;
        let scancodes =
            unsafe { Vec::from_raw_parts(scancodes_ptr, count as usize, count as usize) };

        Ok(scancodes)
    }
}

impl Display for GuestKeyboardEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
