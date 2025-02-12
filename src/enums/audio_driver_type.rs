use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Host audio driver type.
#[derive(Debug)]
pub enum AudioDriverType {
    /// Use the default audio driver automatically determined for the host that this VirtualBox instance is running on.
    ///
    /// Useful for VMs which need to run on different host OSes.
    Default,
    /// Null value, also means "dummy audio driver".
    Null,
    /// Open Sound System (Linux / Unix hosts only).
    OSS,
    /// Advanced Linux Sound Architecture (Linux hosts only).
    ALSA,
    /// PulseAudio (Linux hosts only).
    Pulse,
    /// Windows multimedia (Windows hosts only, not supported at the moment).
    WinMM,
    /// DirectSound (Windows hosts only).
    DirectSound,
    /// Windows Audio Session (Windows hosts only).
    WAS,
    /// CoreAudio (Mac hosts only).
    CoreAudio,
    /// Reserved for historical reasons.
    MMPM,
    /// Reserved for historical reasons.
    SolAudio,
}

impl From<u32> for AudioDriverType {
    fn from(value: u32) -> Self {
        match value {
            #[cfg(not(is_v_6_1))]
            raw::AudioDriverType_AudioDriverType_Default => AudioDriverType::Default,
            raw::AudioDriverType_AudioDriverType_Null => AudioDriverType::Null,
            raw::AudioDriverType_AudioDriverType_OSS => AudioDriverType::OSS,
            raw::AudioDriverType_AudioDriverType_ALSA => AudioDriverType::ALSA,
            raw::AudioDriverType_AudioDriverType_Pulse => AudioDriverType::Pulse,
            raw::AudioDriverType_AudioDriverType_WinMM => AudioDriverType::WinMM,
            raw::AudioDriverType_AudioDriverType_DirectSound => AudioDriverType::DirectSound,
            #[cfg(not(is_v_6_1))]
            raw::AudioDriverType_AudioDriverType_WAS => AudioDriverType::WAS,
            raw::AudioDriverType_AudioDriverType_CoreAudio => AudioDriverType::CoreAudio,
            raw::AudioDriverType_AudioDriverType_MMPM => AudioDriverType::MMPM,
            raw::AudioDriverType_AudioDriverType_SolAudio => AudioDriverType::SolAudio,
            _ => {
                error!("Unknown AudioDriverType. Type: {}", value);
                AudioDriverType::Null
            }
        }
    }
}

impl Into<u32> for AudioDriverType {
    fn into(self) -> u32 {
        match self {
            #[cfg(not(is_v_6_1))]
            AudioDriverType::Default => raw::AudioDriverType_AudioDriverType_Default,
            AudioDriverType::Null => raw::AudioDriverType_AudioDriverType_Null,
            AudioDriverType::OSS => raw::AudioDriverType_AudioDriverType_OSS,
            AudioDriverType::ALSA => raw::AudioDriverType_AudioDriverType_ALSA,
            AudioDriverType::Pulse => raw::AudioDriverType_AudioDriverType_Pulse,
            AudioDriverType::WinMM => raw::AudioDriverType_AudioDriverType_WinMM,
            AudioDriverType::DirectSound => raw::AudioDriverType_AudioDriverType_DirectSound,
            // AudioDriverType::WAS => raw::AudioDriverType_AudioDriverType_WAS,
            AudioDriverType::CoreAudio => raw::AudioDriverType_AudioDriverType_CoreAudio,
            AudioDriverType::MMPM => raw::AudioDriverType_AudioDriverType_MMPM,
            AudioDriverType::SolAudio => raw::AudioDriverType_AudioDriverType_SolAudio,
            _ => raw::AudioDriverType_AudioDriverType_Null,
        }
    }
}

impl Display for AudioDriverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
