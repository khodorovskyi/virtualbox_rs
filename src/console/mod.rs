pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IConsole;

/// Interface for controlling the virtual machine console.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_console.html](https://www.virtualbox.org/sdkref/interface_i_console.html)

pub struct Console {
    object: *mut IConsole,
}

impl Console {
    pub(crate) fn new(object: *mut IConsole) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Console refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Console. Error: {:?}", err)
            }
        }
    }
}
