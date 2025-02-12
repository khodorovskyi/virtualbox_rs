use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IPCIDeviceAttachment;

/// Information about PCI attachments.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_p_c_i_device_attachment.html](https://www.virtualbox.org/sdkref/interface_i_p_c_i_device_attachment.html)
#[derive(Debug)]
pub struct PCIDeviceAttachment {
    object: *mut IPCIDeviceAttachment,
}

impl PCIDeviceAttachment {
    pub(crate) fn new(object: *mut IPCIDeviceAttachment) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for PCIDeviceAttachment {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("PCIDeviceAttachment refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop PCIDeviceAttachment. Error: {:?}", err)
            }
        }
    }
}
