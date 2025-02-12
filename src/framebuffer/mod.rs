mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IFramebuffer;

/// IFramebuffer Interface Reference
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_framebuffer.html](https://www.virtualbox.org/sdkref/interface_i_framebuffer.html)
#[derive(Debug, Clone)]
pub struct Framebuffer {
    object: *mut IFramebuffer,
}

impl Framebuffer {
    pub(crate) fn new(object: *mut IFramebuffer) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Framebuffer refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Framebuffer. Error: {:?}", err)
            }
        }
    }
}

unsafe impl Sync for Framebuffer {}
unsafe impl Send for Framebuffer {}
