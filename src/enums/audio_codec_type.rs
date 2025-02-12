#[cfg(doc)]
use crate::AudioAdapter;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// The exact variant of audio codec hardware presented to the guest; see [`AudioAdapter::get_audio_codec`].
#[derive(Debug)]
pub enum AudioCodecType {
    /// null value.
    ///
    /// Never used by the API.
    Null,
    /// SB16; this is the only option for the SB16 device.
    SB16,
    /// A STAC9700 AC'97 codec.
    STAC9700,
    /// An AD1980 AC'97 codec.
    ///
    /// Recommended for Linux guests.
    AD1980,
    /// A STAC9221 HDA codec.
    STAC9221,
}

impl From<u32> for AudioCodecType {
    fn from(value: u32) -> Self {
        match value {
            raw::AudioCodecType_AudioCodecType_Null => AudioCodecType::Null,
            raw::AudioCodecType_AudioCodecType_SB16 => AudioCodecType::SB16,
            raw::AudioCodecType_AudioCodecType_STAC9700 => AudioCodecType::STAC9700,
            raw::AudioCodecType_AudioCodecType_AD1980 => AudioCodecType::AD1980,
            raw::AudioCodecType_AudioCodecType_STAC9221 => AudioCodecType::STAC9221,
            _ => {
                error!("Unknown AudioCodecType. Type: {}", value);
                AudioCodecType::Null
            }
        }
    }
}

impl Into<u32> for AudioCodecType {
    fn into(self) -> u32 {
        match self {
            AudioCodecType::Null => raw::AudioCodecType_AudioCodecType_Null,
            AudioCodecType::SB16 => raw::AudioCodecType_AudioCodecType_SB16,
            AudioCodecType::STAC9700 => raw::AudioCodecType_AudioCodecType_STAC9700,
            AudioCodecType::AD1980 => raw::AudioCodecType_AudioCodecType_AD1980,
            AudioCodecType::STAC9221 => raw::AudioCodecType_AudioCodecType_STAC9221,
        }
    }
}

impl Display for AudioCodecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
