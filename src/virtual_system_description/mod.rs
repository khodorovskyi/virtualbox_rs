use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IVirtualSystemDescription;

/// Represents one virtual system (machine) in an appliance.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_virtual_system_description.html](https://www.virtualbox.org/sdkref/interface_i_virtual_system_description.html)
#[derive(Debug)]
pub struct VirtualSystemDescription {
    object: *mut IVirtualSystemDescription,
}

impl VirtualSystemDescription {
    pub(crate) fn new(object: *mut IVirtualSystemDescription) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VirtualSystemDescription {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("VirtualSystemDescription refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VirtualSystemDescription. Error: {:?}", err)
            }
        }
    }
}
