use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
#[derive(Debug, Eq, PartialEq)]
pub enum SessionState {
    /// Null value (never used by the API).
    Null,
    /// In [Machine::get_session_state](crate::Machine::get_session_state), this means that the machine is not locked for any sessions.
    ///
    /// In [`Session::get_state`](crate::Session::get_state), this means that no machine is currently locked for this session.
    Unlocked,
    ///In [`Machine::get_session_state`](crate::Machine::get_session_state), this means that the machine is currently locked for a session,
    /// whose process identifier can then be found in the IMachine::sessionPID attribute.
    ///
    /// In [`Session::get_state`](crate::Session::get_state), this means that a machine is currently locked for this session,
    /// and the mutable machine object can be found in the [`Session::get_machine`](crate::Session::get_machine) attribute
    /// (see [`Machine::lock_machine`](crate::Machine::lock_machine) for details).
    Locked,
    /// A new process is being spawned for the machine as a result of
    /// [`Machine::launch_vm_process`](crate::Machine::launch_vm_process) call.
    ///
    /// This state also occurs as a short transient state during an
    /// [`Machine::lock_machine`](crate::Machine::lock_machine) call.
    Spawning,
    /// The session is being unlocked.
    Unlocking,
}

impl From<u32> for SessionState {
    fn from(value: u32) -> Self {
        match value {
            raw::SessionState_SessionState_Unlocked => SessionState::Unlocked,
            raw::SessionState_SessionState_Locked => SessionState::Locked,
            raw::SessionState_SessionState_Spawning => SessionState::Spawning,
            raw::SessionState_SessionState_Unlocking => SessionState::Unlocking,
            _ => {
                error!("SessionState::from. Unknown state: {}", value);
                SessionState::Null
            }
        }
    }
}

impl Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
