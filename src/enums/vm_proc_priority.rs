use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Virtual machine process priorities.
#[derive(Debug, Eq, PartialEq)]
pub enum VMProcPriority {
    /// Invalid priority, do not use.
    Invalid,
    /// Default process priority determined by the OS.
    Default,
    /// Assumes a scheduling policy which puts the process at the default priority and with all threads at the same priority.
    Flat,
    /// Assumes a scheduling policy which puts the process mostly below the default priority of the host OS.
    Low,
    /// Assume a scheduling policy which shares the CPU resources fairly with other processes running with the default priority of the host OS.
    Normal,
    /// Assumes a scheduling policy which puts the task above the default priority of the host OS.
    ///
    /// This policy might easily cause other tasks in the system to starve.
    High,
}
impl Into<u32> for VMProcPriority {
    fn into(self) -> u32 {
        match self {
            VMProcPriority::Default => raw::VMProcPriority_VMProcPriority_Default,
            VMProcPriority::Flat => raw::VMProcPriority_VMProcPriority_Flat,
            VMProcPriority::Low => raw::VMProcPriority_VMProcPriority_Low,
            VMProcPriority::Normal => raw::VMProcPriority_VMProcPriority_Normal,
            VMProcPriority::High => raw::VMProcPriority_VMProcPriority_High,
            _ => raw::VMProcPriority_VMProcPriority_Invalid,
        }
    }
}

impl From<u32> for VMProcPriority {
    fn from(value: u32) -> Self {
        match value {
            raw::VMProcPriority_VMProcPriority_Invalid => VMProcPriority::Invalid,
            raw::VMProcPriority_VMProcPriority_Default => VMProcPriority::Default,
            raw::VMProcPriority_VMProcPriority_Flat => VMProcPriority::Flat,
            raw::VMProcPriority_VMProcPriority_Low => VMProcPriority::Low,
            raw::VMProcPriority_VMProcPriority_Normal => VMProcPriority::Normal,
            raw::VMProcPriority_VMProcPriority_High => VMProcPriority::High,
            _ => {
                error!("VMProcPriority::from. Unknown type: {}", value);
                VMProcPriority::Invalid
            }
        }
    }
}

impl Display for VMProcPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
