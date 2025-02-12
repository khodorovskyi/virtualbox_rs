use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IUnattended;

/// Represents a platform-independent Unattended in OVF format.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_unattended.html](https://www.virtualbox.org/sdkref/interface_i_unattended.html)
#[derive(Debug)]
pub struct Unattended {
    object: *mut IUnattended,
}

impl Unattended {
    pub(crate) fn new(object: *mut IUnattended) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Unattended {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Unattended refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Unattended. Error: {:?}", err)
            }
        }
    }
}
