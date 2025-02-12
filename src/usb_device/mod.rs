use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IUSBDevice;

/// Virtual USB device attached to the virtual machine
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_u_s_b_device.html](https://www.virtualbox.org/sdkref/interface_i_u_s_b_device.html)
#[derive(Debug)]
pub struct USBDevice {
    object: *mut IUSBDevice,
}

impl USBDevice {
    pub(crate) fn new(object: *mut IUSBDevice) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for USBDevice {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("USBDevice refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop USBDevice. Error: {:?}", err)
            }
        }
    }
}
