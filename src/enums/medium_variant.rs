use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// More than one flag may be set.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MediumVariant {
    /// No particular variant requested, results in using the backend default.
    Standard,
    /// VMDK image split in chunks of less than 2GByte.
    VmdkSplit2G,
    /// VMDK image representing a raw disk.
    VmdkRawDisk,
    /// VMDK streamOptimized image.
    ///
    /// Special import/export format which is read-only/append-only.
    VmdkStreamOptimized,
    /// VMDK format variant used on ESX products.
    VmdkESX,
    /// Fill new blocks with zeroes while expanding image file.
    VdiZeroExpand,
    /// Fixed image.
    ///
    /// Only allowed for base images.
    Fixed,
    /// Differencing image.
    ///
    /// Only allowed for child images.
    Diff,
    /// Special flag which requests formatting the disk image.
    ///
    /// Right now supported for floppy images only.
    Formatted,
    /// Special flag which suppresses automatic creation of the subdirectory.
    ///
    /// Only used when passing the medium variant as an input parameter.
    NoCreateDir,
}
impl Into<u32> for MediumVariant {
    fn into(self) -> u32 {
        match self {
            MediumVariant::Standard => raw::MediumVariant_MediumVariant_Standard,
            MediumVariant::VmdkSplit2G => raw::MediumVariant_MediumVariant_VmdkSplit2G,
            MediumVariant::VmdkRawDisk => raw::MediumVariant_MediumVariant_VmdkRawDisk,
            MediumVariant::VmdkStreamOptimized => {
                raw::MediumVariant_MediumVariant_VmdkStreamOptimized
            }
            MediumVariant::VmdkESX => raw::MediumVariant_MediumVariant_VmdkESX,
            MediumVariant::VdiZeroExpand => raw::MediumVariant_MediumVariant_VdiZeroExpand,
            MediumVariant::Fixed => raw::MediumVariant_MediumVariant_Fixed,
            MediumVariant::Diff => raw::MediumVariant_MediumVariant_Diff,
            MediumVariant::Formatted => raw::MediumVariant_MediumVariant_Formatted,
            MediumVariant::NoCreateDir => raw::MediumVariant_MediumVariant_NoCreateDir,
        }
    }
}

impl From<u32> for MediumVariant {
    fn from(value: u32) -> Self {
        match value {
            raw::MediumVariant_MediumVariant_Standard => MediumVariant::Standard,
            raw::MediumVariant_MediumVariant_VmdkSplit2G => MediumVariant::VmdkSplit2G,
            raw::MediumVariant_MediumVariant_VmdkRawDisk => MediumVariant::VmdkRawDisk,
            raw::MediumVariant_MediumVariant_VmdkStreamOptimized => {
                MediumVariant::VmdkStreamOptimized
            }
            raw::MediumVariant_MediumVariant_VmdkESX => MediumVariant::VmdkESX,
            raw::MediumVariant_MediumVariant_VdiZeroExpand => MediumVariant::VdiZeroExpand,
            raw::MediumVariant_MediumVariant_Fixed => MediumVariant::Fixed,
            raw::MediumVariant_MediumVariant_Diff => MediumVariant::Diff,
            raw::MediumVariant_MediumVariant_Formatted => MediumVariant::Formatted,
            raw::MediumVariant_MediumVariant_NoCreateDir => MediumVariant::NoCreateDir,
            _ => {
                error!("MediumVariant::from. Unknown type: {}", value);
                MediumVariant::Standard
            }
        }
    }
}

impl Display for MediumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
