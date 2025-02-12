use crate::enums::CPUArchitecture;
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::ICPUProfile;

mod implementation;

/// CPU profile
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_c_p_u_profile.html](https://www.virtualbox.org/sdkref/interface_i_c_p_u_profile.html)
pub struct CPUProfile {
    pub(crate) object: *mut ICPUProfile,
}

impl CPUProfile {
    pub(crate) fn new(object: *mut ICPUProfile) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for CPUProfile {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("CPUProfile refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop CPUProfile. Error: {:?}", err)
            }
        }
    }
}

impl Display for CPUProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("name", self.get_name().unwrap_or("").to_string());
        map.insert("full_name", self.get_full_name().unwrap_or("").to_string());
        map.insert(
            "architecture",
            self.get_architecture()
                .unwrap_or(CPUArchitecture::Any)
                .to_string(),
        );
        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for CPUProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
