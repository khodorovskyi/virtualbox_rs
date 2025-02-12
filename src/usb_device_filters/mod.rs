use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IUSBDeviceFilters;

///
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_u_s_b_device_filters.html](https://www.virtualbox.org/sdkref/interface_i_u_s_b_device_filters.html)
#[derive(Debug)]
pub struct USBDeviceFilters {
    object: *mut IUSBDeviceFilters,
}

impl USBDeviceFilters {
    pub(crate) fn new(object: *mut IUSBDeviceFilters) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for USBDeviceFilters {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("USBDeviceFilters refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop USBDeviceFilters. Error: {:?}", err)
            }
        }
    }
}
