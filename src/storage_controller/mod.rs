use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IStorageController;

#[cfg(doc)]
use crate::Machine;

///Represents a storage controller that is attached to a virtual machine ([`Machine`])
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_storage_controller.html](https://www.virtualbox.org/sdkref/interface_i_storage_controller.html)
#[derive(Debug)]
pub struct StorageController {
    object: *mut IStorageController,
}

impl StorageController {
    pub(crate) fn new(object: *mut IStorageController) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for StorageController {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("StorageController refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop StorageController. Error: {:?}", err)
            }
        }
    }
}
