use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IGuestFile;

/// Implementation of the IFile object for files in the guest.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_guest_file.html](https://www.virtualbox.org/sdkref/interface_i_guest_file.html)
#[derive(Debug)]
pub struct GuestFile {
    object: *mut IGuestFile,
}

impl GuestFile {
    pub(crate) fn new(object: *mut IGuestFile) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for GuestFile {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("GuestFile refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop GuestFile. Error: {:?}", err)
            }
        }
    }
}
