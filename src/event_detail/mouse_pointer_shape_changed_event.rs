use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use std::slice;
use vbox_raw::sys_lib::{
    IEvent, IMousePointerShapeChangedEvent, IMOUSEPOINTERSHAPECHANGEDEVENT_IID_STR,
};

/// Notification when guest mouse event happens.
#[derive(Debug)]
pub struct MousePointerShapeChangedEvent {
    /// Flag whether the pointer is visible.
    pub visible: bool,
    /// Flag whether the pointer has an alpha channel.
    pub alpha: bool,
    /// The pointer hot spot X coordinate.
    pub x_hot: u32,
    /// The pointer hot spot Y coordinate.
    pub y_hot: u32,
    /// Width of the pointer shape in pixels.
    pub width: u32,
    /// Height of the pointer shape in pixels.
    pub height: u32,
    /// Shape buffer arrays.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://https://www.virtualbox.org/sdkref/interface_i_mouse_pointer_shape_changed_event.html](https://www.virtualbox.org/sdkref/interface_i_mouse_pointer_shape_changed_event.html#aa7201ffe4cc150f4497b1971fcad1f02)
    pub shape: &'static [u8],
}

impl MousePointerShapeChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MousePointerShapeChangedEvent(detail),
            Err(err) => {
                error!("MousePointerShapeChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMOUSEPOINTERSHAPECHANGEDEVENT_IID_STR)?;
        let visible = Self::get_visible(obj1)?;
        let alpha = Self::get_alpha(obj1)?;
        let x_hot = Self::get_x_hot(obj1)?;
        let y_hot = Self::get_y_hot(obj1)?;
        let width = Self::get_width(obj1)?;
        let height = Self::get_height(obj1)?;
        let shape = Self::get_shape(obj1)?;

        Ok(Self {
            visible,
            alpha,
            x_hot,
            y_hot,
            width,
            height,
            shape,
        })
    }

    fn get_visible(object: *mut IMousePointerShapeChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetVisible)
    }
    fn get_alpha(object: *mut IMousePointerShapeChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetAlpha)
    }
    fn get_x_hot(object: *mut IMousePointerShapeChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetXhot, u32)
    }
    fn get_y_hot(object: *mut IMousePointerShapeChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetYhot, u32)
    }
    fn get_width(object: *mut IMousePointerShapeChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetWidth, u32)
    }
    fn get_height(object: *mut IMousePointerShapeChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetHeight, u32)
    }
    fn get_shape(object: *mut IMousePointerShapeChangedEvent) -> Result<&'static [u8], VboxError> {
        let mut shape_size = 0;
        let shape = get_function_result_pointer!(object, GetShape, *mut u8, &mut shape_size)?;
        let shape_slice = unsafe { slice::from_raw_parts(shape, shape_size as usize) };
        Ok(shape_slice)
    }
}

impl Display for MousePointerShapeChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
