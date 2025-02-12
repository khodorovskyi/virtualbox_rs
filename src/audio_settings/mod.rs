pub mod implementation;
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IAudioSettings;

/// The AudioSettings interface represents the audio settings for a virtual machine. More...
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_audio_settings.html](https://www.virtualbox.org/sdkref/interface_i_audio_settings.html)

pub struct AudioSettings {
    object: *mut IAudioSettings,
}

impl AudioSettings {
    pub(crate) fn new(object: *mut IAudioSettings) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for AudioSettings {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("AudioSettings refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop AudioSettings. Error: {:?}", err)
            }
        }
    }
}
