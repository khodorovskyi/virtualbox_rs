pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::ISnapshot;

/// The Snapshot interface represents a snapshot of the virtual machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_snapshot.html](https://www.virtualbox.org/sdkref/interface_i_snapshot.html)

pub struct Snapshot {
    pub(crate) object: *mut ISnapshot,
}

impl Snapshot {
    pub(crate) fn new(object: *mut ISnapshot) -> Self {
        Self { object }
    }
}

impl Snapshot {
    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Snapshot refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Snapshot. Error: {:?}", err)
            }
        }
    }
}

impl Display for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("id", self.get_id().unwrap_or("").to_string());
        map.insert("name", self.get_name().unwrap_or("").to_string());
        map.insert(
            "description",
            self.get_description().unwrap_or("").to_string(),
        );
        map.insert("time_stamp", self.get_time_stamp().unwrap_or(0).to_string());
        map.insert("online", self.get_online().unwrap_or(false).to_string());
        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for Snapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
