use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IUpdateAgent;

/// Abstract parent interface for handling updateable software components.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_update_agent.html](https://www.virtualbox.org/sdkref/interface_i_update_agent.html)
#[derive(Debug)]
pub struct UpdateAgent {
    object: *mut IUpdateAgent,
}

impl UpdateAgent {
    pub(crate) fn new(object: *mut IUpdateAgent) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for UpdateAgent {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("UpdateAgent refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop UpdateAgent. Error: {:?}", err)
            }
        }
    }
}
