#[cfg(is_v_7_1)]
use log::error;
use std::fmt::Display;
#[cfg(is_v_7_1)]
use vbox_raw::sys_lib as raw;

/// Graphics features.
#[derive(Debug, PartialEq)]
pub enum GraphicsFeature {
    /// No feature specified, invalid.
    None,
    /// 2D video acceleration.
    Acceleration2DVideo,
    /// 3D acceleration.
    Acceleration3D,
}

#[cfg(is_v_7_1)]
impl From<u32> for GraphicsFeature {
    fn from(value: u32) -> Self {
        match value {
            raw::GraphicsFeature_GraphicsFeature_None => GraphicsFeature::None,
            raw::GraphicsFeature_GraphicsFeature_Acceleration2DVideo => GraphicsFeature::Acceleration2DVideo,
            raw::GraphicsFeature_GraphicsFeature_Acceleration3D => GraphicsFeature::Acceleration3D,
            _ => {
                error!("Acceleration3D GraphicsFeature. GraphicsFeature: {}", value);
                GraphicsFeature::Acceleration3D
            }
        }
    }
}

#[cfg(is_v_7_1)]
impl Into<u32> for GraphicsFeature {
    fn into(self) -> u32 {
        match self {
            GraphicsFeature::None => raw::GraphicsFeature_GraphicsFeature_None,
            GraphicsFeature::Acceleration2DVideo => raw::GraphicsFeature_GraphicsFeature_Acceleration2DVideo,
            GraphicsFeature::Acceleration3D => raw::GraphicsFeature_GraphicsFeature_Acceleration3D,
        }
    }
}

#[cfg(not(is_v_7_1))]
impl From<u32> for GraphicsFeature {
    fn from(_value: u32) -> Self {
        GraphicsFeature::None
    }
}
#[cfg(not(is_v_7_1))]
impl Into<u32> for GraphicsFeature {
    fn into(self) -> u32 {
        0
    }
}

impl Display for GraphicsFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
