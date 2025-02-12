use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IMediumAttachment;

/// The IMediumAttachment interface links storage media to virtual machines. More...
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_medium_attachment.html](https://www.virtualbox.org/sdkref/interface_i_medium_attachment.html)
#[derive(Debug)]
pub struct MediumAttachment {
    object: *mut IMediumAttachment,
}

impl MediumAttachment {
    pub(crate) fn new(object: *mut IMediumAttachment) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for MediumAttachment {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("MediumAttachment refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop MediumAttachment. Error: {:?}", err)
            }
        }
    }
}
