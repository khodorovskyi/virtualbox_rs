pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::virtualbox::VirtualBox;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::ISystemProperties;

/// The SystemProperties interface represents global properties of the given VirtualBox installation.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_system_properties.html](https://www.virtualbox.org/sdkref/interface_i_system_properties.html)

#[derive(Debug)]
pub struct SystemProperties {
    object: *mut ISystemProperties,
}

impl SystemProperties {
    pub(crate) fn new(object: *mut ISystemProperties) -> Self {
        Self { object }
    }

    /// Initializes the SystemProperties.
    ///
    /// # Returns
    ///
    /// Returns [`SystemProperties`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method checks the VirtualBox version before initializing the system_properties.
    /// It ensures compatibility and prevents potential issues due to version mismatches.
    /// Use this method if you have not checked the version beforehand.
    pub fn init() -> Result<SystemProperties, VboxError> {
        let vbox = VirtualBox::init()?;
        vbox.get_system_properties()
    }

    /// Initializes the SystemProperties without checking the version.
    ///
    /// # Returns
    ///
    /// Returns [`SystemProperties`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init_unchecked().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method skips the version check and directly initializes the system_properties.
    /// Use this method only if you have already checked the version and are confident it is correct.
    /// If the version is not checked and does not match, the application may crash with a core dump on random method calls.
    /// The speed of calling `init_unchecked` is minimally different from the regular `init`.
    pub fn init_unchecked() -> Result<SystemProperties, VboxError> {
        let vbox = VirtualBox::init_unchecked()?;
        vbox.get_system_properties()
    }
}

impl SystemProperties {
    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for SystemProperties {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("SystemProperties refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop SystemProperties. Error: {:?}", err)
            }
        }
    }
}
