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

#[derive(Debug)]
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
    pub(crate) fn add_ref(&self) -> Result<i32, VboxError> {
        call_function!(self.object, AddRef)
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
impl Clone for Mouse {
    fn clone(&self) -> Self {
        let clone_object = Self::new(self.object);
        let _ = clone_object.add_ref();
        clone_object
    }
}

unsafe impl Send for Mouse {}
unsafe impl Sync for Mouse {}
