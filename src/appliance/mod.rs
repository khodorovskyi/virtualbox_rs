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

    pub fn init() -> Result<Self, VboxError> {
        let vbox = VirtualBox::init()?;
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
