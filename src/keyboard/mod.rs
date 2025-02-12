pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IKeyboard;

/// The Keyboard interface represents the virtual machine's keyboard.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_keyboard.html](https://www.virtualbox.org/sdkref/interface_i_keyboard.html)

#[derive(Debug, Clone)]
pub struct Keyboard {
    object: *mut IKeyboard,
}

impl Keyboard {
    pub(crate) fn new(object: *mut IKeyboard) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Keyboard refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Keyboard. Error: {:?}", err)
            }
        }
    }
}

unsafe impl Send for Keyboard {}
unsafe impl Sync for Keyboard {}
