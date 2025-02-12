use crate::enums::MachineState;
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::IMachine;

mod implementation;

/// The Machine interface represents a virtual machine, or guest, created in VirtualBox.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html)

pub struct Machine {
    pub(crate) object: *mut IMachine,
}

impl Machine {
    pub(crate) fn new(object: *mut IMachine) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Machine {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Machine refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Machine. Error: {:?}", err)
            }
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("name", self.get_name().unwrap_or("").to_string());
        map.insert("id", self.get_id().unwrap_or("").to_string());
        map.insert(
            "description",
            self.get_description().unwrap_or("").to_string(),
        );
        map.insert(
            "state",
            self.get_state().unwrap_or(MachineState::Null).to_string(),
        );

        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
