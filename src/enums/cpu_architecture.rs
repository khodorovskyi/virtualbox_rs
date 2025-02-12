#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib as raw;

/// Basic CPU architecture types.
#[derive(Debug, Copy, Clone)]
pub enum CPUArchitecture {
    /// Matches any CPU architecture.
    Any,
    /// 32-bit (and 16-bit) x86.
    X86,
    /// 64-bit x86.
    ///
    /// (Also known as x86-64 or x64.)
    AMD64,
    /// 32-bit only ARMv8.
    ///
    /// (Also known as AArch32 or ARM32.)
    ARMv8_32,
    /// 64-bit only ARMv8.
    ///
    /// (Also known as AArch64 or ARM64.)
    ARMv8_64
}
#[cfg(not(is_v_6_1))]
impl From<u32> for CPUArchitecture {
    fn from(value: u32) -> Self {
        match value {
            raw::CPUArchitecture_CPUArchitecture_Any => CPUArchitecture::Any,
            raw::CPUArchitecture_CPUArchitecture_x86 => CPUArchitecture::X86,
            raw::CPUArchitecture_CPUArchitecture_AMD64 => CPUArchitecture::AMD64,
            #[cfg(is_v_7_1)]
            raw::CPUArchitecture_CPUArchitecture_ARMv8_32 => CPUArchitecture::ARMv8_32,
            #[cfg(is_v_7_1)]
            raw::CPUArchitecture_CPUArchitecture_ARMv8_64 => CPUArchitecture::ARMv8_64,
            _ => {
                error!("Unknown CPUArchitecture. CPUArchitecture: {}", value);
                CPUArchitecture::Any
            }
        }
    }
}

#[cfg(not(is_v_6_1))]
impl Into<u32> for CPUArchitecture {
    fn into(self) -> u32 {
        match self {
            CPUArchitecture::X86 => raw::CPUArchitecture_CPUArchitecture_x86,
            CPUArchitecture::AMD64 => raw::CPUArchitecture_CPUArchitecture_AMD64,
            #[cfg(is_v_7_1)]
            CPUArchitecture::ARMv8_32 => raw::CPUArchitecture_CPUArchitecture_ARMv8_32,
            #[cfg(is_v_7_1)]
            CPUArchitecture::ARMv8_64 => raw::CPUArchitecture_CPUArchitecture_ARMv8_64,
            _ => raw::CPUArchitecture_CPUArchitecture_Any
        }
    }
}

#[cfg(is_v_6_1)]
impl From<u32> for CPUArchitecture {
    fn from(_value: u32) -> Self {
        CPUArchitecture::Any
    }
}

#[cfg(is_v_6_1)]
impl Into<u32> for CPUArchitecture {
    fn into(self) -> u32 {
        0
    }
}

impl Display for CPUArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
