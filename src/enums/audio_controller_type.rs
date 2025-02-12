use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Virtual audio controller type.
#[derive(Debug)]
pub enum AudioControllerType {
    AC97,
    SB16,
    HDA,
}

impl From<u32> for AudioControllerType {
    fn from(value: u32) -> Self {
        match value {
            raw::AudioControllerType_AudioControllerType_AC97 => AudioControllerType::AC97,
            raw::AudioControllerType_AudioControllerType_SB16 => AudioControllerType::SB16,
            raw::AudioControllerType_AudioControllerType_HDA => AudioControllerType::HDA,
            _ => {
                error!("Unknown AudioControllerType. Type: {}", value);
                AudioControllerType::AC97
            }
        }
    }
}

impl Into<u32> for AudioControllerType {
    fn into(self) -> u32 {
        match self {
            AudioControllerType::AC97 => raw::AudioControllerType_AudioControllerType_AC97,
            AudioControllerType::SB16 => raw::AudioControllerType_AudioControllerType_SB16,
            AudioControllerType::HDA => raw::AudioControllerType_AudioControllerType_HDA,
        }
    }
}

impl Display for AudioControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
