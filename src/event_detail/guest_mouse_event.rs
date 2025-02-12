use crate::enums::MouseButtonState;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IGuestMouseEvent, IGUESTMOUSEEVENT_IID_STR};

/// Notification when guest mouse event happens.
#[derive(Debug)]
pub struct GuestMouseEvent {
    /// If this event is relative, absolute or multi-touch.
    pub mode: u32,
    /// New X position, or X delta.
    pub x: i32,
    /// New Y position, or Y delta.
    pub y: i32,
    /// Z delta.
    pub z: i32,
    /// W delta.
    pub w: i32,
    /// Button state [`MouseButtonState`].
    pub buttons: MouseButtonState,
    /// Current generation of event, incremented on reuse.
    pub generation: u32,
}

impl GuestMouseEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestMouseEvent(detail),
            Err(err) => {
                error!("GuestMouseEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTMOUSEEVENT_IID_STR)?;
        let mode = Self::get_mode(obj1)?;
        let x = Self::get_x(obj1)?;
        let y = Self::get_y(obj1)?;
        let z = Self::get_z(obj1)?;
        let w = Self::get_w(obj1)?;
        let buttons = Self::get_buttons(obj1)?;
        let generation = Self::get_generation(obj1)?;
        Ok(Self {
            mode,
            x,
            y,
            z,
            w,
            buttons,
            generation,
        })
    }

    fn get_mode(object: *mut IGuestMouseEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetMode, u32)
    }
    fn get_x(object: *mut IGuestMouseEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetX, i32)
    }
    fn get_y(object: *mut IGuestMouseEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetY, i32)
    }
    fn get_z(object: *mut IGuestMouseEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetZ, i32)
    }
    fn get_w(object: *mut IGuestMouseEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetW, i32)
    }

    fn get_buttons(object: *mut IGuestMouseEvent) -> Result<MouseButtonState, VboxError> {
        let state = get_function_result_number!(object, GetButtons, i32)?;
        Ok(MouseButtonState::from(state))
    }
    fn get_generation(object: *mut IGuestMouseEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetGeneration, u32)
    }
}

impl Display for GuestMouseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
