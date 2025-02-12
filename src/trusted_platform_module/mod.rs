use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ITrustedPlatformModule;

/// The [`TrustedPlatformModule`] interface represents the settings of the virtual machine's trusted platform module.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_trusted_platform_module.html](https://www.virtualbox.org/sdkref/interface_i_trusted_platform_module.html)
#[derive(Debug)]
pub struct TrustedPlatformModule {
    object: *mut ITrustedPlatformModule,
}

impl TrustedPlatformModule {
    pub(crate) fn new(object: *mut ITrustedPlatformModule) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for TrustedPlatformModule {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("TrustedPlatformModule refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop TrustedPlatformModule. Error: {:?}", err)
            }
        }
    }
}
