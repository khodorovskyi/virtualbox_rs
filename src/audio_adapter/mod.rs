pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IAudioAdapter;

/// Interface for controlling the virtual machine AudioAdapter.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_audio_adapter.html](https://www.virtualbox.org/sdkref/interface_i_audio_adapter.html)
#[derive(Debug)]
pub struct AudioAdapter {
    object: *mut IAudioAdapter,
}

impl AudioAdapter {
    pub(crate) fn new(object: *mut IAudioAdapter) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for AudioAdapter {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("AudioAdapter refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop AudioAdapter. Error: {:?}", err)
            }
        }
    }
}
