pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::virtualbox::VirtualBox;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ISystemProperties;

/// The SystemProperties interface represents global properties of the given VirtualBox installation.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_system_properties.html](https://www.virtualbox.org/sdkref/interface_i_system_properties.html)

#[derive(Debug)]
pub struct SystemProperties {
    object: *mut ISystemProperties,
}

impl SystemProperties {
    pub(crate) fn new(object: *mut ISystemProperties) -> Self {
        Self { object }
    }

    pub fn init() -> Result<SystemProperties, VboxError> {
        let vbox = VirtualBox::init()?;
        vbox.get_system_properties()
    }
}

impl SystemProperties {
    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for SystemProperties {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("SystemProperties refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop SystemProperties. Error: {:?}", err)
            }
        }
    }
}
