use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IUSBController;

///
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_u_s_b_controller.html](https://www.virtualbox.org/sdkref/interface_i_u_s_b_controller.html)
#[derive(Debug)]
pub struct USBController {
    object: *mut IUSBController,
}

impl USBController {
    pub(crate) fn new(object: *mut IUSBController) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for USBController {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("USBController refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop USBController. Error: {:?}", err)
            }
        }
    }
}
