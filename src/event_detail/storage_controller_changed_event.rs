use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_str;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IStorageControllerChangedEvent, ISTORAGECONTROLLERCHANGEDEVENT_IID_STR,
};

/// Notification when a storage controllers changes.
#[derive(Debug)]
pub struct StorageControllerChangedEvent {
    /// The id of the machine containing the storage controller.
    pub machin_id: &'static str,
    /// The name of the storage controller.
    pub controller_name: &'static str,
}

impl StorageControllerChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::StorageControllerChangedEvent(detail),
            Err(err) => {
                error!("StorageControllerChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ISTORAGECONTROLLERCHANGEDEVENT_IID_STR)?;
        let machin_id = Self::get_machin_id(obj1)?;
        let controller_name = Self::get_controller_name(obj1)?;
        Ok(Self {
            machin_id,
            controller_name,
        })
    }

    fn get_machin_id(
        object: *mut IStorageControllerChangedEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachinId)
    }

    fn get_controller_name(
        object: *mut IStorageControllerChangedEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetControllerName)
    }
}

impl Display for StorageControllerChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
