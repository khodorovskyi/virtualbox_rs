use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
#[cfg(doc)]
use crate::GraphicsAdapter;

/// Graphics controller type, used with [`GraphicsAdapter::get_graphics_controller_type`].
#[derive(Debug, Clone, Copy)]
pub enum GraphicsControllerType {
    /// Reserved value, invalid.
    Null,
    /// VirtualBox VGA device.
    VBoxVGA,
    /// VMware SVGA II device.
    VMSVGA,
    /// VirtualBox VGA device with VMware SVGA II extensions.
    VBoxSVGA,
}
impl From<u32> for GraphicsControllerType {
    fn from(value: u32) -> Self {
        match value {
            raw::GraphicsControllerType_GraphicsControllerType_Null => GraphicsControllerType::Null,
            raw::GraphicsControllerType_GraphicsControllerType_VBoxVGA => GraphicsControllerType::VBoxVGA,
            raw::GraphicsControllerType_GraphicsControllerType_VMSVGA => GraphicsControllerType::VMSVGA,
            raw::GraphicsControllerType_GraphicsControllerType_VBoxSVGA => GraphicsControllerType::VBoxSVGA,
            _ => {
                error!("Unknown GraphicsControllerType. Channel: {}", value);
                GraphicsControllerType::Null
            }
        }
    }
}
impl Into<u32> for GraphicsControllerType {
    fn into(self) -> u32 {
        match self {
            GraphicsControllerType::Null => raw::GraphicsControllerType_GraphicsControllerType_Null,
            GraphicsControllerType::VBoxVGA => raw::GraphicsControllerType_GraphicsControllerType_VBoxVGA,
            GraphicsControllerType::VMSVGA => raw::GraphicsControllerType_GraphicsControllerType_VMSVGA,
            GraphicsControllerType::VBoxSVGA => raw::GraphicsControllerType_GraphicsControllerType_VBoxSVGA,
        }
    }
}

impl Display for GraphicsControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
