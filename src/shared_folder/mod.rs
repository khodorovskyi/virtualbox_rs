use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ISharedFolder;

/// The [`SharedFolder`] interface represents a folder in the host computer's file system accessible from the guest OS running inside a virtual machine using an associated logical name. More...
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_shared_folder.html](https://www.virtualbox.org/sdkref/interface_i_shared_folder.html)
#[derive(Debug)]
pub struct SharedFolder {
    object: *mut ISharedFolder,
}

impl SharedFolder {
    pub(crate) fn new(object: *mut ISharedFolder) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for SharedFolder {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("SharedFolder refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop SharedFolder. Error: {:?}", err)
            }
        }
    }
}
