use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IGuestProcess;

/// Implementation of the IProcess object for processes the host has started in the guest.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_guest_process.html](https://www.virtualbox.org/sdkref/interface_i_guest_process.html)
#[derive(Debug)]
pub struct GuestProcess {
    object: *mut IGuestProcess,
}

impl GuestProcess {
    pub(crate) fn new(object: *mut IGuestProcess) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for GuestProcess {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("GuestProcess refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop GuestProcess. Error: {:?}", err)
            }
        }
    }
}
