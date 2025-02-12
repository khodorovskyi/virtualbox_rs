use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// The USB controller type.
#[derive(Debug, Eq, PartialEq)]
pub enum USBControllerType {
    /// null value.
    ///
    /// Never used by the API.
    Null,
    OHCI,
    EHCI,
    XHCI,
    /// Last element (invalid).
    ///
    /// Used for parameter checks.
    Last,
}
impl Into<u32> for USBControllerType {
    fn into(self) -> u32 {
        match self {
            USBControllerType::OHCI => raw::USBControllerType_USBControllerType_OHCI,
            USBControllerType::EHCI => raw::USBControllerType_USBControllerType_EHCI,
            USBControllerType::XHCI => raw::USBControllerType_USBControllerType_XHCI,
            USBControllerType::Last => raw::USBControllerType_USBControllerType_Last,
            _ => raw::USBControllerType_USBControllerType_Null,
        }
    }
}

impl From<u32> for USBControllerType {
    fn from(value: u32) -> Self {
        match value {
            raw::USBControllerType_USBControllerType_Null => USBControllerType::Null,
            raw::USBControllerType_USBControllerType_OHCI => USBControllerType::OHCI,
            raw::USBControllerType_USBControllerType_EHCI => USBControllerType::EHCI,
            raw::USBControllerType_USBControllerType_XHCI => USBControllerType::XHCI,
            raw::USBControllerType_USBControllerType_Last => USBControllerType::Last,
            _ => {
                error!("USBControllerType::from. Unknown type: {}", value);
                USBControllerType::Null
            }
        }
    }
}

impl Display for USBControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
