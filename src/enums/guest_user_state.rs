use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// State a guest user has been changed to.
#[derive(Debug)]
pub enum GuestUserState {
    /// Unknown state.
    ///
    /// Not being used.
    Unknown,
    /// A guest user has been successfully logged into the guest OS.
    LoggedIn,
    /// A guest user has been successfully logged out of the guest OS.
    LoggedOut,
    /// A guest user has locked its account.
    ///
    /// This might include running a password-protected screensaver in the guest.
    Locked,
    /// A guest user has unlocked its account.
    Unlocked,
    /// A guest user has been disabled by the guest OS.
    Disabled,
    /// A guest user currently is not using the guest OS.
    Idle,
    /// A guest user continued using the guest OS after being idle.
    InUse,
    /// A guest user has been successfully created.
    Created,
    /// A guest user has been successfully deleted.
    Deleted,
    /// To guest OS has changed the session of a user.
    SessionChanged,
    /// To guest OS has changed the authentication credentials of a user.
    ///
    /// This might include changed passwords and authentication types.
    CredentialsChanged,
    /// To guest OS has changed the role of a user permanently, e.g.
    ///
    /// granting / denying administrative rights.
    RoleChanged,
    /// To guest OS has added a user to a specific user group.
    GroupAdded,
    /// To guest OS has removed a user from a specific user group.
    GroupRemoved,
    /// To guest OS temporarily has elevated a user to perform a certain task.
    Elevated,
}

impl From<u32> for GuestUserState {
    fn from(value: u32) -> Self {
        match value {
            raw::GuestUserState_GuestUserState_Unknown => GuestUserState::Unknown,
            raw::GuestUserState_GuestUserState_LoggedIn => GuestUserState::LoggedIn,
            raw::GuestUserState_GuestUserState_LoggedOut => GuestUserState::LoggedOut,
            raw::GuestUserState_GuestUserState_Locked => GuestUserState::Locked,
            raw::GuestUserState_GuestUserState_Unlocked => GuestUserState::Unlocked,
            raw::GuestUserState_GuestUserState_Disabled => GuestUserState::Disabled,
            raw::GuestUserState_GuestUserState_Idle => GuestUserState::Idle,
            raw::GuestUserState_GuestUserState_InUse => GuestUserState::InUse,
            raw::GuestUserState_GuestUserState_Created => GuestUserState::Created,
            raw::GuestUserState_GuestUserState_Deleted => GuestUserState::Deleted,
            raw::GuestUserState_GuestUserState_SessionChanged => GuestUserState::SessionChanged,
            raw::GuestUserState_GuestUserState_CredentialsChanged => {
                GuestUserState::CredentialsChanged
            }
            raw::GuestUserState_GuestUserState_RoleChanged => GuestUserState::RoleChanged,
            raw::GuestUserState_GuestUserState_GroupAdded => GuestUserState::GroupAdded,
            raw::GuestUserState_GuestUserState_GroupRemoved => GuestUserState::GroupRemoved,
            raw::GuestUserState_GuestUserState_Elevated => GuestUserState::Elevated,
            _ => {
                error!("Unknown GuestUserState/ State: {}", value);
                GuestUserState::Unknown
            }
        }
    }
}

impl Into<u32> for GuestUserState {
    fn into(self) -> u32 {
        match self {
            GuestUserState::Unknown => raw::GuestUserState_GuestUserState_Unknown,
            GuestUserState::LoggedIn => raw::GuestUserState_GuestUserState_LoggedIn,
            GuestUserState::LoggedOut => raw::GuestUserState_GuestUserState_LoggedOut,
            GuestUserState::Locked => raw::GuestUserState_GuestUserState_Locked,
            GuestUserState::Unlocked => raw::GuestUserState_GuestUserState_Unlocked,
            GuestUserState::Disabled => raw::GuestUserState_GuestUserState_Disabled,
            GuestUserState::Idle => raw::GuestUserState_GuestUserState_Idle,
            GuestUserState::InUse => raw::GuestUserState_GuestUserState_InUse,
            GuestUserState::Created => raw::GuestUserState_GuestUserState_Created,
            GuestUserState::Deleted => raw::GuestUserState_GuestUserState_Deleted,
            GuestUserState::SessionChanged => raw::GuestUserState_GuestUserState_SessionChanged,
            GuestUserState::CredentialsChanged => {
                raw::GuestUserState_GuestUserState_CredentialsChanged
            }
            GuestUserState::RoleChanged => raw::GuestUserState_GuestUserState_RoleChanged,
            GuestUserState::GroupAdded => raw::GuestUserState_GuestUserState_GroupAdded,
            GuestUserState::GroupRemoved => raw::GuestUserState_GuestUserState_GroupRemoved,
            GuestUserState::Elevated => raw::GuestUserState_GuestUserState_Elevated,
        }
    }
}

impl Display for GuestUserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
