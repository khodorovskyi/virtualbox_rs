use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IGuestOSType;

/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_guest_o_s_type.html](https://www.virtualbox.org/sdkref/interface_i_guest_o_s_type.html)
#[derive(Debug)]
pub struct GuestOSType {
    object: *mut IGuestOSType,
}

impl GuestOSType {
    pub(crate) fn new(object: *mut IGuestOSType) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for GuestOSType {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("GuestOSType refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop GuestOSType. Error: {:?}", err)
            }
        }
    }
}
