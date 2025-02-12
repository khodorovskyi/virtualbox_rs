use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Access mode for opening files.
#[derive(Debug, Copy, Clone)]
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

impl From<u32> for AccessMode {
    fn from(value: u32) -> Self {
        match value {
            raw::AccessMode_AccessMode_ReadOnly => AccessMode::ReadOnly,
            raw::AccessMode_AccessMode_ReadWrite => AccessMode::ReadWrite,
            _ => {
                error!("Unknown AdditionsUpdateFlag. Flag: {}", value);
                AccessMode::ReadOnly
            }
        }
    }
}

impl Into<u32> for AccessMode {
    fn into(self) -> u32 {
        match self {
            AccessMode::ReadOnly => raw::AccessMode_AccessMode_ReadOnly,
            AccessMode::ReadWrite => raw::AccessMode_AccessMode_ReadWrite,
        }
    }
}

impl Display for AccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
