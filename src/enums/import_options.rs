#[cfg(doc)]
use crate::Appliance;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Import options, used with [`Appliance::import_machines`].
#[derive(Debug, Copy, Clone)]
pub enum ImportOptions {
    /// Don't generate new MAC addresses of the attached network adapters.
    KeepAllMACs,
    /// Don't generate new MAC addresses of the attached network adapters when they are using NAT.
    KeepNATMACs,
    /// Import all disks to VDI format.
    ImportToVDI,
}

impl From<u32> for ImportOptions {
    fn from(value: u32) -> Self {
        match value {
            raw::ImportOptions_ImportOptions_KeepAllMACs => ImportOptions::KeepAllMACs,
            raw::ImportOptions_ImportOptions_KeepNATMACs => ImportOptions::KeepNATMACs,
            raw::ImportOptions_ImportOptions_ImportToVDI => ImportOptions::ImportToVDI,
            _ => {
                error!("Unknown ImportOptions. Flag: {}", value);
                ImportOptions::KeepAllMACs
            }
        }
    }
}

impl Into<u32> for ImportOptions {
    fn into(self) -> u32 {
        match self {
            ImportOptions::KeepAllMACs => raw::ImportOptions_ImportOptions_KeepAllMACs,
            ImportOptions::KeepNATMACs => raw::ImportOptions_ImportOptions_KeepNATMACs,
            ImportOptions::ImportToVDI => raw::ImportOptions_ImportOptions_ImportToVDI,
        }
    }
}

impl Display for ImportOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
