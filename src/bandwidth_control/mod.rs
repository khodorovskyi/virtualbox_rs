pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IBandwidthControl;

/// Controls the bandwidth groups of one machine used to cap I/O done by a VM.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_bandwidth_control.html](https://www.virtualbox.org/sdkref/interface_i_bandwidth_control.html)
#[derive(Debug)]
pub struct BandwidthControl {
    object: *mut IBandwidthControl,
}

impl BandwidthControl {
    pub(crate) fn new(object: *mut IBandwidthControl) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for BandwidthControl {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("BandwidthControl refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop BandwidthControl. Error: {:?}", err)
            }
        }
    }
}
