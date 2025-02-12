use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

#[derive(Debug, Eq, PartialEq)]
pub enum SessionType {
    /// Placeholder value, do not use when obtaining a lock.
    Null,
    /// Request only a shared lock for remote-controlling the machine.
    ///
    /// Such a lock allows changing certain VM settings which can be safely modified for a running VM.
    Shared,
    /// Lock the machine for writing.
    ///
    /// This requests an exclusive lock, i.e. there cannot be any other API client holding any type of lock for this VM concurrently. Remember that a VM process counts as an API client which implicitly holds the equivalent of a shared lock during the entire VM runtime.
    Write,
    /// Lock the machine for writing, and create objects necessary for running a VM in this process.
    VM,
}

impl From<u32> for SessionType {
    fn from(value: u32) -> Self {
        match value {
            raw::LockType_LockType_Null => SessionType::Null,
            raw::LockType_LockType_Shared => SessionType::Shared,
            raw::LockType_LockType_Write => SessionType::Write,
            raw::LockType_LockType_VM => SessionType::VM,
            _ => {
                error!("SessionType::from. Unknown type: {}", value);
                SessionType::Null
            }
        }
    }
}

impl Into<u32> for SessionType {
    fn into(self) -> u32 {
        match self {
            SessionType::Null => raw::LockType_LockType_Null,
            SessionType::Shared => raw::LockType_LockType_Shared,
            SessionType::Write => raw::LockType_LockType_Write,
            SessionType::VM => raw::LockType_LockType_VM,
        }
    }
}

impl Display for SessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
