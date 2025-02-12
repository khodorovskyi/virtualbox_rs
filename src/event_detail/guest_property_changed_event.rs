use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_bool;
use crate::utility::macros::macros::get_function_result_str;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IGuestPropertyChangedEvent, IGUESTPROPERTYCHANGEDEVENT_IID_STR};

/// Notification when a guest property has changed
#[derive(Debug)]
pub struct GuestPropertyChangedEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// The name of the property that has changed.
    pub name: &'static str,
    /// The new property value.
    pub value: &'static str,
    /// The new property flags.
    pub flags: &'static str,
    /// A flag which indicates that property was deleted.
    /// <div class="warning">
    ///  This flag only exists for versions 7 and above.
    ///  For compatibility with earlier versions, this field has been retained, but it always returns false
    /// </div>
    pub f_was_deleted: bool,
}

impl GuestPropertyChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestPropertyChangedEvent(detail),
            Err(err) => {
                error!("GuestPropertyChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTPROPERTYCHANGEDEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let name = Self::get_name(obj1)?;
        let value = Self::get_value(obj1)?;
        let flags = Self::get_flags(obj1)?;
        let f_was_deleted = Self::get_f_was_deleted(obj1)?;
        Ok(Self {
            machine_id,
            name,
            value,
            flags,
            f_was_deleted,
        })
    }

    fn get_machine_id(object: *mut IGuestPropertyChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }

    fn get_name(object: *mut IGuestPropertyChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetName)
    }
    fn get_value(object: *mut IGuestPropertyChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetValue)
    }
    fn get_flags(object: *mut IGuestPropertyChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetFlags)
    }

    #[cfg(not(is_v_6_1))]
    fn get_f_was_deleted(new_obj: *mut IGuestPropertyChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetFWasDeleted)
    }
    #[cfg(is_v_6_1)]
    fn get_f_was_deleted(_new_obj: *mut IGuestPropertyChangedEvent) -> Result<bool, VboxError> {
        Ok(false)
    }
}

impl Display for GuestPropertyChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
