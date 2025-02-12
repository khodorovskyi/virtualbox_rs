use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IGraphicsAdapter;

mod implementation;
/// The [`GraphicsAdapter`] interface represents the graphics adapter of the virtual machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_graphics_adapter.html](https://www.virtualbox.org/sdkref/interface_i_graphics_adapter.html)
#[derive(Debug)]
pub struct GraphicsAdapter {
    object: *mut IGraphicsAdapter,
}

impl GraphicsAdapter {
    pub(crate) fn new(object: *mut IGraphicsAdapter) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for GraphicsAdapter {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("GraphicsAdapter refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop GraphicsAdapter. Error: {:?}", err)
            }
        }
    }
}
