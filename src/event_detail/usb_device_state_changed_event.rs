use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_pointer};
use crate::{USBDevice, VboxError, VirtualBoxErrorInfo};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IUSBDevice, IUSBDeviceStateChangedEvent, IVirtualBoxErrorInfo,
    IUSBDEVICESTATECHANGEDEVENT_IID_STR,
};

/// Notification when a USB device is attached to or detached from the virtual USB controller.
#[derive(Debug)]
pub struct USBDeviceStateChangedEvent {
    /// Device that is subject to state change.
    pub device: USBDevice,
    /// true if the device was attached and false otherwise.
    pub attached: bool,
    /// None on success or an error message object on failure.
    pub error: Option<VirtualBoxErrorInfo>,
}

impl USBDeviceStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::USBDeviceStateChangedEvent(detail),
            Err(err) => {
                error!("USBDeviceStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IUSBDEVICESTATECHANGEDEVENT_IID_STR)?;
        let device = Self::get_device(obj1)?;
        let attached = Self::get_attached(obj1)?;
        let error = Self::get_error(obj1)?;
        Ok(Self {
            device,
            attached,
            error,
        })
    }

    fn get_device(new_obj: *mut IUSBDeviceStateChangedEvent) -> Result<USBDevice, VboxError> {
        let device = get_function_result_pointer!(new_obj, GetDevice, *mut IUSBDevice)?;
        Ok(USBDevice::new(device))
    }

    fn get_attached(new_obj: *mut IUSBDeviceStateChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetAttached)
    }
    fn get_error(
        new_obj: *mut IUSBDeviceStateChangedEvent,
    ) -> Result<Option<VirtualBoxErrorInfo>, VboxError> {
        let error = get_function_result_pointer!(new_obj, GetError, *mut IVirtualBoxErrorInfo);
        if let Err(ref error) = error {
            if error.is_null() {
                return Ok(None);
            }
        }
        Ok(Some(VirtualBoxErrorInfo::new(error?)))
    }
}

impl Display for USBDeviceStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        let err = match &self.error {
            None => "".to_string(),
            Some(error) => error.to_string(),
        };
        write!(f, "{}", format!("{} error:  {}", s, err))
    }
}
