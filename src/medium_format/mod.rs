mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IMediumFormat;

/// The IMediumFormat interface represents a medium format.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_medium_format.html](https://www.virtualbox.org/sdkref/interface_i_medium_format.html)
#[derive(Debug)]
pub struct MediumFormat {
    object: *mut IMediumFormat,
}

impl MediumFormat {
    pub(crate) fn new(object: *mut IMediumFormat) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for MediumFormat {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("MediumFormat refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop MediumFormat. Error: {:?}", err)
            }
        }
    }
}
