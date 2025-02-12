use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_pointer, get_function_result_str,
};
use crate::{PCIDeviceAttachment, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IHostPCIDevicePlugEvent, IPCIDeviceAttachment, IHOSTPCIDEVICEPLUGEVENT_IID_STR,
};

/// Notification when a USB device is attached to or detached from the virtual USB controller.
#[derive(Debug)]
pub struct HostPCIDevicePlugEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// If device successfully plugged or unplugged.
    pub plugged: bool,
    /// If operation was successful, if false - 'message' attribute may be of interest.
    pub success: bool,
    /// Attachment info for this device.
    pub attachment: PCIDeviceAttachment,
    /// Optional error message.
    pub message: &'static str,
}

impl HostPCIDevicePlugEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::HostPCIDevicePlugEvent(detail),
            Err(err) => {
                error!("HostPCIDevicePlugEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IHOSTPCIDEVICEPLUGEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let plugged = Self::get_plugged(obj1)?;
        let success = Self::get_success(obj1)?;
        let attachment = Self::get_attachment(obj1)?;
        let message = Self::get_message(obj1)?;
        Ok(Self {
            machine_id,
            plugged,
            success,
            attachment,
            message,
        })
    }

    fn get_machine_id(object: *mut IHostPCIDevicePlugEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }
    fn get_plugged(new_obj: *mut IHostPCIDevicePlugEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetPlugged)
    }
    fn get_success(new_obj: *mut IHostPCIDevicePlugEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetSuccess)
    }
    fn get_attachment(
        new_obj: *mut IHostPCIDevicePlugEvent,
    ) -> Result<PCIDeviceAttachment, VboxError> {
        let attachment =
            get_function_result_pointer!(new_obj, GetAttachment, *mut IPCIDeviceAttachment)?;
        Ok(PCIDeviceAttachment::new(attachment))
    }

    fn get_message(object: *mut IHostPCIDevicePlugEvent) -> Result<&'static str, VboxError> {
        let message = get_function_result_str!(object, GetMessage);
        if let Err(ref error) = message {
            if error.is_null() {
                return Ok("");
            }
        }
        message
    }
}

impl Display for HostPCIDevicePlugEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);

        write!(f, "{}", format!("{}", s))
    }
}
