use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IRuntimeErrorEvent, IRUNTIMEERROREVENT_IID_STR};

/// Notification when an error happens during the virtual machine execution
#[derive(Debug)]
pub struct RuntimeErrorEvent {
    /// Whether the error is fatal or not.
    pub fatal: bool,
    /// Error identifier.
    pub id: &'static str,
    /// Optional error message.
    pub message: &'static str,
}

impl RuntimeErrorEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::RuntimeErrorEvent(detail),
            Err(err) => {
                error!("RuntimeErrorEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IRUNTIMEERROREVENT_IID_STR)?;
        let fatal = Self::get_fatal(obj1)?;
        let id = Self::get_id(obj1)?;
        let message = Self::get_message(obj1)?;
        Ok(Self { fatal, id, message })
    }

    fn get_fatal(new_obj: *mut IRuntimeErrorEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetFatal)
    }

    fn get_id(object: *mut IRuntimeErrorEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetId)
    }
    fn get_message(object: *mut IRuntimeErrorEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMessage)
    }
}

impl Display for RuntimeErrorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
