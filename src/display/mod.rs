mod implementation;

use crate::enums::GuestMonitorStatus;
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use vbox_raw::sys_lib::IDisplay;

/// The Display interface represents the virtual machine's display
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_display.html](https://www.virtualbox.org/sdkref/interface_i_display.html)
#[derive(Clone)]
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

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        match self.get_screen_resolution(0) {
            Ok(resolutio) => {
                map.insert("width", resolutio.width.to_string());
                map.insert("height", resolutio.height.to_string());
                map.insert("bits_per_pixel", resolutio.bits_per_pixel.to_string());
                map.insert("x_origin", resolutio.x_origin.to_string());
                map.insert("y_origin", resolutio.y_origin.to_string());
                map.insert(
                    "guest_monitor_status",
                    resolutio.guest_monitor_status.to_string(),
                );
            }
            Err(_) => {}
        }
        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}
impl Debug for Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
#[derive(Debug)]
pub struct Resolution {
    /// The width of the screen.
    pub width: u32,
    /// The height of the screen.
    pub height: u32,
    /// The number of bits per pixel.
    pub bits_per_pixel: u32,
    /// The X origin of the screen.
    pub x_origin: i32,
    /// The Y origin of the screen.
    pub y_origin: i32,
    /// The status of the guest monitor.
    pub guest_monitor_status: GuestMonitorStatus,
}

#[derive(Debug)]
pub struct VideoModeHint {
    /// The number of the guest output to query.
    pub display: u32,
    /// True if a monitor is connected, False otherwise.
    pub enabled: bool,
    /// True, if the position of the guest screen was specified, False otherwise.
    pub change_origin: bool,
    /// The X origin of the guest screen.
    pub origin_x: i32,
    /// The Y origin of the guest screen.
    pub origin_y: i32,
    /// The width of the monitor preferred mode.
    pub width: u32,
    /// The height of the monitor preferred mode.
    pub height: u32,
    /// The number of bits per pixel of the monitor preferred mode.
    pub bits_per_pixel: u32,
}

unsafe impl Sync for Display {}
unsafe impl Send for Display {}
