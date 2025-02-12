use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IVRDEServer;

///
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_v_r_d_e_server.html](https://www.virtualbox.org/sdkref/interface_i_v_r_d_e_server.html)
#[derive(Debug)]
pub struct VRDEServer {
    object: *mut IVRDEServer,
}

impl VRDEServer {
    pub(crate) fn new(object: *mut IVRDEServer) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VRDEServer {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("VRDEServer refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VRDEServer. Error: {:?}", err)
            }
        }
    }
}
