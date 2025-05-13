mod implementation;
mod framebuffer_impl;

use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::{IFramebuffer};
use crate::enums::BitmapFormat;
use crate::framebuffer::framebuffer_impl::IFramebufferImpl;

/// IFramebuffer Interface Reference
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_framebuffer.html](https://www.virtualbox.org/sdkref/interface_i_framebuffer.html)
#[derive(Clone)]
pub struct Framebuffer {
    pub(crate) object: *mut IFramebuffer,
}

impl Framebuffer {
    pub(crate) fn new(object: *mut IFramebuffer) -> Self {
        Self { object }
    }
    pub fn init() -> Self {
        Self { object: IFramebufferImpl::new() }
    }

    fn get_object_impl(&self) -> Option<*mut IFramebufferImpl> {
        if self.object.is_null() {
            log::error!("The `this` pointer is null.");
            return None;
        }
        unsafe {
            let impl_ref = &mut *(self.object as *mut IFramebufferImpl);
            if impl_ref.lpVtbl.is_null() {
                log::error!("Invalid IFramebufferImpl: lpVtbl is null.");
                return None;
            }
            Some(impl_ref)
        }
    }
    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Framebuffer refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Framebuffer. Error: {:?}", err)
            }
        }
    }
}

impl std::fmt::Display for Framebuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("width", self.get_width().unwrap_or(0).to_string());
        map.insert("height", self.get_height().unwrap_or(0).to_string());
        map.insert("bits_per_pixel", self.get_bits_per_pixel().unwrap_or(0).to_string());
        map.insert("bytes_per_line", self.get_bytes_per_line().unwrap_or(0).to_string());
        map.insert("get_pixel_format", self.get_pixel_format().unwrap_or(BitmapFormat::Opaque).to_string());
        map.insert("get_height_reduction", self.get_height_reduction().unwrap_or(0).to_string());
        map.insert("get_win_id", self.get_win_id().unwrap_or(0).to_string());
        map.insert("capabilities", format!("{:?}", self.get_capabilities().unwrap_or(vec![])));

        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for Framebuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
unsafe impl Sync for Framebuffer {}
unsafe impl Send for Framebuffer {}
