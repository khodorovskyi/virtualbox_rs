use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IExtPack;
// TODO: implement all methods of IExtPack

/// Interface for managing VirtualBox Extension Packs.
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_ext_pack_manager.html](https://www.virtualbox.org/sdkref/interface_i_ext_pack_manager.html)
#[derive(Debug)]
pub struct ExtPack {
    object: *mut IExtPack,
}

impl ExtPack {
    // pub(crate) fn new(object: *mut IExtPack) -> Self {
    //     Self { object }
    // }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for ExtPack {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("ExtPack refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop ExtPack. Error: {:?}", err)
            }
        }
    }
}
