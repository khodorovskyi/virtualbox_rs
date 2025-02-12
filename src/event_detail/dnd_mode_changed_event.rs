use crate::enums::DnDMode;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IDnDModeChangedEvent, IEvent, IDNDMODECHANGEDEVENT_IID_STR};

/// Notification when the drag'n drop mode changes
#[derive(Debug)]
pub struct DnDModeChangedEvent {
    /// The new drag'n drop mode. [`DnDMode`].
    pub dnd_mode: DnDMode,
}

impl DnDModeChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::DnDModeChangedEvent(detail),
            Err(err) => {
                error!("DnDModeChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IDNDMODECHANGEDEVENT_IID_STR)?;
        let dnd_mode = Self::get_dnd_mode(obj1)?;
        Ok(Self { dnd_mode })
    }

    fn get_dnd_mode(new_obj: *mut IDnDModeChangedEvent) -> Result<DnDMode, VboxError> {
        let dnd_mode = get_function_result_number!(new_obj, GetDndMode, u32)?;
        Ok(DnDMode::from(dnd_mode))
    }
}

impl Display for DnDModeChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
