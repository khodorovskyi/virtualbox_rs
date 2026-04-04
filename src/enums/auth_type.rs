use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// VirtualBox authentication type
#[derive(Debug)]
pub enum AuthType {
    /// Null value, also means "no authentication".
    Null,
    External,
    Guest,
}

impl From<u32> for AuthType {
    fn from(value: u32) -> Self {
        match value {
            raw::AuthType_AuthType_Null => AuthType::Null,
            raw::AuthType_AuthType_External => AuthType::External,
            raw::AuthType_AuthType_Guest => AuthType::Guest,
            _ => {
                error!("Unknown AuthType. Type: {}", value);
                AuthType::Null
            }
        }
    }
}

impl Into<u32> for AuthType {
    fn into(self) -> u32 {
        match self {
            AuthType::Null => raw::AuthType_AuthType_Null,
            AuthType::External => raw::AuthType_AuthType_External,
            AuthType::Guest => raw::AuthType_AuthType_Guest,
        }
    }
}

impl Display for AuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
