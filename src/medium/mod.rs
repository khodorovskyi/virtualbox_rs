use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::IMedium;
use crate::enums::{DeviceType, MediumState};

mod implementation;

/// The IMedium interface represents virtual storage for a machine's hard disks, CD/DVD or floppy drives.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_medium.html](https://www.virtualbox.org/sdkref/interface_i_medium.html)
pub struct Medium {
    pub(crate) object: *mut IMedium,
}

impl Medium {
    pub(crate) fn new(object: *mut IMedium) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Medium {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Medium refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Medium. Error: {:?}", err)
            }
        }
    }
}

impl Display for Medium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("id", self.get_id().unwrap_or("").to_string());
        map.insert("name", self.get_name().unwrap_or("").to_string());
        map.insert("state", self.get_state().unwrap_or(MediumState::NotCreated).to_string());
        map.insert("location", self.get_location().unwrap_or("").to_string());
        map.insert("device_type", self.get_device_type().unwrap_or(DeviceType::Null).to_string());
        map.insert("size", self.get_size().unwrap_or(0).to_string());
        map.insert("logical_size", self.get_logical_size().unwrap_or(0).to_string());
        map.insert(
            "description",
            self.get_description().unwrap_or("").to_string(),
        );
        map.insert("location", self.get_location().unwrap_or("").to_string());
        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for Medium {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
