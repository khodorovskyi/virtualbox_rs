pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IMouse;

/// The IMouse interface represents the virtual machine's mouse.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_mouse.html](https://www.virtualbox.org/sdkref/interface_i_mouse.html)

#[derive(Debug, Clone)]
pub struct Mouse {
    object: *mut IMouse,
}

impl Mouse {
    pub(crate) fn new(object: *mut IMouse) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Mouse {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Mouse refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Mouse. Error: {:?}", err)
            }
        }
    }
}

unsafe impl Send for Mouse {}
unsafe impl Sync for Mouse {}
