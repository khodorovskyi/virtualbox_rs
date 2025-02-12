use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IHostOnlyNetwork;

/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_host_only_network.html](https://www.virtualbox.org/sdkref/interface_i_host_only_network.html)
#[derive(Debug)]
pub struct HostOnlyNetwork {
    pub(crate) object: *mut IHostOnlyNetwork,
}

impl HostOnlyNetwork {
    pub(crate) fn new(object: *mut IHostOnlyNetwork) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for HostOnlyNetwork {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("HostOnlyNetwork refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop HostOnlyNetwork. Error: {:?}", err)
            }
        }
    }
}
