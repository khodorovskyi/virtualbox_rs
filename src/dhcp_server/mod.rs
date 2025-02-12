use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IDHCPServer;

/// The DHCPServer interface represents the VirtualBox DHCP server configuration.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_d_h_c_p_server.html](https://www.virtualbox.org/sdkref/interface_i_d_h_c_p_server.html)
#[derive(Debug)]
pub struct DHCPServer {
    pub(crate) object: *mut IDHCPServer,
}

impl DHCPServer {
    pub(crate) fn new(object: *mut IDHCPServer) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for DHCPServer {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("DHCPServer refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop DHCPServer. Error: {:?}", err)
            }
        }
    }
}
