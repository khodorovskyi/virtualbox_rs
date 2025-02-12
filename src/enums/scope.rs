use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
///Scope of the operation.
///
/// A generic enumeration used in various methods to define the action or argument scope.
#[derive(Debug)]
pub enum Scope {
    Global,
    Machine,
    Session,
}

impl From<u32> for Scope {
    fn from(value: u32) -> Self {
        match value {
            raw::Scope_Scope_Session => Scope::Session,
            raw::Scope_Scope_Machine => Scope::Machine,
            raw::Scope_Scope_Global => Scope::Global,
            _ => {
                error!("Unknown Scope/ Scope: {}", value);
                Scope::Global
            }
        }
    }
}

impl Into<u32> for Scope {
    fn into(self) -> u32 {
        match self {
            Scope::Global => raw::Scope_Scope_Global,
            Scope::Machine => raw::Scope_Scope_Machine,
            Scope::Session => raw::Scope_Scope_Session,
        }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}", self))
    }
}
