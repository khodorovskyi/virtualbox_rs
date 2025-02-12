pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::fmt::Display;
use vbox_raw::sys_lib::IEventListener;

/// Event listener.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_event_listener.html](https://www.virtualbox.org/sdkref/interface_i_event_listener.html)
#[derive(Clone, Debug)]
pub struct EventListener {
    pub(crate) object: *mut IEventListener,
}

impl EventListener {
    pub(crate) fn new(object: *mut IEventListener) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("EventListener refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop EventListener. Error: {:?}", err)
            }
        }
    }
}

impl Display for EventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}", self))
    }
}
