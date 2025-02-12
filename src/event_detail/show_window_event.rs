use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IShowWindowEvent, ISHOWWINDOWEVENT_IID_STR};

/// Notification when a call to IMachine::showConsoleWindow requests the console window to be activated and brought to foreground on the desktop of the host PC.
#[derive(Debug)]
pub struct ShowWindowEvent {
    /// Platform-dependent identifier of the top-level VM console window, or zero if this method has performed all actions necessary to implement the show window semantics for the given platform and/or this VirtualBox front-end.
    pub win_id: i64, // TODO add SetWinId
}

impl ShowWindowEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ShowWindowEvent(detail),
            Err(err) => {
                error!("ShowWindowEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ISHOWWINDOWEVENT_IID_STR)?;
        let win_id = Self::get_win_id(obj1)?;
        Ok(Self { win_id })
    }

    fn get_win_id(new_obj: *mut IShowWindowEvent) -> Result<i64, VboxError> {
        get_function_result_number!(new_obj, GetWinId, i64)
    }
}

impl Display for ShowWindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
