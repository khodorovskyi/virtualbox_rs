pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::virtualbox_client::VirtualBoxClient;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IVirtualBox;

/// The VirtualBox interface represents the main interface exposed by the product that provides virtual machine management.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_virtual_box.html](https://www.virtualbox.org/sdkref/interface_i_virtual_box.html)

pub struct VirtualBox {
    object: *mut IVirtualBox,
}

impl VirtualBox {
    pub(crate) fn new(object: *mut IVirtualBox) -> Self {
        Self { object }
    }

    pub fn init() -> Result<Self, VboxError> {
        debug!("VirtualBox::init");

        let client = VirtualBoxClient::init()?;
        client.get_virtualbox()
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VirtualBox {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("VirtualBox refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VirtualBox. Error: {:?}", err)
            }
        }
    }
}

unsafe impl Send for VirtualBox {}
