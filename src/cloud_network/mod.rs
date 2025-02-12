use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ICloudNetwork;

/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_cloud_network.html](https://www.virtualbox.org/sdkref/interface_i_cloud_network.html)
#[derive(Debug)]
pub struct CloudNetwork {
    pub(crate) object: *mut ICloudNetwork,
}

impl CloudNetwork {
    pub(crate) fn new(object: *mut ICloudNetwork) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for CloudNetwork {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("CloudNetwork refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop CloudNetwork. Error: {:?}", err)
            }
        }
    }
}
