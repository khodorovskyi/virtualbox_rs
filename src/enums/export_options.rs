#[cfg(doc)]
use crate::Appliance;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Export options, used with [`Appliance::write`].
#[derive(Debug, Copy, Clone)]
pub enum ExportOptions {
    /// Write the optional manifest file (.mf) which is used for integrity checks prior import.
    CreateManifest,
    /// Export DVD images.
    ///
    /// Default is not to export them as it is rarely needed for typical VMs.
    ExportDVDImages,
    /// Do not export any MAC address information.
    ///
    /// Default is to keep them to avoid losing information which can cause trouble after import, at the price of risking duplicate MAC addresses, if the import options are used to keep them.
    StripAllMACs,
    /// Do not export any MAC address information, except for adapters using NAT.
    ///
    /// Default is to keep them to avoid losing information which can cause trouble after import, at the price of risking duplicate MAC addresses, if the import options are used to keep them.
    StripAllNonNATMACs,
}

impl From<u32> for ExportOptions {
    fn from(value: u32) -> Self {
        match value {
            raw::ExportOptions_ExportOptions_CreateManifest => ExportOptions::CreateManifest,
            raw::ExportOptions_ExportOptions_ExportDVDImages => ExportOptions::ExportDVDImages,
            raw::ExportOptions_ExportOptions_StripAllMACs => ExportOptions::StripAllMACs,
            raw::ExportOptions_ExportOptions_StripAllNonNATMACs => {
                ExportOptions::StripAllNonNATMACs
            }
            _ => {
                error!("Unknown ExportOptions. Flag: {}", value);
                ExportOptions::CreateManifest
            }
        }
    }
}

impl Into<u32> for ExportOptions {
    fn into(self) -> u32 {
        match self {
            ExportOptions::CreateManifest => raw::ExportOptions_ExportOptions_CreateManifest,
            ExportOptions::ExportDVDImages => raw::ExportOptions_ExportOptions_ExportDVDImages,
            ExportOptions::StripAllMACs => raw::ExportOptions_ExportOptions_StripAllMACs,
            ExportOptions::StripAllNonNATMACs => {
                raw::ExportOptions_ExportOptions_StripAllNonNATMACs
            }
        }
    }
}

impl Display for ExportOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
