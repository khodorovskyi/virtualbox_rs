use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::INvramStore;

/// Provides access to the NVRAM store collecting all permanent states from different sources (UEFI, TPM, etc.).
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_nvram_store.html](https://www.virtualbox.org/sdkref/interface_i_nvram_store.html)
#[derive(Debug)]
pub struct NvramStore {
    object: *mut INvramStore,
}

impl NvramStore {
    pub(crate) fn new(object: *mut INvramStore) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for NvramStore {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("NvramStore refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop NvramStore. Error: {:?}", err)
            }
        }
    }
}
