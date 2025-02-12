use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_bool;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IMouseCapabilityChangedEvent, IMOUSECAPABILITYCHANGEDEVENT_IID_STR,
};

/// Notification when the mouse capabilities reported by the guest have changed
#[derive(Debug)]
pub struct MouseCapabilityChangedEvent {
    /// If this event is relative, absolute or multi-touch.
    pub supports_absolute: bool,
    /// Supports relative coordinates.
    pub supports_relative: bool,
    /// Supports multi-touch events, touchscreen variant.
    /// <div class="warning">
    ///  This flag only exists for versions 7 and above.
    ///  For compatibility with earlier versions, this field has been retained, but it always returns false
    /// </div>
    pub supports_touch_screen: bool,
    /// Supports multi-touch events, touchpad variant.
    /// <div class="warning">
    ///  This flag only exists for versions 7 and above.
    ///  For compatibility with earlier versions, this field has been retained, but it always returns false
    /// </div>
    pub supports_touch_pad: bool,
    /// If host cursor is needed.
    pub needs_host_cursor: bool,
}

impl MouseCapabilityChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MouseCapabilityChangedEvent(detail),
            Err(err) => {
                error!("MouseCapabilityChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMOUSECAPABILITYCHANGEDEVENT_IID_STR)?;
        let supports_absolute = Self::get_supports_absolute(obj1)?;
        let supports_relative = Self::get_supports_relative(obj1)?;
        let supports_touch_screen = Self::get_supports_touch_screen(obj1)?;
        let supports_touch_pad = Self::get_supports_touch_pad(obj1)?;
        let needs_host_cursor = Self::get_needs_host_cursor(obj1)?;
        Ok(Self {
            supports_absolute,
            supports_relative,
            supports_touch_screen,
            supports_touch_pad,
            needs_host_cursor,
        })
    }

    fn get_supports_absolute(object: *mut IMouseCapabilityChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetSupportsAbsolute)
    }
    fn get_supports_relative(object: *mut IMouseCapabilityChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetSupportsRelative)
    }

    #[cfg(not(is_v_6_1))]
    fn get_supports_touch_screen(
        object: *mut IMouseCapabilityChangedEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetSupportsTouchScreen)
    }
    #[cfg(is_v_6_1)]
    fn get_supports_touch_screen(
        _object: *mut IMouseCapabilityChangedEvent,
    ) -> Result<bool, VboxError> {
        Ok(false)
    }
    #[cfg(not(is_v_6_1))]
    fn get_supports_touch_pad(
        object: *mut IMouseCapabilityChangedEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetSupportsTouchPad)
    }
    #[cfg(is_v_6_1)]
    fn get_supports_touch_pad(
        _object: *mut IMouseCapabilityChangedEvent,
    ) -> Result<bool, VboxError> {
        Ok(false)
    }
    fn get_needs_host_cursor(object: *mut IMouseCapabilityChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetNeedsHostCursor)
    }
}

impl Display for MouseCapabilityChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
