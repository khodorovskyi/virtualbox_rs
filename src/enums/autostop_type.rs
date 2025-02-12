#[cfg(doc)]
use crate::Machine;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Autostop types, used with [`Machine::get_autostop_type`].
#[derive(Debug, Copy, Clone)]
pub enum AutostopType {
    /// Stopping the VM during system shutdown is disabled.
    Disabled,

    /// The state of the VM will be saved when the system shuts down.
    SaveState,

    /// The VM is powered off when the system shuts down.
    PowerOff,

    /// An ACPI shutdown event is generated.
    AcpiShutdown,
}

impl From<u32> for AutostopType {
    fn from(value: u32) -> Self {
        match value {
            raw::AutostopType_AutostopType_Disabled => AutostopType::Disabled,
            raw::AutostopType_AutostopType_SaveState => AutostopType::SaveState,
            raw::AutostopType_AutostopType_PowerOff => AutostopType::PowerOff,
            raw::AutostopType_AutostopType_AcpiShutdown => AutostopType::AcpiShutdown,
            _ => {
                error!("Unknown AutostopType. AutostopType: {}", value);
                AutostopType::Disabled
            }
        }
    }
}

impl Into<u32> for AutostopType {
    fn into(self) -> u32 {
        match self {
            AutostopType::Disabled => raw::AutostopType_AutostopType_Disabled,
            AutostopType::SaveState => raw::AutostopType_AutostopType_SaveState,
            AutostopType::PowerOff => raw::AutostopType_AutostopType_PowerOff,
            AutostopType::AcpiShutdown => raw::AutostopType_AutostopType_AcpiShutdown,
        }
    }
}

impl Display for AutostopType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
