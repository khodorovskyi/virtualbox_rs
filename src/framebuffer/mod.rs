mod framebuffer_impl;
mod implementation;
mod message_handler;

use crate::enums::BitmapFormat;
use crate::framebuffer::framebuffer_impl::{IFramebufferImpl};
use crate::framebuffer::message_handler::handle_message;
use crate::utility::macros::macros::call_function;
use crate::{Display, VboxError};
use log::{debug, error};
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Formatter};
use std::sync::{Mutex, OnceLock};
use tokio::sync::mpsc;
use vbox_raw::sys_lib::{IFramebuffer};
use crate::VboxErrorType::NS_ERROR_NOT_IMPLEMENTED;

/// IFramebuffer Interface Reference
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_framebuffer.html](https://www.virtualbox.org/sdkref/interface_i_framebuffer.html)
static CHANNEL_REGISTRY: OnceLock<Mutex<HashMap<u32, mpsc::Sender<FramebufferEventInternal>>>> =
    OnceLock::new();

pub struct Framebuffer {
    pub(crate) object: *mut IFramebuffer,
    framebuffer_id: String,
    display: Display,
    tx: mpsc::Sender<FramebufferEventInternal>,
}

impl Framebuffer {
    pub fn new(mut display: Display, pixel_format: BitmapFormat, max_fpx: u32) -> Result<Self, VboxError> {

        if pixel_format != BitmapFormat::JPEG && pixel_format != BitmapFormat::BGRA {
            return Err(VboxError{
                error_type: NS_ERROR_NOT_IMPLEMENTED,
                code: 2147500033,
                fn_name: "Framebuffer::new".to_string(),
                msg: "Only JPEG and BGRA are supported at the moment.".to_string(),
            })
        }
        CHANNEL_REGISTRY.get_or_init(|| HashMap::new().into());
        let (tx, rx) = mpsc::channel::<FramebufferEventInternal>(100);
        let tx_clone = tx.clone();
        let channel_id = generate_channel_id();
        let channel_id_copy = channel_id;

        let framebuffer_impl = IFramebufferImpl::new(pixel_format, channel_id_copy);
        let framebuffer_id = display.attach_framebuffer(0, framebuffer_impl)?;

        if let Some(registry) = CHANNEL_REGISTRY.get() {
            let mut registry = registry.lock().unwrap();
            registry.insert(channel_id,  tx_clone);
        }
        handle_message(rx, display.clone(), framebuffer_id.to_string(), max_fpx);
        Ok(Self {
            object: framebuffer_impl,
            framebuffer_id: framebuffer_id.to_string(),
            display,
            tx,
        })
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        match self.display.detach_framebuffer(0, &self.framebuffer_id) {
            Ok(_) => {
                debug!("Framebuffer detached successfully.");
                let _ = self.display.release();
            }
            Err(error) => {
                error!("Error detaching framebuffer: {:?}", error);
            }
        };
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
        map.insert(
            "bits_per_pixel",
            self.get_bits_per_pixel().unwrap_or(0).to_string(),
        );
        map.insert(
            "bytes_per_line",
            self.get_bytes_per_line().unwrap_or(0).to_string(),
        );
        map.insert(
            "get_pixel_format",
            self.get_pixel_format()
                .unwrap_or(BitmapFormat::Opaque)
                .to_string(),
        );
        map.insert(
            "get_height_reduction",
            self.get_height_reduction().unwrap_or(0).to_string(),
        );
        map.insert("get_win_id", self.get_win_id().unwrap_or(0).to_string());
        map.insert(
            "capabilities",
            format!("{:?}", self.get_capabilities().unwrap_or(vec![])),
        );

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

fn generate_channel_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs() as u32
}
#[derive(Debug)]
pub enum FramebufferEventInternal {
    FramebufferImage(FramebufferImage),
    ChangeResolution(u32, u32),
    GetImage(tokio::sync::oneshot::Sender<FramebufferEvent>, bool),
}

#[derive(Debug)]
pub enum FramebufferEvent {
    FramebufferImage(FramebufferImage),
    ChangeResolution(u32, u32),
    None
}
#[derive(Debug)]
pub struct FramebufferImage {
    pub data: Vec<u8>,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub original_width: u32,
    pub original_height: u32,
}

impl Into<FramebufferEvent> for FramebufferEventInternal {
    fn into(self) -> FramebufferEvent {
        match self {
            FramebufferEventInternal::FramebufferImage(image) => {
                FramebufferEvent::FramebufferImage(image)
            }
            FramebufferEventInternal::ChangeResolution(width, height) => {
                FramebufferEvent::ChangeResolution(width, height)
            }
            FramebufferEventInternal::GetImage(_, _) => {
                FramebufferEvent::None
            }
        }
    }
}

unsafe impl Send for Framebuffer {
    
}
unsafe impl Sync for Framebuffer {}
impl From<FramebufferImage> for FramebufferEvent {
    fn from(image: FramebufferImage) -> Self {
        FramebufferEvent::FramebufferImage(image)
    }
}

