use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_bool;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IKeyboardLedsChangedEvent, IKEYBOARDLEDSCHANGEDEVENT_IID_STR};

/// Notification when the guest OS executes the KBD_CMD_SET_LEDS command to alter the state of the keyboard LEDs.
#[derive(Debug)]
pub struct KeyboardLedsChangedEvent {
    /// NumLock status.
    pub num_lock: bool,
    /// CapsLock status.
    pub caps_lock: bool,
    /// ScrollLock status.
    pub scroll_lock: bool,
}

impl KeyboardLedsChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::KeyboardLedsChangedEvent(detail),
            Err(err) => {
                error!("KeyboardLedsChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IKEYBOARDLEDSCHANGEDEVENT_IID_STR)?;
        let num_lock = Self::get_num_lock(obj1)?;
        let caps_lock = Self::get_caps_lock(obj1)?;
        let scroll_lock = Self::get_scroll_lock(obj1)?;
        Ok(Self {
            num_lock,
            caps_lock,
            scroll_lock,
        })
    }

    fn get_num_lock(object: *mut IKeyboardLedsChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetNumLock)
    }
    fn get_caps_lock(object: *mut IKeyboardLedsChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetCapsLock)
    }
    fn get_scroll_lock(object: *mut IKeyboardLedsChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetScrollLock)
    }
}

impl Display for KeyboardLedsChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
