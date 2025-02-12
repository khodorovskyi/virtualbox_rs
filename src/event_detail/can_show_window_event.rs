use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_bool;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{ICanShowWindowEvent, IEvent, ICANSHOWWINDOWEVENT_IID_STR};

/// Notification when a call to IMachine::canShowConsoleWindow is made by a front-end to check if a subsequent call to IMachine::showConsoleWindow can succeed
// TODO add IVetoEvent
#[derive(Debug)]
pub struct CanShowWindowEvent {
    pub midl_does_not_like_empty_interfaces: bool,
}

impl CanShowWindowEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CanShowWindowEvent(detail),
            Err(err) => {
                error!("CanShowWindowEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICANSHOWWINDOWEVENT_IID_STR)?;
        let midl_does_not_like_empty_interfaces =
            Self::get_midl_does_not_like_empty_interfaces(obj1)?;
        Ok(Self {
            midl_does_not_like_empty_interfaces,
        })
    }

    fn get_midl_does_not_like_empty_interfaces(
        new_obj: *mut ICanShowWindowEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetMidlDoesNotLikeEmptyInterfaces)
    }
}

impl Display for CanShowWindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
