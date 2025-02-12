use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ISerialPort;

/// Virtual serial port device
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_serial_port.html](https://www.virtualbox.org/sdkref/interface_i_serial_port.html)
#[derive(Debug)]
pub struct SerialPort {
    object: *mut ISerialPort,
}

impl SerialPort {
    pub(crate) fn new(object: *mut ISerialPort) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for SerialPort {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("SerialPort refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop SerialPort. Error: {:?}", err)
            }
        }
    }
}
