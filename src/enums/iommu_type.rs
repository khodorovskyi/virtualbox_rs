#[cfg(doc)]
use crate::Machine;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;

/// This enumeration represents possible values for the [`Machine::get_iommu_type`] attribute.
#[derive(Debug, Copy, Clone)]
pub enum IommuType {
    /// No IOMMU is present.
    None,
    /// No IOMMU is present.
    Automatic,
    /// An AMD IOMMU.
    AMD,
    /// An Intel IOMMU.
    Intel,
}

#[cfg(not(is_v_6_1))]
impl From<u32> for IommuType {
    fn from(value: u32) -> Self {
        match value {
            raw::IommuType_IommuType_None => IommuType::None,
            raw::IommuType_IommuType_Automatic => IommuType::Automatic,
            raw::IommuType_IommuType_AMD => IommuType::AMD,
            raw::IommuType_IommuType_Intel => IommuType::Intel,
            _ => {
                error!("Unknown IommuType. Flag: {}", value);
                IommuType::None
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for IommuType {
    fn into(self) -> u32 {
        match self {
            IommuType::None => raw::IommuType_IommuType_None,
            IommuType::Automatic => raw::IommuType_IommuType_Automatic,
            IommuType::AMD => raw::IommuType_IommuType_AMD,
            IommuType::Intel => raw::IommuType_IommuType_Intel,
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for IommuType {
    fn from(_value: u32) -> Self {
        IommuType::None
    }
}

#[cfg(is_v_6_1)]
impl Into<u32> for IommuType {
    fn into(self) -> u32 {
        0
    }
}
impl Display for IommuType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
