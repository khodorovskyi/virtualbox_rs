use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IExtPackManager;

/// Interface for managing VirtualBox Extension Packs.
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_ext_pack_manager.html](https://www.virtualbox.org/sdkref/interface_i_ext_pack_manager.html)
#[derive(Debug)]
pub struct ExtPackManager {
    object: *mut IExtPackManager,
}

impl ExtPackManager {
    pub(crate) fn new(object: *mut IExtPackManager) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for ExtPackManager {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("ExtPackManager refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop ExtPackManager. Error: {:?}", err)
            }
        }
    }
}
