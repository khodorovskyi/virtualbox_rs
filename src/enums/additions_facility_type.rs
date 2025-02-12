use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
/// Guest Additions facility IDs.
#[derive(Debug)]
pub enum AdditionsFacilityType {
    /// No/invalid facility.
    None,
    /// VirtualBox base driver (VBoxGuest).
    VBoxGuestDriver,
    /// Auto-logon modules (VBoxGINA, VBoxCredProv, pam_vbox).
    AutoLogon,
    /// VirtualBox system service (VBoxService).
    VBoxService,
    /// VirtualBox desktop integration (VBoxTray on Windows, VBoxClient on non-Windows).
    VBoxTrayClient,
    /// Seamless guest desktop integration.
    Seamless,
    /// Guest graphics mode.
    ///
    /// If not enabled, seamless rendering will not work, resize hints are not immediately acted on and guest display resizes are probably not initiated by the Guest Additions.
    Graphics,
    /// Guest supports monitor hotplug.
    MonitorAttach,
    /// All facilities selected.
    All,
}

impl From<u32> for AdditionsFacilityType {
    fn from(value: u32) -> Self {
        match value {
            raw::AdditionsFacilityType_AdditionsFacilityType_None => AdditionsFacilityType::None,
            raw::AdditionsFacilityType_AdditionsFacilityType_VBoxGuestDriver => {
                AdditionsFacilityType::VBoxGuestDriver
            }
            raw::AdditionsFacilityType_AdditionsFacilityType_AutoLogon => {
                AdditionsFacilityType::AutoLogon
            }
            raw::AdditionsFacilityType_AdditionsFacilityType_VBoxService => {
                AdditionsFacilityType::VBoxService
            }
            raw::AdditionsFacilityType_AdditionsFacilityType_VBoxTrayClient => {
                AdditionsFacilityType::VBoxTrayClient
            }
            raw::AdditionsFacilityType_AdditionsFacilityType_Seamless => {
                AdditionsFacilityType::Seamless
            }
            raw::AdditionsFacilityType_AdditionsFacilityType_Graphics => {
                AdditionsFacilityType::Graphics
            }
            raw::AdditionsFacilityType_AdditionsFacilityType_MonitorAttach => {
                AdditionsFacilityType::MonitorAttach
            }
            raw::AdditionsFacilityType_AdditionsFacilityType_All => AdditionsFacilityType::All,
            _ => {
                error!("Unknown AdditionsFacilityTyp/ Type: {}", value);
                AdditionsFacilityType::None
            }
        }
    }
}

impl Into<u32> for AdditionsFacilityType {
    fn into(self) -> u32 {
        match self {
            AdditionsFacilityType::None => raw::AdditionsFacilityType_AdditionsFacilityType_None,
            AdditionsFacilityType::VBoxGuestDriver => {
                raw::AdditionsFacilityType_AdditionsFacilityType_VBoxGuestDriver
            }
            AdditionsFacilityType::AutoLogon => {
                raw::AdditionsFacilityType_AdditionsFacilityType_AutoLogon
            }
            AdditionsFacilityType::VBoxService => {
                raw::AdditionsFacilityType_AdditionsFacilityType_VBoxService
            }
            AdditionsFacilityType::VBoxTrayClient => {
                raw::AdditionsFacilityType_AdditionsFacilityType_VBoxTrayClient
            }
            AdditionsFacilityType::Seamless => {
                raw::AdditionsFacilityType_AdditionsFacilityType_Seamless
            }
            AdditionsFacilityType::Graphics => {
                raw::AdditionsFacilityType_AdditionsFacilityType_Graphics
            }
            AdditionsFacilityType::MonitorAttach => {
                raw::AdditionsFacilityType_AdditionsFacilityType_MonitorAttach
            }
            AdditionsFacilityType::All => raw::AdditionsFacilityType_AdditionsFacilityType_All,
        }
    }
}

impl Display for AdditionsFacilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
