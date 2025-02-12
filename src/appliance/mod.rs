mod implementation;

use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use crate::utility::macros::macros::call_function;
use crate::{VboxError, VirtualBox};
use log::{debug, error};
use vbox_raw::sys_lib::IAppliance;

/// Represents a platform-independent appliance in OVF format.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_appliance.html](https://www.virtualbox.org/sdkref/interface_i_appliance.html)
pub struct Appliance {
    pub(crate) object: *mut IAppliance,
}

impl Appliance {
    pub(crate) fn new(object: *mut IAppliance) -> Self {
        Self { object }
    }

    /// Initializes the Appliance.
    ///
    /// # Returns
    ///
    /// Returns [`Appliance`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::Appliance;
    ///
    /// let appliance = Appliance::init().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method checks the VirtualBox version before initializing the appliance.
    /// It ensures compatibility and prevents potential issues due to version mismatches.
    /// Use this method if you have not checked the version beforehand.
    pub fn init() -> Result<Self, VboxError> {
        let vbox = VirtualBox::init()?;
        vbox.create_appliance()
    }

    /// Initializes the Appliance without checking the version.
    ///
    /// # Returns
    ///
    /// Returns [`Appliance`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::Appliance;
    ///
    /// let appliance = Appliance::init_unchecked().unwrap();
    /// ```
    /// # Details
    ///
    /// This method skips the version check and directly initializes the appliance.
    /// Use this method only if you have already checked the version and are confident it is correct.
    /// If the version is not checked and does not match, the application may crash with a core dump on random method calls.
    /// The speed of calling `init_unchecked` is minimally different from the regular `init`.
    pub fn init_unchecked() -> Result<Self, VboxError> {
        let vbox = VirtualBox::init_unchecked()?;
        vbox.create_appliance()
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Appliance {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Appliance refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Appliance. Error: {:?}", err)
            }
        }
    }
}

impl Display for Appliance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("path", self.get_path().unwrap_or("").to_string());
        map.insert("disks", format!("{:?}", self.get_disks().unwrap_or(vec![])));
        map.insert("virtual_system_descriptions", format!("{:?}", self.get_virtual_system_descriptions().unwrap_or(vec![])));
        map.insert("machines", format!("{:?}", self.get_machines().unwrap_or(vec![])));
        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for Appliance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
