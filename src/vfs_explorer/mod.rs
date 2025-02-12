use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IVFSExplorer;

/// The VFSExplorer interface unifies access to different file system types.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_v_f_s_explorer.html](https://www.virtualbox.org/sdkref/interface_i_v_f_s_explorer.html)
#[derive(Debug)]
pub struct VFSExplorer {
    object: *mut IVFSExplorer,
}

impl VFSExplorer {
    pub(crate) fn new(object: *mut IVFSExplorer) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VFSExplorer {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("release refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VFSExplorer. Error: {:?}", err)
            }
        }
    }
}
