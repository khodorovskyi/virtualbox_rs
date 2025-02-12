#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_pointer;
use crate::GuestDebugControl;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{
    IGuestDebugControl, IGuestDebugControlChangedEvent, IGUESTDEBUGCONTROLCHANGEDEVENT_IID_STR,
};

/// Notification when a property of the guest debug settings changes.
#[derive(Debug)]
pub struct GuestDebugControlChangedEvent {
    /// Guest debug control object that is subject to change.
    pub guest_debug_control: GuestDebugControl,
}

#[cfg(not(is_v_6_1))]
impl GuestDebugControlChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestDebugControlChangedEvent(detail),
            Err(err) => {
                error!("GuestDebugControlChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTDEBUGCONTROLCHANGEDEVENT_IID_STR)?;
        let guest_debug_control = Self::get_guest_debug_control(obj1)?;
        Ok(Self {
            guest_debug_control,
        })
    }

    fn get_guest_debug_control(
        new_obj: *mut IGuestDebugControlChangedEvent,
    ) -> Result<GuestDebugControl, VboxError> {
        let guest_debug_control =
            get_function_result_pointer!(new_obj, GetGuestDebugControl, *mut IGuestDebugControl)?;
        Ok(GuestDebugControl::new(guest_debug_control))
    }
}
#[cfg(is_v_6_1)]
impl GuestDebugControlChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for GuestDebugControlChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
