
pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::{SystemProperties, VboxError};
use log::{debug, error};
use vbox_raw::sys_lib::IPlatformProperties;

/// Properties of a specific virtualization platform.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_platform_properties.html](https://www.virtualbox.org/sdkref/interface_i_platform_properties.html)

#[derive(Debug)]
pub struct PlatformProperties {
    object: *mut IPlatformProperties,
}

impl PlatformProperties {
    pub(crate) fn new(object: *mut IPlatformProperties) -> Self {
        Self { object }
    }

    pub fn init() -> Result<PlatformProperties, VboxError> {
        let system_properties = SystemProperties::init()?;
        system_properties.get_platform_properties()
    }
}

impl PlatformProperties {
    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for PlatformProperties {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("PlatformProperties refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop PlatformProperties. Error: {:?}", err)
            }
        }
    }
}
