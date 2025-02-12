use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_bool;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IVBoxSVCAvailabilityChangedEvent, IVBOXSVCAVAILABILITYCHANGEDEVENT_IID_STR,
};

/// Notification when VBoxSVC becomes unavailable (due to a crash or similar unexpected circumstances) or available again
#[derive(Debug)]
pub struct VBoxSVCAvailabilityChangedEvent {
    /// Whether VBoxSVC is available now.
    pub available: bool,
}

impl VBoxSVCAvailabilityChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::VBoxSVCAvailabilityChangedEvent(detail),
            Err(err) => {
                error!("VBoxSVCAvailabilityChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IVBOXSVCAVAILABILITYCHANGEDEVENT_IID_STR)?;
        let available = Self::get_available(obj1)?;
        Ok(Self { available })
    }

    fn get_available(new_obj: *mut IVBoxSVCAvailabilityChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetAvailable)
    }
}

impl Display for VBoxSVCAvailabilityChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
