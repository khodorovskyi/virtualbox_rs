
pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::{SystemProperties, VboxError};
use log::{debug, error};
use vbox_raw::sys_lib::IPlatformProperties;

/// Properties of a specific virtualization platform.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_platform_properties.html](https://www.virtualbox.org/sdkref/interface_i_platform_properties.html)

#[derive(Debug)]
pub struct PlatformProperties {
    object: *mut IPlatformProperties,
}

impl PlatformProperties {
    pub(crate) fn new(object: *mut IPlatformProperties) -> Self {
        Self { object }
    }

    /// Initializes the PlatformProperties.
    ///
    /// # Returns
    ///
    /// Returns [`PlatformProperties`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method checks the VirtualBox version before initializing the platform_properties.
    /// It ensures compatibility and prevents potential issues due to version mismatches.
    /// Use this method if you have not checked the version beforehand.
    pub fn init() -> Result<PlatformProperties, VboxError> {
        let system_properties = SystemProperties::init()?;
        system_properties.get_platform_properties()
    }

    /// Initializes the PlatformProperties without checking the version.
    ///
    /// # Returns
    ///
    /// Returns [`PlatformProperties`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init_unchecked().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method skips the version check and directly initializes the platform_properties.
    /// Use this method only if you have already checked the version and are confident it is correct.
    /// If the version is not checked and does not match, the application may crash with a core dump on random method calls.
    /// The speed of calling `init_unchecked` is minimally different from the regular `init`.
    pub fn init_unchecked() -> Result<PlatformProperties, VboxError> {
        let system_properties = SystemProperties::init()?;
        system_properties.get_platform_properties()
    }
}

impl PlatformProperties {
    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for PlatformProperties {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("PlatformProperties refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop PlatformProperties. Error: {:?}", err)
            }
        }
    }
}
