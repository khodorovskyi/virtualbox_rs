mod implementation;
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IMediumIO;

#[cfg(doc)]
use crate::Medium;
/// The MediumIO interface is used to access and modify the content of a medium.
///
/// The IMediumIO interface is used to access and modify the content of a medium.
///
/// It is returned by [`Medium::open_for_io`].
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_medium_i_o.html](https://www.virtualbox.org/sdkref/interface_i_medium_i_o.html)
#[derive(Debug)]
pub struct MediumIO {
    object: *mut IMediumIO,
}

impl MediumIO {
    pub(crate) fn new(object: *mut IMediumIO) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for MediumIO {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("MediumIO refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop MediumIO. Error: {:?}", err)
            }
        }
    }
}
