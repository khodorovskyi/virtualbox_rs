use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Guest Additions update flags.
#[derive(Debug, Copy, Clone)]
pub enum AdditionsUpdateFlag {
    /// No flag set.
    None,
    /// Starts the regular updating process and waits until the actual Guest Additions update inside the guest was started.
    ///
    /// This can be necessary due to needed interaction with the guest OS during the installation phase.
    WaitForUpdateStartOnly,
}

impl From<u32> for AdditionsUpdateFlag {
    fn from(value: u32) -> Self {
        match value {
            raw::AdditionsUpdateFlag_AdditionsUpdateFlag_None => AdditionsUpdateFlag::None,
            raw::AdditionsUpdateFlag_AdditionsUpdateFlag_WaitForUpdateStartOnly => {
                AdditionsUpdateFlag::WaitForUpdateStartOnly
            }
            _ => {
                error!("Unknown AdditionsUpdateFlag. Flag: {}", value);
                AdditionsUpdateFlag::None
            }
        }
    }
}

impl Into<u32> for AdditionsUpdateFlag {
    fn into(self) -> u32 {
        match self {
            AdditionsUpdateFlag::None => raw::AdditionsUpdateFlag_AdditionsUpdateFlag_None,
            AdditionsUpdateFlag::WaitForUpdateStartOnly => {
                raw::AdditionsUpdateFlag_AdditionsUpdateFlag_WaitForUpdateStartOnly
            }
        }
    }
}

impl Display for AdditionsUpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
