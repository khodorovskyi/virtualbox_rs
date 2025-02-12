#[cfg(doc)]
use crate::Machine;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Clone mode, used with [`Machine::clone_to`].
#[derive(Debug, Copy, Clone)]
pub enum CloneMode {
    /// Clone the state of the selected machine.
    MachineState,
    /// Clone the state of the selected machine and its child snapshots if present.
    MachineAndChildStates,
    /// Clone all states (including all snapshots) of the machine, regardless of the machine object used.
    AllStates,
}

impl From<u32> for CloneMode {
    fn from(value: u32) -> Self {
        match value {
            raw::CloneMode_CloneMode_MachineState => CloneMode::MachineState,
            raw::CloneMode_CloneMode_MachineAndChildStates => CloneMode::MachineAndChildStates,
            raw::CloneMode_CloneMode_AllStates => CloneMode::AllStates,
            _ => {
                error!("Unknown CloneMode. CloneMode: {}", value);
                CloneMode::MachineState
            }
        }
    }
}

impl Into<u32> for CloneMode {
    fn into(self) -> u32 {
        match self {
            CloneMode::MachineState => raw::CloneMode_CloneMode_MachineState,
            CloneMode::MachineAndChildStates => raw::CloneMode_CloneMode_MachineAndChildStates,
            CloneMode::AllStates => raw::CloneMode_CloneMode_AllStates,
        }
    }
}

impl Display for CloneMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
