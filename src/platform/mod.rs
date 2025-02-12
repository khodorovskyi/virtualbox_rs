pub mod implementation;

use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use crate::utility::macros::macros::call_function;
use crate::{VboxError};
use log::{debug, error};
use vbox_raw::sys_lib::IPlatform;
use crate::enums::{ChipsetType, IommuType, PlatformArchitecture};

/// Properties of a specific virtualization platform.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_platform_properties.html](https://www.virtualbox.org/sdkref/interface_i_platform_properties.html)

pub struct Platform {
    object: *mut IPlatform,
}

impl Platform {
    pub(crate) fn new(object: *mut IPlatform) -> Self {
        Self { object }
    }
}

impl Platform {
    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Platform refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Platform. Error: {:?}", err)
            }
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("architecture", self.get_architecture().unwrap_or(PlatformArchitecture::None).to_string());
        map.insert("chipset_type", self.get_chipset_type().unwrap_or(ChipsetType::Null).to_string());
        map.insert("iommu_type", self.get_iommu_type().unwrap_or(IommuType::None).to_string());
        map.insert("rtc_use_utc", self.get_rtc_use_utc().unwrap_or(false).to_string());


        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

