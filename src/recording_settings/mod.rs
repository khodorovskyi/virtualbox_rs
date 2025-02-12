use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IRecordingSettings;

/// The [`RecordingSettings`] interface represents recording settings of the virtual machine. More...
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_recording_settings.html](https://www.virtualbox.org/sdkref/interface_i_recording_settings.html)
#[derive(Debug)]
pub struct RecordingSettings {
    object: *mut IRecordingSettings,
}

impl RecordingSettings {
    pub(crate) fn new(object: *mut IRecordingSettings) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for RecordingSettings {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("RecordingSettings refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop RecordingSettings. Error: {:?}", err)
            }
        }
    }
}
