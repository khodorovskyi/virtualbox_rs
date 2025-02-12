use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_pointer};
use crate::{MediumAttachment, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IMediumAttachment, IStorageDeviceChangedEvent, ISTORAGEDEVICECHANGEDEVENT_IID_STR,
};

/// Notification when a call to IMachine::showConsoleWindow requests the console window to be activated and brought to foreground on the desktop of the host PC.
#[derive(Debug)]
pub struct StorageDeviceChangedEvent {
    /// Storage device that is subject to change.
    pub storage_device: MediumAttachment,
    /// Flag whether the device was removed or added to the VM.
    pub removed: bool,
    /// Flag whether the guest should be notified about the change.
    pub silent: bool,
}

impl StorageDeviceChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::StorageDeviceChangedEvent(detail),
            Err(err) => {
                error!("StorageDeviceChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ISTORAGEDEVICECHANGEDEVENT_IID_STR)?;
        let storage_device = Self::get_storage_device(obj1)?;
        let removed = Self::get_removed(obj1)?;
        let silent = Self::get_silent(obj1)?;
        Ok(Self {
            storage_device,
            removed,
            silent,
        })
    }

    fn get_storage_device(
        new_obj: *mut IStorageDeviceChangedEvent,
    ) -> Result<MediumAttachment, VboxError> {
        let storage_device =
            get_function_result_pointer!(new_obj, GetStorageDevice, *mut IMediumAttachment)?;
        Ok(MediumAttachment::new(storage_device))
    }
    fn get_removed(new_obj: *mut IStorageDeviceChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetRemoved)
    }
    fn get_silent(new_obj: *mut IStorageDeviceChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetSilent)
    }
}

impl Display for StorageDeviceChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
