use std::fmt::Display;
use vbox_raw::sys_lib as raw;

#[derive(Debug, Eq, PartialEq)]
pub enum BitmapFormat {
    /// Unknown buffer format (the user may not assume any particular format of
    ///         the buffer).
    Opaque,
    /// Generic BGR format without alpha channel.
    BGR,
    /// 4 bytes per pixel: B, G, R, 0.
    BGR0,
    /// 4 bytes per pixel: B, G, R, A.
    BGRA,
    /// 4 bytes per pixel: R, G, B, A.
    RGBA,
    /// PNG image.
    PNG,
    /// JPEG image.
    JPEG,
}

impl Into<u32> for BitmapFormat {
    fn into(self) -> u32 {
        match self {
            BitmapFormat::Opaque => raw::BitmapFormat_BitmapFormat_Opaque,
            BitmapFormat::BGR => raw::BitmapFormat_BitmapFormat_BGR,
            BitmapFormat::BGR0 => raw::BitmapFormat_BitmapFormat_BGR0,
            BitmapFormat::BGRA => raw::BitmapFormat_BitmapFormat_BGRA,
            BitmapFormat::RGBA => raw::BitmapFormat_BitmapFormat_RGBA,
            BitmapFormat::PNG => raw::BitmapFormat_BitmapFormat_PNG,
            BitmapFormat::JPEG => raw::BitmapFormat_BitmapFormat_JPEG,
        }
    }
}
impl From<u32> for BitmapFormat {
    fn from(value: u32) -> Self {
        match value {
            raw::BitmapFormat_BitmapFormat_BGR => BitmapFormat::BGR,
            raw::BitmapFormat_BitmapFormat_BGR0 => BitmapFormat::BGR0,
            raw::BitmapFormat_BitmapFormat_BGRA => BitmapFormat::BGRA,
            raw::BitmapFormat_BitmapFormat_RGBA => BitmapFormat::RGBA,
            raw::BitmapFormat_BitmapFormat_PNG => BitmapFormat::PNG,
            raw::BitmapFormat_BitmapFormat_JPEG => BitmapFormat::JPEG,
            _ => BitmapFormat::Opaque,
        }
    }
}
impl Display for BitmapFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
