use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Type of pointing device used in a virtual machine.
#[derive(Debug, Eq, PartialEq)]
pub enum PointingHIDType {
    /// No mouse.
    None,
    /// PS/2 auxiliary device, a.k.a.
    ///
    /// mouse.
    PS2Mouse,
    /// USB mouse (relative pointer).
    USBMouse,
    /// USB tablet (absolute pointer).
    ///
    /// Also enables a relative USB mouse in addition.
    USBTablet,
    /// Combined device, working as PS/2 or USB mouse, depending on guest behavior.
    ///
    /// Using this device can have negative performance implications.
    ComboMouse,
    /// USB multi-touch device, just touchscreen.
    ///
    /// It is a specific mode of the USB tablet and also enables the mouse device.
    USBMultiTouch,
    /// USB multi-touch device, touchscreen plus touchpad.
    ///
    /// It also enables the mouse device.
    USBMultiTouchScreenPlusPad,
}
impl Into<u32> for PointingHIDType {
    fn into(self) -> u32 {
        match self {
            PointingHIDType::PS2Mouse => raw::PointingHIDType_PointingHIDType_PS2Mouse,
            PointingHIDType::USBMouse => raw::PointingHIDType_PointingHIDType_USBMouse,
            PointingHIDType::USBTablet => raw::PointingHIDType_PointingHIDType_USBTablet,
            PointingHIDType::ComboMouse => raw::PointingHIDType_PointingHIDType_ComboMouse,
            #[cfg(not(is_v_6_1))]
            PointingHIDType::USBMultiTouch => raw::PointingHIDType_PointingHIDType_USBMultiTouch,
            #[cfg(not(is_v_6_1))]
            PointingHIDType::USBMultiTouchScreenPlusPad => {
                raw::PointingHIDType_PointingHIDType_USBMultiTouchScreenPlusPad
            }
            _ => raw::PointingHIDType_PointingHIDType_None,
        }
    }
}

impl From<u32> for PointingHIDType {
    fn from(value: u32) -> Self {
        match value {
            raw::PointingHIDType_PointingHIDType_None => PointingHIDType::None,
            raw::PointingHIDType_PointingHIDType_PS2Mouse => PointingHIDType::PS2Mouse,
            raw::PointingHIDType_PointingHIDType_USBMouse => PointingHIDType::USBMouse,
            raw::PointingHIDType_PointingHIDType_USBTablet => PointingHIDType::USBTablet,
            raw::PointingHIDType_PointingHIDType_ComboMouse => PointingHIDType::ComboMouse,
            #[cfg(not(is_v_6_1))]
            raw::PointingHIDType_PointingHIDType_USBMultiTouch => PointingHIDType::USBMultiTouch,
            #[cfg(not(is_v_6_1))]
            raw::PointingHIDType_PointingHIDType_USBMultiTouchScreenPlusPad => {
                PointingHIDType::USBMultiTouchScreenPlusPad
            }
            _ => {
                error!("PointingHIDType::from. Unknown type: {}", value);
                PointingHIDType::None
            }
        }
    }
}

impl Display for PointingHIDType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
