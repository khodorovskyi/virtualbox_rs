pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::virtualbox_client::VirtualBoxClient;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ISession;

/// The Session interface represents a client process and allows for locking virtual machines (represented by Machine objects) to prevent conflicting changes to the machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_session.html](https://www.virtualbox.org/sdkref/interface_i_session.html)

pub struct Session {
    pub(crate) object: *mut ISession,
}

impl Session {
    pub(crate) fn new(object: *mut ISession) -> Self {
        Self { object }
    }

    pub fn init() -> Result<Session, VboxError> {
        let vbox_client = VirtualBoxClient::init()?;
        vbox_client.get_session()
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        match self.unlock_machine() {
            Ok(_) => {
                debug!("Session unlock");
            }
            Err(err) => {
                error!("Failed unlock Session. Error: {:?}", err)
            }
        }
        match self.release() {
            Ok(count) => {
                debug!("Session refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Session. Error: {:?}", err)
            }
        }
    }
}
