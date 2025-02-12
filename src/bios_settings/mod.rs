use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IBIOSSettings;

/// The [`BIOSSettings`] interface represents BIOS settings of the virtual machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_b_i_o_s_settings.html](https://www.virtualbox.org/sdkref/interface_i_b_i_o_s_settings.html)
#[derive(Debug)]
pub struct BIOSSettings {
    object: *mut IBIOSSettings,
}

impl BIOSSettings {
    pub(crate) fn new(object: *mut IBIOSSettings) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for BIOSSettings {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("release refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop BIOSSettings. Error: {:?}", err)
            }
        }
    }
}
