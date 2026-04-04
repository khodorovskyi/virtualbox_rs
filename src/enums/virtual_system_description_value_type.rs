#[cfg(doc)]
use crate::VirtualSystemDescription;
use log::error;
use vbox_raw::sys_lib as raw;

/// Used with [`VirtualSystemDescription::get_values_by_type`] to describe the value type to fetch.
#[derive(Debug, Copy, Clone)]
pub enum VirtualSystemDescriptionValueType {
    Reference,
    Original,
    Auto,
    ExtraConfig,
}

impl From<u32> for VirtualSystemDescriptionValueType {
    fn from(value: u32) -> Self {
        match value {
            raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_Reference  => VirtualSystemDescriptionValueType::Reference ,
            raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_Original => VirtualSystemDescriptionValueType::Original,
            raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_Auto => VirtualSystemDescriptionValueType::Auto,
            raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_ExtraConfig => VirtualSystemDescriptionValueType::ExtraConfig,
            _ => {
                error!("Unknown VirtualSystemDescriptionValueType. Flag: {}", value);
                VirtualSystemDescriptionValueType::Reference
            }
        }
    }
}

impl Into<u32> for VirtualSystemDescriptionValueType {
    fn into(self) -> u32 {
        match self {
            VirtualSystemDescriptionValueType::Reference => {
                raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_Reference
            }
            VirtualSystemDescriptionValueType::Original => {
                raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_Original
            }
            VirtualSystemDescriptionValueType::Auto => {
                raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_Auto
            }
            VirtualSystemDescriptionValueType::ExtraConfig => {
                raw::VirtualSystemDescriptionValueType_VirtualSystemDescriptionValueType_ExtraConfig
            }
        }
    }
}
