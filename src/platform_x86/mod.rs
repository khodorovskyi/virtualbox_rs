pub mod implementation;

use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use crate::utility::macros::macros::call_function;
use crate::{VboxError};
use log::{debug, error};
use vbox_raw::sys_lib::IPlatformX86;

/// The x86 specific platform properties for a virtual machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_platform_x86.html](https://www.virtualbox.org/sdkref/interface_i_platform_x86.html)

pub struct PlatformX86 {
    object: *mut IPlatformX86,
}

impl PlatformX86 {
    pub(crate) fn new(object: *mut IPlatformX86) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for PlatformX86 {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("release refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop PlatformX86. Error: {:?}", err)
            }
        }
    }
}

impl Display for PlatformX86 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("hpet_enabled", self.get_hpet_enabled().unwrap_or(false).to_string());


        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for PlatformX86 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}