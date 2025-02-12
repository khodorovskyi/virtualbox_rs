use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_str;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IExtraDataChangedEvent, IEXTRADATACHANGEDEVENT_IID_STR};

/// Notification when machine specific or global extra data has changed
#[derive(Debug)]
pub struct ExtraDataChangedEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// Extra data key that has changed.
    pub key: &'static str,
    /// Extra data value for the given key.
    pub value: &'static str,
}

impl ExtraDataChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ExtraDataChangedEvent(detail),
            Err(err) => {
                error!("ExtraDataChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IEXTRADATACHANGEDEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let key = Self::get_key(obj1)?;
        let value = Self::get_value(obj1)?;
        Ok(Self {
            machine_id,
            key,
            value,
        })
    }

    fn get_machine_id(object: *mut IExtraDataChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }
    fn get_key(object: *mut IExtraDataChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetKey)
    }
    fn get_value(object: *mut IExtraDataChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetValue)
    }
}

impl Display for ExtraDataChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
