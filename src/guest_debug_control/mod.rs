pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IGuestDebugControl;

/// Controls the guest debug settings of one virtual machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_guest_debug_control.html](https://www.virtualbox.org/sdkref/interface_i_guest_debug_control.html)
#[derive(Debug)]
pub struct GuestDebugControl {
    object: *mut IGuestDebugControl,
}

impl GuestDebugControl {
    pub(crate) fn new(object: *mut IGuestDebugControl) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for GuestDebugControl {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("GuestDebugControl refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop GuestDebugControl. Error: {:?}", err)
            }
        }
    }
}
