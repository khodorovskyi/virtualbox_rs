use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Guest Additions run level type.
#[derive(Debug)]
pub enum AdditionsRunLevelType {
    /// Guest Additions are not loaded.
    None,
    /// Guest drivers are loaded.
    System,
    /// Common components (such as application services) are loaded.
    Userland,
    /// Per-user desktop components are loaded.
    Desktop,
}

impl From<u32> for AdditionsRunLevelType {
    fn from(value: u32) -> Self {
        match value {
            raw::AdditionsRunLevelType_AdditionsRunLevelType_None => AdditionsRunLevelType::None,
            raw::AdditionsRunLevelType_AdditionsRunLevelType_System => {
                AdditionsRunLevelType::System
            }
            raw::AdditionsRunLevelType_AdditionsRunLevelType_Userland => {
                AdditionsRunLevelType::Userland
            }
            raw::AdditionsRunLevelType_AdditionsRunLevelType_Desktop => {
                AdditionsRunLevelType::Desktop
            }
            _ => {
                error!("Unknown AdditionsRunLevelType. Type: {}", value);
                AdditionsRunLevelType::None
            }
        }
    }
}

impl Into<u32> for AdditionsRunLevelType {
    fn into(self) -> u32 {
        match self {
            AdditionsRunLevelType::None => raw::AdditionsRunLevelType_AdditionsRunLevelType_None,
            AdditionsRunLevelType::System => {
                raw::AdditionsRunLevelType_AdditionsRunLevelType_System
            }
            AdditionsRunLevelType::Userland => {
                raw::AdditionsRunLevelType_AdditionsRunLevelType_Userland
            }
            AdditionsRunLevelType::Desktop => {
                raw::AdditionsRunLevelType_AdditionsRunLevelType_Desktop
            }
        }
    }
}

impl Display for AdditionsRunLevelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
