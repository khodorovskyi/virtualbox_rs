use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Type of keyboard device used in a virtual machine.
#[derive(Debug, Eq, PartialEq)]
pub enum KeyboardHIDType {
    /// No keyboard.
    None,
    /// PS/2 keyboard.
    PS2Keyboard,
    /// USB keyboard.
    USBKeyboard,
    /// Combined device, working as PS/2 or USB keyboard, depending on guest behavior.
    ///
    /// Using of such device can have negative performance implications.
    ComboKeyboard,
}
impl Into<u32> for KeyboardHIDType {
    fn into(self) -> u32 {
        match self {
            KeyboardHIDType::PS2Keyboard => raw::KeyboardHIDType_KeyboardHIDType_PS2Keyboard,
            KeyboardHIDType::USBKeyboard => raw::KeyboardHIDType_KeyboardHIDType_USBKeyboard,
            KeyboardHIDType::ComboKeyboard => raw::KeyboardHIDType_KeyboardHIDType_ComboKeyboard,
            _ => raw::KeyboardHIDType_KeyboardHIDType_None,
        }
    }
}

impl From<u32> for KeyboardHIDType {
    fn from(value: u32) -> Self {
        match value {
            raw::KeyboardHIDType_KeyboardHIDType_None => KeyboardHIDType::None,
            raw::KeyboardHIDType_KeyboardHIDType_PS2Keyboard => KeyboardHIDType::PS2Keyboard,
            raw::KeyboardHIDType_KeyboardHIDType_USBKeyboard => KeyboardHIDType::USBKeyboard,
            raw::KeyboardHIDType_KeyboardHIDType_ComboKeyboard => KeyboardHIDType::ComboKeyboard,
            _ => {
                error!("KeyboardHIDType::from. Unknown type: {}", value);
                KeyboardHIDType::None
            }
        }
    }
}

impl Display for KeyboardHIDType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
