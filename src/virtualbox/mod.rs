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

    /// Initializes the VirtualBox.
    ///
    /// # Returns
    ///
    /// Returns [`VirtualBoxClient`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method checks the VirtualBox version before initializing the virtualbox.
    /// It ensures compatibility and prevents potential issues due to version mismatches.
    /// Use this method if you have not checked the version beforehand.
    pub fn init() -> Result<Self, VboxError> {
        debug!("VirtualBox::init");

        let client = VirtualBoxClient::init()?;
        client.get_virtualbox()
    }
    /// Initializes the VirtualBox without checking the version.
    ///
    /// # Returns
    ///
    /// Returns [`VirtualBoxClient`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init_unchecked().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method skips the version check and directly initializes the virtualbox.
    /// Use this method only if you have already checked the version and are confident it is correct.
    /// If the version is not checked and does not match, the application may crash with a core dump on random method calls.
    /// The speed of calling `init_unchecked` is minimally different from the regular `init`.
    pub fn init_unchecked() -> Result<Self, VboxError> {
        debug!("VirtualBox::init_unchecked");

        let client = VirtualBoxClient::init_unchecked()?;
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
