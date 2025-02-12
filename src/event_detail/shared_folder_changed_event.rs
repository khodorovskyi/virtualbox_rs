use crate::enums::Scope;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, ISharedFolderChangedEvent, ISHAREDFOLDERCHANGEDEVENT_IID_STR};

/// Notification when a shared folder is added or removed.
#[derive(Debug)]
pub struct SharedFolderChangedEvent {
    /// [`Scope`] of the notification.
    pub scope: Scope,
}

impl SharedFolderChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::SharedFolderChangedEvent(detail),
            Err(err) => {
                error!("SharedFolderChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ISHAREDFOLDERCHANGEDEVENT_IID_STR)?;
        let scope = Self::get_scope(obj1)?;
        Ok(Self { scope })
    }

    fn get_scope(new_obj: *mut ISharedFolderChangedEvent) -> Result<Scope, VboxError> {
        let scope = get_function_result_number!(new_obj, GetScope, u32)?;
        Ok(Scope::from(scope))
    }
}

impl Display for SharedFolderChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
