pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IHostNetworkInterface;

/// Represents one of host's network interfaces
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_host_network_interface.html](https://www.virtualbox.org/sdkref/interface_i_host_network_interface.html)

#[derive(Debug)]
pub struct HostNetworkInterface {
    object: *mut IHostNetworkInterface,
}

impl HostNetworkInterface {
    pub(crate) fn new(object: *mut IHostNetworkInterface) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for HostNetworkInterface {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("HostNetworkInterface refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop HostNetworkInterface. Error: {:?}", err)
            }
        }
    }
}
