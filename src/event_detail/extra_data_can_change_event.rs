use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_str;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IExtraDataCanChangeEvent, IEXTRADATACANCHANGEEVENT_IID_STR};

/// Notification when someone tries to change extra data for either the given machine or (if null) global extra data.
#[derive(Debug)]
pub struct ExtraDataCanChangeEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// Extra data key that has changed.
    pub key: &'static str,
    /// Extra data value for the given key.
    pub value: &'static str,
}

impl ExtraDataCanChangeEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ExtraDataCanChangeEvent(detail),
            Err(err) => {
                error!("ExtraDataCanChangeEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IEXTRADATACANCHANGEEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let key = Self::get_key(obj1)?;
        let value = Self::get_value(obj1)?;
        Ok(Self {
            machine_id,
            key,
            value,
        })
    }

    fn get_machine_id(object: *mut IExtraDataCanChangeEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }
    fn get_key(object: *mut IExtraDataCanChangeEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetKey)
    }
    fn get_value(object: *mut IExtraDataCanChangeEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetValue)
    }
}

impl Display for ExtraDataCanChangeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
