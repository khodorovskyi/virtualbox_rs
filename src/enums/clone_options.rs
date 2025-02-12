#[cfg(doc)]
use crate::Machine;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Clone options, used with [`Machine::clone_to`].
#[derive(Debug, Copy, Clone)]
pub enum CloneOptions {
    /// Create a clone VM where all virtual disks are linked to the original VM.
    Link,
    /// Don't generate new MAC addresses of the attached network adapters.
    KeepAllMACs,
    /// Don't generate new MAC addresses of the attached network adapters when they are using NAT.
    KeepNATMACs,
    /// Don't change the disk names.
    KeepDiskNames,
    /// Don't change UUID of the machine hardware.
    KeepHwUUIDs,
}

impl From<u32> for CloneOptions {
    fn from(value: u32) -> Self {
        match value {
            raw::CloneOptions_CloneOptions_Link => CloneOptions::Link,
            raw::CloneOptions_CloneOptions_KeepAllMACs => CloneOptions::KeepAllMACs,
            raw::CloneOptions_CloneOptions_KeepNATMACs => CloneOptions::KeepNATMACs,
            raw::CloneOptions_CloneOptions_KeepDiskNames => CloneOptions::KeepDiskNames,
            raw::CloneOptions_CloneOptions_KeepHwUUIDs => CloneOptions::KeepHwUUIDs,
            _ => {
                error!("Unknown CloneOptions. Type: {}", value);
                CloneOptions::KeepAllMACs
            }
        }
    }
}

impl Into<u32> for CloneOptions {
    fn into(self) -> u32 {
        match self {
            CloneOptions::Link => raw::CloneOptions_CloneOptions_Link,
            CloneOptions::KeepAllMACs => raw::CloneOptions_CloneOptions_KeepAllMACs,
            CloneOptions::KeepNATMACs => raw::CloneOptions_CloneOptions_KeepNATMACs,
            CloneOptions::KeepDiskNames => raw::CloneOptions_CloneOptions_KeepDiskNames,
            CloneOptions::KeepHwUUIDs => raw::CloneOptions_CloneOptions_KeepHwUUIDs,
        }
    }
}

impl Display for CloneOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
