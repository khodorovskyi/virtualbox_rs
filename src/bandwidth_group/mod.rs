pub mod implementation;

use std::collections::BTreeMap;
use crate::enums::BandwidthGroupType;
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::IBandwidthGroup;

/// Represents one bandwidth group.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_bandwidth_group.html](https://www.virtualbox.org/sdkref/interface_i_bandwidth_group.html)
pub struct BandwidthGroup {
    pub(crate) object: *mut IBandwidthGroup,
}

impl BandwidthGroup {
    pub(crate) fn new(object: *mut IBandwidthGroup) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for BandwidthGroup {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("BandwidthGroup refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop BandwidthGroup. Error: {:?}", err)
            }
        }
    }
}

impl Display for BandwidthGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("name", self.get_name().unwrap_or("").to_string());
        map.insert("type", format!("{:?}", self.get_type().unwrap_or(BandwidthGroupType::Null)));
        map.insert("reference", format!("{:?}", self.get_reference().unwrap_or(0)));
        map.insert("max_bytes_per_sec", format!("{:?}", self.get_max_bytes_per_sec().unwrap_or(0)));

        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for BandwidthGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
