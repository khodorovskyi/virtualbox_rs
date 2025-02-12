pub mod implementation;

use crate::enums::VBoxEventType;
use crate::event_detail::{create_event_detail, DetailEvent};
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, trace};
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;

/// Abstract parent interface for VirtualBox events
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_event.html](https://www.virtualbox.org/sdkref/interface_i_event.html)

#[derive(Debug)]
pub struct Event {
    pub(crate) object: *mut IEvent,
}

impl Event {
    pub(crate) fn new(object: *mut IEvent) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }

    pub fn get_event_detail(&self) -> DetailEvent {
        create_event_detail(&self)
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Event refcount: {}", count)
            }
            Err(err) => {
                trace!("Failed drop Event. Error: {:?}", err)
            }
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let event_type = self.get_type().unwrap_or(VBoxEventType::Invalid);
        let event_detail = self.get_event_detail();
        write!(
            f,
            "{}",
            format!("EventType: {event_type}, DetailEvent: {event_detail}")
        )
    }
}
