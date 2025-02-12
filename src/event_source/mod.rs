mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IEventSource;

/// Event source.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_event_source.html](https://www.virtualbox.org/sdkref/interface_i_event_source.html)
#[derive(Debug)]
pub struct EventSource {
    pub(crate) object: *mut IEventSource,
}

impl EventSource {
    pub(crate) fn new(object: *mut IEventSource) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("EventSource refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop EventSource. Error: {:?}", err)
            }
        }
    }
}

unsafe impl Send for EventSource {}
