use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IParallelPort;

/// The IParallelPort interface represents the virtual parallel port device.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_parallel_port.html](https://www.virtualbox.org/sdkref/interface_i_parallel_port.html)
#[derive(Debug)]
pub struct ParallelPort {
    object: *mut IParallelPort,
}

impl ParallelPort {
    pub(crate) fn new(object: *mut IParallelPort) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for ParallelPort {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("ParallelPort refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop ParallelPort. Error: {:?}", err)
            }
        }
    }
}
