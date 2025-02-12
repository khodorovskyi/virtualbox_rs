
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IPlatformARM;

/// The ARM-specific platform properties for a virtual machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_platform_a_r_m.html](https://www.virtualbox.org/sdkref/interface_i_platform_a_r_m.html)
#[derive(Debug)]
pub struct PlatformARM {
    object: *mut IPlatformARM,
}

impl PlatformARM {
    pub(crate) fn new(object: *mut IPlatformARM) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for PlatformARM {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("release refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop PlatformARM. Error: {:?}", err)
            }
        }
    }
}
