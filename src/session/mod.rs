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

    /// Initializes the Session.
    ///
    /// # Returns
    ///
    /// Returns [`Session`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::Session;
    ///
    /// let session = Session::init().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method checks the VirtualBox version before initializing the session.
    /// It ensures compatibility and prevents potential issues due to version mismatches.
    /// Use this method if you have not checked the version beforehand.
    pub fn init() -> Result<Session, VboxError> {
        let vbox_client = VirtualBoxClient::init()?;
        vbox_client.get_session()
    }


    /// Initializes the Session without checking the version.
    ///
    /// # Returns
    ///
    /// Returns [`Session`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Session;
    ///
    /// let session = Session::init_unchecked().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method skips the version check and directly initializes the session.
    /// Use this method only if you have already checked the version and are confident it is correct.
    /// If the version is not checked and does not match, the application may crash with a core dump on random method calls.
    /// The speed of calling `init_unchecked` is minimally different from the regular `init`.
    pub fn init_unchecked() -> Result<Session, VboxError> {
        let vbox_client = VirtualBoxClient::init_unchecked()?;
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
