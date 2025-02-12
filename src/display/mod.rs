mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use vbox_raw::sys_lib::IDisplay;

/// The Display interface represents the virtual machine's display
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_display.html](https://www.virtualbox.org/sdkref/interface_i_display.html)
#[derive(Debug, Clone)]
pub struct Display {
    object: *mut IDisplay,
    framebuffer_ids: Arc<Mutex<HashMap<&'static str, u32>>>,
}

impl Display {
    pub(crate) fn new(object: *mut IDisplay) -> Self {
        Self {
            object,
            framebuffer_ids: Arc::new(Default::default()),
        }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }

    fn detach_framebuffers(&self) -> Result<(), VboxError> {
        let framebuffer_ids = self
            .framebuffer_ids
            .lock()
            .map_err(|err| VboxError::new(0, "attach_framebuffer", err.to_string(), None))?;
        for (framebuffer_id, screen_id) in framebuffer_ids.iter() {
            self.detach_framebuffer(screen_id.clone(), framebuffer_id)?;
        }
        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        match self.detach_framebuffers() {
            Ok(_) => {}
            Err(err) => {
                error!("Failed detach framebuffers. Error: {:?}", err)
            }
        }

        match self.release() {
            Ok(count) => {
                debug!("Display refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Display. Error: {:?}", err)
            }
        }
    }
}

#[derive(Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u32,
    pub x_origin: i32,
    pub y_origin: i32,
    pub guest_monitor_status: u32,
}

unsafe impl Sync for Display {}
unsafe impl Send for Display {}
