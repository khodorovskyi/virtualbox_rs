use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::INATNetwork;

/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_n_a_t_network.html](https://www.virtualbox.org/sdkref/interface_i_n_a_t_network.html)
#[derive(Debug)]
pub struct NATNetwork {
    pub(crate) object: *mut INATNetwork,
}

impl NATNetwork {
    pub(crate) fn new(object: *mut INATNetwork) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for NATNetwork {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("NATNetwork refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop NATNetwork. Error: {:?}", err)
            }
        }
    }
}
