pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::{VboxError, VirtualBox};
use log::{debug, error};
use vbox_raw::sys_lib::IHost;

/// The IHost interface represents the physical machine that this VirtualBox installation runs on. More...
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_host.html](https://www.virtualbox.org/sdkref/interface_i_host.html)

#[derive(Debug)]
pub struct Host {
    object: *mut IHost,
}

impl Host {
    pub(crate) fn new(object: *mut IHost) -> Self {
        Self { object }
    }

    pub fn init() -> Result<Self, VboxError> {
        let vbox = VirtualBox::init()?;
        vbox.get_host()
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Host refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Host. Error: {:?}", err)
            }
        }
    }
}
