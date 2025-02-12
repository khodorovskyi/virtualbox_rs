use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::{SerialPort, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, ISerialPort, ISerialPortChangedEvent, ISERIALPORTCHANGEDEVENT_IID_STR,
};

/// Notification when a property of one of the virtual serial ports changes.
#[derive(Debug)]
pub struct SerialPortChangedEvent {
    /// Serial port that is subject to change.
    pub serial_port: SerialPort,
}

impl SerialPortChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::SerialPortChangedEvent(detail),
            Err(err) => {
                error!("SerialPortChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ISERIALPORTCHANGEDEVENT_IID_STR)?;
        let serial_port = Self::get_serial_port(obj1)?;
        Ok(Self { serial_port })
    }

    fn get_serial_port(new_obj: *mut ISerialPortChangedEvent) -> Result<SerialPort, VboxError> {
        let serial_port = get_function_result_pointer!(new_obj, GetSerialPort, *mut ISerialPort)?;
        Ok(SerialPort::new(serial_port))
    }
}

impl Display for SerialPortChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
