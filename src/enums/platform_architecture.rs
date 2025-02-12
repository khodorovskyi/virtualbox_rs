#[cfg(is_v_7_1)]
use log::error;
use std::fmt::Display;
#[cfg(is_v_7_1)]
use vbox_raw::sys_lib as raw;
#[cfg(doc)]
use crate::enums::CPUArchitecture;

/// Platform architecture.
#[derive(Debug)]
pub enum PlatformArchitecture {
    /// No platform selected.
    ///
    /// Never used by the API.
    None,
    /// x86-based platform (AMD64 / x86).
    ///
    /// Valid CPUArchitecture choices: [`CPUArchitecture::AMD64`], [`CPUArchitecture::X86`]
    X86,
    /// ARM-based platform (AArch32, AArch64).
    ///
    /// Valid CPUArchitecture choices: [`CPUArchitecture::ARMv8_32`], [`CPUArchitecture::ARMv8_64`]
    ARM,
}

#[cfg(is_v_7_1)]
impl From<u32> for PlatformArchitecture {
    fn from(value: u32) -> Self {
        match value {
            raw::PlatformArchitecture_PlatformArchitecture_None => PlatformArchitecture::None,
            raw::PlatformArchitecture_PlatformArchitecture_x86 => PlatformArchitecture::X86,
            raw::PlatformArchitecture_PlatformArchitecture_ARM => PlatformArchitecture::ARM,
            _ => {
                error!("ARM PlatformArchitecture. PlatformArchitecture: {}", value);
                PlatformArchitecture::ARM
            }
        }
    }
}

#[cfg(is_v_7_1)]
impl Into<u32> for PlatformArchitecture {
    fn into(self) -> u32 {
        match self {
            PlatformArchitecture::None => raw::PlatformArchitecture_PlatformArchitecture_None,
            PlatformArchitecture::X86 => raw::PlatformArchitecture_PlatformArchitecture_x86,
            PlatformArchitecture::ARM => raw::PlatformArchitecture_PlatformArchitecture_ARM,
        }
    }
}

#[cfg(not(is_v_7_1))]
impl From<u32> for PlatformArchitecture {
    fn from(_value: u32) -> Self {
        PlatformArchitecture::None
    }
}

#[cfg(not(is_v_7_1))]
impl Into<u32> for PlatformArchitecture {
    fn into(self) -> u32 {
        0
    }
}
impl Display for PlatformArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
