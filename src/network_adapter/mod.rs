pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::INetworkAdapter;

/// The NetworkAdapter interface represents the audio settings for a virtual machine. More...
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_network_adapter.html](https://www.virtualbox.org/sdkref/interface_i_network_adapter.html)
#[derive(Debug)]
pub struct NetworkAdapter {
    object: *mut INetworkAdapter,
}

impl NetworkAdapter {
    pub(crate) fn new(object: *mut INetworkAdapter) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for NetworkAdapter {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("NetworkAdapter refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop NetworkAdapter. Error: {:?}", err)
            }
        }
    }
}
