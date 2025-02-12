#![allow(unreachable_patterns)]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

#[derive(Debug, Eq, PartialEq)]
pub enum MachineState {
    /// Null value (never used by the API).
    Null,
    /// The machine is not running and has no saved execution state; it has
    /// either never been started or been shut down successfully.
    PoweredOff,
    /// The machine is not currently running, but the execution state of the machine
    /// has been saved to an external file when it was running, from where
    /// it can be resumed.
    Saved,
    /// The machine was teleported to a different host (or process) and then
    /// powered off. Take care when powering it on again may corrupt resources
    /// it shares with the teleportation target (e.g. disk and network).
    Teleported,
    /// The process running the machine has terminated abnormally. This may
    /// indicate a crash of the VM process in host execution context, or
    /// the VM process has been terminated externally.
    Aborted,
    /// A machine in the @c Saved stated has terminated abnormally before
    /// reaching the @c Running state. Similar to the @c Aborted state,
    /// this may indicate a crash of the VM process in host execution
    /// context, or the VM process has been terminated externally.
    /// <div class="warning">
    ///  This state only exists for versions 7 and above.
    ///  To ensure compatibility with earlier versions, this field has been retained, but the machine will never transition to this state.
    /// </div>
    AbortedSaved,
    /// The machine is currently being executed.
    Running,
    /// Execution of the machine has been paused.
    Paused,
    /// Execution of the machine has reached the "Guru Meditation"
    /// condition. This indicates a severe error in the hypervisor itself.
    Stuck,
    /// The machine is about to be teleported to a different host or process.
    /// It is possible to pause a machine in this state, but it will go to the
    /// TeleportingPausedVM state and it will not be
    /// possible to resume it again unless the teleportation fails
    Teleporting,
    /// A live snapshot is being taken. The machine is running normally, but
    /// some of the runtime configuration options are inaccessible. Also, if
    /// paused while in this state it will transition to
    /// @c OnlineSnapshotting and it will not be resume the
    /// execution until the snapshot operation has completed.
    LiveSnapshotting,
    /// Machine is being started after powering it on from a
    /// zero execution state.
    Starting,
    /// Machine is being normally stopped powering it off, or after the guest OS
    /// has initiated a shutdown sequence.
    Stopping,
    /// Machine is saving its execution state to a file.
    Saving,
    /// Execution state of the machine is being restored from a file
    /// after powering it on from either the @c Saved or @c AbortedSaved
    /// execution state.
    Restoring,
    /// The machine is being teleported to another host or process, but it is
    /// not running. This is the paused variant of the
    /// Teleporting state
    TeleportingPausedVM,
    /// Teleporting the machine state in from another host or process.
    TeleportingIn,
    /// Like DeletingSnapshot, but the merging of media is ongoing in
    ///  he background while the machine is running.
    DeletingSnapshotOnline,
    /// Like DeletingSnapshotOnline, but the machine was paused when the
    /// merging of differencing media was started.
    DeletingSnapshotPaused,
    /// Like LiveSnapshotting, but the machine was paused when the
    /// merging of differencing media was started.
    OnlineSnapshotting,
    /// A machine snapshot is being restored; this typically does not take long.
    RestoringSnapshot,
    /// A machine snapshot is being deleted; this can take a long time since this
    /// may require merging differencing media. This value indicates that the
    /// machine is not running while the snapshot is being deleted.
    DeletingSnapshot,
    /// Lengthy setup operation is in progress.
    SettingUp,
    /// Taking an (offline) snapshot.
    Snapshotting,
    /// Pseudo-state: first online state (for use in relational expressions).
    FirstOnline,
    /// Pseudo-state: last online state (for use in relational expressions).
    LastOnline,
    /// Pseudo-state: first transient state (for use in relational expressions).
    FirstTransient,
    /// Pseudo-state: last transient state (for use in relational expressions).
    LastTransient,
}

impl From<u32> for MachineState {
    fn from(value: u32) -> Self {
        match value {
            raw::MachineState_MachineState_Null => MachineState::Null,
            raw::MachineState_MachineState_PoweredOff => MachineState::PoweredOff,
            raw::MachineState_MachineState_Saved => MachineState::Saved,
            raw::MachineState_MachineState_Teleported => MachineState::Teleported,
            raw::MachineState_MachineState_Aborted => MachineState::Aborted,
            #[cfg(not(is_v_6_1))]
            raw::MachineState_MachineState_AbortedSaved => MachineState::AbortedSaved,
            raw::MachineState_MachineState_Running => MachineState::Running,
            raw::MachineState_MachineState_Paused => MachineState::Paused,
            raw::MachineState_MachineState_Stuck => MachineState::Stuck,
            raw::MachineState_MachineState_Teleporting => MachineState::Teleporting,
            raw::MachineState_MachineState_LiveSnapshotting => MachineState::LiveSnapshotting,
            raw::MachineState_MachineState_Starting => MachineState::Starting,
            raw::MachineState_MachineState_Stopping => MachineState::Stopping,
            raw::MachineState_MachineState_Saving => MachineState::Saving,
            raw::MachineState_MachineState_Restoring => MachineState::Restoring,
            raw::MachineState_MachineState_TeleportingPausedVM => MachineState::TeleportingPausedVM,
            raw::MachineState_MachineState_TeleportingIn => MachineState::TeleportingIn,
            raw::MachineState_MachineState_DeletingSnapshotOnline => {
                MachineState::DeletingSnapshotOnline
            }
            raw::MachineState_MachineState_DeletingSnapshotPaused => {
                MachineState::DeletingSnapshotPaused
            }
            raw::MachineState_MachineState_OnlineSnapshotting => MachineState::OnlineSnapshotting,
            raw::MachineState_MachineState_RestoringSnapshot => MachineState::RestoringSnapshot,
            raw::MachineState_MachineState_DeletingSnapshot => MachineState::DeletingSnapshot,
            raw::MachineState_MachineState_SettingUp => MachineState::SettingUp,
            raw::MachineState_MachineState_Snapshotting => MachineState::Snapshotting,
            raw::MachineState_MachineState_FirstOnline => MachineState::FirstOnline,
            raw::MachineState_MachineState_LastOnline => MachineState::LastOnline,
            raw::MachineState_MachineState_FirstTransient => MachineState::FirstTransient,
            raw::MachineState_MachineState_LastTransient => MachineState::LastTransient,
            _ => {
                error!("MachineState::from. Unknown state: {}", value);
                MachineState::Null
            }
        }
    }
}

impl Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
