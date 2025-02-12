use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IHostAudioDevice;

/// Represents an audio device provided by the host OS.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_host_audio_device.html](https://www.virtualbox.org/sdkref/interface_i_host_audio_device.html)
#[derive(Debug)]
pub struct HostAudioDevice {
    object: *mut IHostAudioDevice,
}

impl HostAudioDevice {
    pub(crate) fn new(object: *mut IHostAudioDevice) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for HostAudioDevice {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("HostAudioDevice refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop HostAudioDevice. Error: {:?}", err)
            }
        }
    }
}
