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

    /// Initializes the Host.
    ///
    /// # Returns
    ///
    /// Returns [`Host`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method checks the VirtualBox version before initializing the host.
    /// It ensures compatibility and prevents potential issues due to version mismatches.
    /// Use this method if you have not checked the version beforehand.
    pub fn init() -> Result<Self, VboxError> {
        let vbox = VirtualBox::init()?;
        vbox.get_host()
    }

    /// Initializes the Host without checking the version.
    ///
    /// # Returns
    ///
    /// Returns [`Host`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init_unchecked().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method skips the version check and directly initializes the host.
    /// Use this method only if you have already checked the version and are confident it is correct.
    /// If the version is not checked and does not match, the application may crash with a core dump on random method calls.
    /// The speed of calling `init_unchecked` is minimally different from the regular `init`.
    pub fn init_unchecked() -> Result<Self, VboxError> {
        let vbox = VirtualBox::init_unchecked()?;
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
