use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Firmware type.
#[derive(Debug, Copy, Clone)]
pub enum FirmwareType {
    /// BIOS Firmware.
    BIOS,
    /// EFI Firmware, bitness detected basing on OS type.
    EFI,
    /// EFI firmware, 32-bit.
    EFI32,
    /// EFI firmware, 64-bit.
    EFI64,
    /// EFI firmware, combined 32 and 64-bit.
    EFIDUAL,
}

impl From<u32> for FirmwareType {
    fn from(value: u32) -> Self {
        match value {
            raw::FirmwareType_FirmwareType_BIOS => FirmwareType::BIOS,
            raw::FirmwareType_FirmwareType_EFI => FirmwareType::EFI,
            raw::FirmwareType_FirmwareType_EFI32 => FirmwareType::EFI32,
            raw::FirmwareType_FirmwareType_EFI64 => FirmwareType::EFI64,
            raw::FirmwareType_FirmwareType_EFIDUAL => FirmwareType::EFIDUAL,
            _ => {
                error!("Unknown FirmwareType. FirmwareType: {}", value);
                FirmwareType::BIOS
            }
        }
    }
}

impl Into<u32> for FirmwareType {
    fn into(self) -> u32 {
        match self {
            FirmwareType::BIOS => raw::FirmwareType_FirmwareType_BIOS,
            FirmwareType::EFI => raw::FirmwareType_FirmwareType_EFI,
            FirmwareType::EFI32 => raw::FirmwareType_FirmwareType_EFI32,
            FirmwareType::EFI64 => raw::FirmwareType_FirmwareType_EFI64,
            FirmwareType::EFIDUAL => raw::FirmwareType_FirmwareType_EFIDUAL,
        }
    }
}

impl Display for FirmwareType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
