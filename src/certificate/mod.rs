use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ICertificate;

/// X.509 certificate details.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_certificate.html](https://www.virtualbox.org/sdkref/interface_i_certificate.html)
#[derive(Debug)]
pub struct Certificate {
    object: *mut ICertificate,
}

impl Certificate {
    pub(crate) fn new(object: *mut ICertificate) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Certificate {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("release refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Certificate. Error: {:?}", err)
            }
        }
    }
}
