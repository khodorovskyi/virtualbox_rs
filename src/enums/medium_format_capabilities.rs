#[cfg(doc)]
use crate::{Medium, MediumFormat};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Medium format capability flags.
#[derive(Debug, Eq, PartialEq)]
pub enum MediumFormatCapabilities {
    /// Supports UUIDs as expected by VirtualBox code.
    Uuid,
    /// Supports creating fixed size images, allocating all space instantly.
    CreateFixed,
    /// Supports creating dynamically growing images, allocating space on demand.
    CreateDynamic,
    /// Supports creating images split in chunks of a bit less than 2 GBytes.
    CreateSplit2G,
    /// Supports being used as a format for differencing media (see [`Medium::create_diff_storage`]).
    Differencing,
    /// ASupports asynchronous I/O operations for at least some configurations.
    Asynchronous,
    /// The format backend operates on files (the [`Medium::get_location`] attribute of the medium specifies a file used to store medium data; for a list of supported file extensions see [`MediumFormat::describe_file_extensions`]).
    File,
    /// The format backend uses the property interface to configure the storage location and properties (the [`MediumFormat::describe_properties`] method is used to get access to properties supported by the given medium format).
    Properties,
    /// The format backend uses the TCP networking interface for network access.
    TcpNetworking,
    /// The format backend supports virtual filesystem functionality.
    VFS,
    /// The format backend supports discarding blocks.
    Discard,
    /// Indicates that this is a frequently used format backend.
    Preferred,
    CapabilityMask,
}
impl Into<u32> for MediumFormatCapabilities {
    fn into(self) -> u32 {
        match self {
            MediumFormatCapabilities::Uuid => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_Uuid
            }
            MediumFormatCapabilities::CreateFixed => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_CreateFixed
            }
            MediumFormatCapabilities::CreateDynamic => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_CreateDynamic
            }
            MediumFormatCapabilities::CreateSplit2G => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_CreateSplit2G
            }
            MediumFormatCapabilities::Differencing => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_Differencing
            }
            MediumFormatCapabilities::Asynchronous => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_Asynchronous
            }
            MediumFormatCapabilities::File => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_File
            }
            MediumFormatCapabilities::Properties => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_Properties
            }
            MediumFormatCapabilities::TcpNetworking => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_TcpNetworking
            }
            MediumFormatCapabilities::VFS => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_VFS
            }
            MediumFormatCapabilities::Discard => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_Discard
            }
            MediumFormatCapabilities::Preferred => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_Preferred
            }
            MediumFormatCapabilities::CapabilityMask => {
                raw::MediumFormatCapabilities_MediumFormatCapabilities_CapabilityMask
            }
        }
    }
}

impl From<u32> for MediumFormatCapabilities {
    fn from(value: u32) -> Self {
        match value {
            raw::MediumFormatCapabilities_MediumFormatCapabilities_Uuid => {
                MediumFormatCapabilities::Uuid
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_CreateFixed => {
                MediumFormatCapabilities::CreateFixed
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_CreateDynamic => {
                MediumFormatCapabilities::CreateDynamic
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_CreateSplit2G => {
                MediumFormatCapabilities::CreateSplit2G
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_Differencing => {
                MediumFormatCapabilities::Differencing
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_Asynchronous => {
                MediumFormatCapabilities::Asynchronous
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_File => {
                MediumFormatCapabilities::File
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_Properties => {
                MediumFormatCapabilities::Properties
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_TcpNetworking => {
                MediumFormatCapabilities::TcpNetworking
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_VFS => {
                MediumFormatCapabilities::VFS
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_Discard => {
                MediumFormatCapabilities::Discard
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_Preferred => {
                MediumFormatCapabilities::Preferred
            }
            raw::MediumFormatCapabilities_MediumFormatCapabilities_CapabilityMask => {
                MediumFormatCapabilities::CapabilityMask
            }
            _ => {
                error!("MediumFormatCapabilities::from. Unknown type: {}", value);
                MediumFormatCapabilities::Uuid
            }
        }
    }
}

impl Display for MediumFormatCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
