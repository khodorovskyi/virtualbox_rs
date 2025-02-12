use crate::enums::ClipboardMode;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IClipboardModeChangedEvent, IEvent, ICLIPBOARDMODECHANGEDEVENT_IID_STR};

/// Notification when the shared clipboard mode changes.
#[derive(Debug)]
pub struct ClipboardModeChangedEvent {
    /// The new clipboard mode [`ClipboardMode`].
    pub clipboard_mode: ClipboardMode,
}

impl ClipboardModeChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ClipboardModeChangedEvent(detail),
            Err(err) => {
                error!("ClipboardModeChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICLIPBOARDMODECHANGEDEVENT_IID_STR)?;
        let clipboard_mode = Self::get_clipboard_mode(obj1)?;
        Ok(Self { clipboard_mode })
    }

    fn get_clipboard_mode(
        new_obj: *mut IClipboardModeChangedEvent,
    ) -> Result<ClipboardMode, VboxError> {
        let clipboard_mode = get_function_result_number!(new_obj, GetClipboardMode, u32)?;
        Ok(ClipboardMode::from(clipboard_mode))
    }
}

impl Display for ClipboardModeChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
